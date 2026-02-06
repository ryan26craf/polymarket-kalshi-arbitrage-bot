use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id: String,
    pub question: String,
    pub platform: Platform,
    pub yes_price: Decimal,
    pub no_price: Decimal,
    pub volume: Decimal,
    pub liquidity: Decimal,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    Polymarket,
    Kalshi,
}

impl Platform {
    pub fn as_str(&self) -> &str {
        match self {
            Platform::Polymarket => "polymarket",
            Platform::Kalshi => "kalshi",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub id: Option<i64>,
    pub polymarket_market_id: String,
    pub kalshi_market_id: String,
    pub buy_platform: Platform,
    pub sell_platform: Platform,
    pub buy_price: Decimal,
    pub sell_price: Decimal,
    pub profit_percentage: Decimal,
    pub estimated_profit: Decimal,
    pub position_size: Decimal,
    pub detected_at: DateTime<Utc>,
    pub executed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Option<i64>,
    pub opportunity_id: i64,
    pub platform: Platform,
    pub market_id: String,
    pub side: TradeSide,
    pub price: Decimal,
    pub amount: Decimal,
    pub status: TradeStatus,
    pub created_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeStatus {
    Pending,
    Executed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub platform: Platform,
    pub market_id: String,
    pub amount: Decimal,
    pub entry_price: Decimal,
    pub current_value: Decimal,
}
