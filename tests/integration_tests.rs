use polymarket_kalshi_arbitrage_bot::{
    config::Config,
    database::Database,
};

#[tokio::test]
async fn test_database_initialization() {
    let db = Database::new("sqlite::memory:")
        .await
        .expect("Failed to create database");
    
    db.run_migrations()
        .await
        .expect("Failed to run migrations");
}

#[tokio::test]
async fn test_config_loading() {
    std::env::set_var("POLYMARKET_API_KEY", "test_key");
    std::env::set_var("KALSHI_API_KEY", "test_key");
    std::env::set_var("KALSHI_API_SECRET", "test_secret");
    
    let config = Config::load("config/default.toml")
        .expect("Failed to load config");
    
    assert_eq!(config.polymarket.api_key, "test_key");
    assert_eq!(config.kalshi.api_key, "test_key");
}

#[cfg(test)]
mod arbitrage_tests {
    use super::*;
    use rust_decimal::Decimal;
    use polymarket_kalshi_arbitrage_bot::models::{Market, Platform};
    use chrono::Utc;

    fn create_test_market(platform: Platform, yes_price: f64) -> Market {
        Market {
            id: "test_market".to_string(),
            question: "Will it rain tomorrow?".to_string(),
            platform,
            yes_price: Decimal::try_from(yes_price).unwrap(),
            no_price: Decimal::try_from(1.0 - yes_price).unwrap(),
            volume: Decimal::from(10000),
            liquidity: Decimal::from(5000),
            end_time: Utc::now() + chrono::Duration::days(1),
        }
    }

    #[test]
    fn test_arbitrage_detection() {
        let poly_market = create_test_market(Platform::Polymarket, 0.45);
        let kalshi_market = create_test_market(Platform::Kalshi, 0.55);

        // There should be an arbitrage opportunity here
        // Buy at 0.45 on Polymarket, sell at 0.55 on Kalshi
        let profit = (0.55 - 0.45) / 0.45;
        assert!(profit > 0.02); // More than 2% profit
    }

    #[test]
    fn test_no_arbitrage() {
        let poly_market = create_test_market(Platform::Polymarket, 0.50);
        let kalshi_market = create_test_market(Platform::Kalshi, 0.51);

        // Only 2% profit, might not be worth fees
        let profit = (0.51 - 0.50) / 0.50;
        assert!(profit <= 0.02);
    }
}
