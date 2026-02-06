use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub polymarket: PolymarketConfig,
    pub kalshi: KalshiConfig,
    pub bot: BotConfig,
    pub database: DatabaseConfig,
    pub risk: RiskConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PolymarketConfig {
    pub api_key: String,
    pub private_key: String,
    pub wallet_address: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KalshiConfig {
    pub api_key: String,
    pub api_secret: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub min_profit_percentage: f64,
    pub max_position_size: f64,
    pub check_interval_seconds: u64,
    pub enable_execution: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiskConfig {
    pub max_daily_loss: f64,
    pub max_open_positions: usize,
    pub position_size_percentage: f64,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;
        
        // Override with environment variables
        config.load_from_env();
        
        Ok(config)
    }

    fn load_from_env(&mut self) {
        if let Ok(val) = std::env::var("POLYMARKET_API_KEY") {
            self.polymarket.api_key = val;
        }
        if let Ok(val) = std::env::var("POLYMARKET_PRIVATE_KEY") {
            self.polymarket.private_key = val;
        }
        if let Ok(val) = std::env::var("POLYMARKET_WALLET_ADDRESS") {
            self.polymarket.wallet_address = val;
        }
        if let Ok(val) = std::env::var("KALSHI_API_KEY") {
            self.kalshi.api_key = val;
        }
        if let Ok(val) = std::env::var("KALSHI_API_SECRET") {
            self.kalshi.api_secret = val;
        }
        if let Ok(val) = std::env::var("MIN_PROFIT_PERCENTAGE") {
            if let Ok(num) = val.parse() {
                self.bot.min_profit_percentage = num;
            }
        }
        if let Ok(val) = std::env::var("DATABASE_URL") {
            self.database.url = val;
        }
    }
}
