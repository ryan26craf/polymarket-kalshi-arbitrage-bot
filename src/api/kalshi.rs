use anyhow::{Context, Result};
use async_trait::async_trait;
use log::{debug, error};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::models::Market;

#[derive(Debug, Clone)]
pub struct KalshiClient {
    client: Client,
    api_key: String,
    api_secret: String,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct KalshiMarketsResponse {
    markets: Vec<KalshiMarket>,
}

#[derive(Debug, Deserialize)]
struct KalshiMarket {
    ticker: String,
    title: String,
    yes_bid: f64,
    yes_ask: f64,
    volume: f64,
    open_interest: f64,
    close_time: String,
}

#[derive(Debug, Serialize)]
struct CreateOrderRequest {
    ticker: String,
    action: String,
    side: String,
    count: i32,
    #[serde(rename = "type")]
    order_type: String,
    yes_price: Option<i32>,
}

impl KalshiClient {
    pub fn new(api_key: String, api_secret: String, base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key,
            api_secret,
            base_url,
        }
    }

    pub async fn get_markets(&self) -> Result<Vec<Market>> {
        debug!("Fetching Kalshi markets");

        let url = format!("{}/trade-api/v2/markets", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to fetch markets from Kalshi")?;

        if !response.status().is_success() {
            error!("Kalshi API error: {}", response.status());
            return Err(anyhow::anyhow!("API request failed"));
        }

        let data: KalshiMarketsResponse = response
            .json()
            .await
            .context("Failed to parse Kalshi response")?;

        let markets = data
            .markets
            .into_iter()
            .filter_map(|m| self.parse_market(m).ok())
            .collect();

        Ok(markets)
    }

    fn parse_market(&self, market: KalshiMarket) -> Result<Market> {
        let yes_price = Decimal::try_from(market.yes_ask / 100.0)?;
        let no_price = Decimal::try_from((100.0 - market.yes_bid) / 100.0)?;

        Ok(Market {
            id: market.ticker,
            question: market.title,
            platform: crate::models::Platform::Kalshi,
            yes_price,
            no_price,
            volume: Decimal::try_from(market.volume)?,
            liquidity: Decimal::try_from(market.open_interest)?,
            end_time: chrono::DateTime::parse_from_rfc3339(&market.close_time)?
                .with_timezone(&chrono::Utc),
        })
    }

    pub async fn place_order(
        &self,
        ticker: &str,
        side: &str,
        price_cents: i32,
        count: i32,
    ) -> Result<String> {
        debug!("Placing order on Kalshi: {} contracts {} @ ${}", count, side, price_cents);

        let url = format!("{}/trade-api/v2/portfolio/orders", self.base_url);
        
        let request = CreateOrderRequest {
            ticker: ticker.to_string(),
            action: "buy".to_string(),
            side: side.to_string(),
            count,
            order_type: "limit".to_string(),
            yes_price: Some(price_cents),
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
            .get("order")
            .and_then(|o| o.get("order_id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid response"))?
            .to_string();

        Ok(order_id)
    }

    fn generate_signature(&self, path: &str, body: &str) -> String {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        
        mac.update(format!("{}{}", path, body).as_bytes());
        
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }
}

#[async_trait]
impl super::polymarket::MarketDataProvider for KalshiClient {
    async fn get_markets(&self) -> Result<Vec<Market>> {
        self.get_markets().await
    }
}
