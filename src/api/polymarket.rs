use anyhow::{Context, Result};
use async_trait::async_trait;
use log::{debug, error};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::models::Market;

#[derive(Debug, Clone)]
pub struct PolymarketClient {
    client: Client,
    api_key: String,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct PolymarketMarketResponse {
    markets: Vec<PolymarketMarket>,
}

#[derive(Debug, Deserialize)]
struct PolymarketMarket {
    id: String,
    question: String,
    #[serde(rename = "bestBid")]
    best_bid: String,
    #[serde(rename = "bestAsk")]
    best_ask: String,
    volume: String,
    liquidity: String,
    #[serde(rename = "endDate")]
    end_date: String,
}

#[derive(Debug, Serialize)]
struct PlaceOrderRequest {
    market_id: String,
    side: String,
    price: String,
    amount: String,
}

impl PolymarketClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key,
            base_url,
        }
    }

    pub async fn get_markets(&self) -> Result<Vec<Market>> {
        debug!("Fetching Polymarket markets");

        let url = format!("{}/markets", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to fetch markets from Polymarket")?;

        if !response.status().is_success() {
            error!("Polymarket API error: {}", response.status());
            return Err(anyhow::anyhow!("API request failed"));
        }

        let data: PolymarketMarketResponse = response
            .json()
            .await
            .context("Failed to parse Polymarket response")?;

        let markets = data
            .markets
            .into_iter()
            .filter_map(|m| self.parse_market(m).ok())
            .collect();

        Ok(markets)
    }

    fn parse_market(&self, market: PolymarketMarket) -> Result<Market> {
        Ok(Market {
            id: market.id,
            question: market.question,
            platform: crate::models::Platform::Polymarket,
            yes_price: market.best_ask.parse()?,
            no_price: market.best_bid.parse()?,
            volume: market.volume.parse()?,
            liquidity: market.liquidity.parse()?,
            end_time: chrono::DateTime::parse_from_rfc3339(&market.end_date)?
                .with_timezone(&chrono::Utc),
        })
    }

    pub async fn place_order(
        &self,
        market_id: &str,
        side: &str,
        price: Decimal,
        amount: Decimal,
    ) -> Result<String> {
        debug!("Placing order on Polymarket: {} {} @ {}", amount, side, price);

        let url = format!("{}/orders", self.base_url);
        
        let request = PlaceOrderRequest {
            market_id: market_id.to_string(),
            side: side.to_string(),
            price: price.to_string(),
            amount: amount.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to place order"));
        }

        let order_id = response
            .json::<serde_json::Value>()
            .await?
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid response"))?
            .to_string();

        Ok(order_id)
    }
}

#[async_trait]
pub trait MarketDataProvider {
    async fn get_markets(&self) -> Result<Vec<Market>>;
}

#[async_trait]
impl MarketDataProvider for PolymarketClient {
    async fn get_markets(&self) -> Result<Vec<Market>> {
        self.get_markets().await
    }
}
