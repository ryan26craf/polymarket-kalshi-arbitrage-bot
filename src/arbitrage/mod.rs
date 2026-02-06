use anyhow::Result;
use log::{info, warn};
use rust_decimal::Decimal;
use std::collections::HashMap;
use tokio::time::{interval, Duration};

use crate::{
    api::{KalshiClient, PolymarketClient},
    config::Config,
    database::Database,
    models::{ArbitrageOpportunity, Market, Platform},
};

pub struct ArbitrageEngine {
    polymarket: PolymarketClient,
    kalshi: KalshiClient,
    database: Database,
    config: Config,
    execution_enabled: bool,
    running: bool,
}

impl ArbitrageEngine {
    pub async fn new(config: Config, database: Database, execution_enabled: bool) -> Result<Self> {
        let polymarket = PolymarketClient::new(
            config.polymarket.api_key.clone(),
            config.polymarket.base_url.clone(),
        );

        let kalshi = KalshiClient::new(
            config.kalshi.api_key.clone(),
            config.kalshi.api_secret.clone(),
            config.kalshi.base_url.clone(),
        );

        Ok(Self {
            polymarket,
            kalshi,
            database,
            config,
            execution_enabled,
            running: false,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.running = true;
        let mut check_interval = interval(Duration::from_secs(
            self.config.bot.check_interval_seconds,
        ));

        info!("Arbitrage engine started");

        while self.running {
            check_interval.tick().await;

            if let Err(e) = self.check_opportunities().await {
                warn!("Error checking opportunities: {}", e);
            }
        }

        Ok(())
    }

    async fn check_opportunities(&self) -> Result<()> {
        // Fetch markets from both platforms
        let polymarket_markets = self.polymarket.get_markets().await?;
        let kalshi_markets = self.kalshi.get_markets().await?;

        info!(
            "Fetched {} Polymarket markets and {} Kalshi markets",
            polymarket_markets.len(),
            kalshi_markets.len()
        );

        // Find matching markets
        let matched_markets = self.match_markets(&polymarket_markets, &kalshi_markets);

        // Identify arbitrage opportunities
        for (poly_market, kalshi_market) in matched_markets {
            if let Some(opportunity) = self.calculate_arbitrage(&poly_market, &kalshi_market) {
                info!(
                    "Found opportunity: {}% profit - {} vs {}",
                    opportunity.profit_percentage,
                    poly_market.question,
                    kalshi_market.question
                );

                // Save to database
                self.database.save_opportunity(&opportunity).await?;

                // Execute if enabled
                if self.execution_enabled {
                    self.execute_opportunity(&opportunity).await?;
                }
            }
        }

        Ok(())
    }

    fn match_markets<'a>(
        &self,
        poly_markets: &'a [Market],
        kalshi_markets: &'a [Market],
    ) -> Vec<(&'a Market, &'a Market)> {
        let mut matches = Vec::new();

        for poly_market in poly_markets {
            for kalshi_market in kalshi_markets {
                if self.markets_match(poly_market, kalshi_market) {
                    matches.push((poly_market, kalshi_market));
                }
            }
        }

        matches
    }

    fn markets_match(&self, market1: &Market, market2: &Market) -> bool {
        // Simple matching based on question similarity
        // In production, use more sophisticated matching
        let q1 = market1.question.to_lowercase();
        let q2 = market2.question.to_lowercase();

        let similarity = self.calculate_similarity(&q1, &q2);
        similarity > 0.7
    }

    fn calculate_similarity(&self, s1: &str, s2: &str) -> f64 {
        // Simple word overlap metric
        let words1: std::collections::HashSet<_> = s1.split_whitespace().collect();
        let words2: std::collections::HashSet<_> = s2.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    fn calculate_arbitrage(
        &self,
        poly_market: &Market,
        kalshi_market: &Market,
    ) -> Option<ArbitrageOpportunity> {
        // Check both directions of arbitrage

        // Direction 1: Buy on Polymarket, Sell on Kalshi
        let profit1 = self.calculate_profit(
            poly_market.yes_price,
            kalshi_market.yes_price,
            Platform::Polymarket,
            Platform::Kalshi,
        );

        // Direction 2: Buy on Kalshi, Sell on Polymarket
        let profit2 = self.calculate_profit(
            kalshi_market.yes_price,
            poly_market.yes_price,
            Platform::Kalshi,
            Platform::Polymarket,
        );

        let (profit_percentage, buy_platform, sell_platform, buy_price, sell_price) =
            if profit1 > profit2 {
                (
                    profit1,
                    Platform::Polymarket,
                    Platform::Kalshi,
                    poly_market.yes_price,
                    kalshi_market.yes_price,
                )
            } else {
                (
                    profit2,
                    Platform::Kalshi,
                    Platform::Polymarket,
                    kalshi_market.yes_price,
                    poly_market.yes_price,
                )
            };

        let min_profit = Decimal::try_from(self.config.bot.min_profit_percentage / 100.0).ok()?;

        if profit_percentage > min_profit {
            let position_size = Decimal::try_from(self.config.bot.max_position_size).ok()?;
            let estimated_profit = position_size * profit_percentage;

            Some(ArbitrageOpportunity {
                id: None,
                polymarket_market_id: poly_market.id.clone(),
                kalshi_market_id: kalshi_market.id.clone(),
                buy_platform,
                sell_platform,
                buy_price,
                sell_price,
                profit_percentage,
                estimated_profit,
                position_size,
                detected_at: chrono::Utc::now(),
                executed: false,
            })
        } else {
            None
        }
    }

    fn calculate_profit(
        &self,
        buy_price: Decimal,
        sell_price: Decimal,
        _buy_platform: Platform,
        _sell_platform: Platform,
    ) -> Decimal {
        if sell_price > buy_price {
            (sell_price - buy_price) / buy_price
        } else {
            Decimal::ZERO
        }
    }

    async fn execute_opportunity(&self, opportunity: &ArbitrageOpportunity) -> Result<()> {
        info!("Executing arbitrage opportunity: {:#?}", opportunity);

        // This is a simplified execution flow
        // In production, add proper error handling, position tracking, etc.

        match opportunity.buy_platform {
            Platform::Polymarket => {
                self.polymarket
                    .place_order(
                        &opportunity.polymarket_market_id,
                        "buy",
                        opportunity.buy_price,
                        opportunity.position_size,
                    )
                    .await?;
            }
            Platform::Kalshi => {
                let price_cents = (opportunity.buy_price * Decimal::from(100))
                    .to_string()
                    .parse()?;
                self.kalshi
                    .place_order(
                        &opportunity.kalshi_market_id,
                        "yes",
                        price_cents,
                        opportunity.position_size.to_string().parse()?,
                    )
                    .await?;
            }
        }

        match opportunity.sell_platform {
            Platform::Polymarket => {
                self.polymarket
                    .place_order(
                        &opportunity.polymarket_market_id,
                        "sell",
                        opportunity.sell_price,
                        opportunity.position_size,
                    )
                    .await?;
            }
            Platform::Kalshi => {
                let price_cents = (opportunity.sell_price * Decimal::from(100))
                    .to_string()
                    .parse()?;
                self.kalshi
                    .place_order(
                        &opportunity.kalshi_market_id,
                        "no",
                        price_cents,
                        opportunity.position_size.to_string().parse()?,
                    )
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down arbitrage engine");
        self.running = false;
        Ok(())
    }
}
