use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Row};

use crate::models::ArbitrageOpportunity;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS opportunities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                polymarket_market_id TEXT NOT NULL,
                kalshi_market_id TEXT NOT NULL,
                buy_platform TEXT NOT NULL,
                sell_platform TEXT NOT NULL,
                buy_price TEXT NOT NULL,
                sell_price TEXT NOT NULL,
                profit_percentage TEXT NOT NULL,
                estimated_profit TEXT NOT NULL,
                position_size TEXT NOT NULL,
                detected_at TEXT NOT NULL,
                executed INTEGER NOT NULL DEFAULT 0
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS trades (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                opportunity_id INTEGER NOT NULL,
                platform TEXT NOT NULL,
                market_id TEXT NOT NULL,
                side TEXT NOT NULL,
                price TEXT NOT NULL,
                amount TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                executed_at TEXT,
                FOREIGN KEY (opportunity_id) REFERENCES opportunities(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_opportunities_detected 
            ON opportunities(detected_at DESC)
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn save_opportunity(&self, opportunity: &ArbitrageOpportunity) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO opportunities (
                polymarket_market_id,
                kalshi_market_id,
                buy_platform,
                sell_platform,
                buy_price,
                sell_price,
                profit_percentage,
                estimated_profit,
                position_size,
                detected_at,
                executed
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&opportunity.polymarket_market_id)
        .bind(&opportunity.kalshi_market_id)
        .bind(opportunity.buy_platform.as_str())
        .bind(opportunity.sell_platform.as_str())
        .bind(opportunity.buy_price.to_string())
        .bind(opportunity.sell_price.to_string())
        .bind(opportunity.profit_percentage.to_string())
        .bind(opportunity.estimated_profit.to_string())
        .bind(opportunity.position_size.to_string())
        .bind(opportunity.detected_at.to_rfc3339())
        .bind(if opportunity.executed { 1 } else { 0 })
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_recent_opportunities(&self, limit: i64) -> Result<Vec<ArbitrageOpportunity>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM opportunities 
            ORDER BY detected_at DESC 
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut opportunities = Vec::new();
        for row in rows {
            opportunities.push(ArbitrageOpportunity {
                id: Some(row.get("id")),
                polymarket_market_id: row.get("polymarket_market_id"),
                kalshi_market_id: row.get("kalshi_market_id"),
                buy_platform: match row.get::<String, _>("buy_platform").as_str() {
                    "polymarket" => crate::models::Platform::Polymarket,
                    _ => crate::models::Platform::Kalshi,
                },
                sell_platform: match row.get::<String, _>("sell_platform").as_str() {
                    "polymarket" => crate::models::Platform::Polymarket,
                    _ => crate::models::Platform::Kalshi,
                },
                buy_price: row.get::<String, _>("buy_price").parse()?,
                sell_price: row.get::<String, _>("sell_price").parse()?,
                profit_percentage: row.get::<String, _>("profit_percentage").parse()?,
                estimated_profit: row.get::<String, _>("estimated_profit").parse()?,
                position_size: row.get::<String, _>("position_size").parse()?,
                detected_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("detected_at"))?
                    .with_timezone(&chrono::Utc),
                executed: row.get::<i32, _>("executed") == 1,
            });
        }

        Ok(opportunities)
    }

    pub async fn mark_opportunity_executed(&self, opportunity_id: i64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE opportunities 
            SET executed = 1 
            WHERE id = ?
            "#,
        )
        .bind(opportunity_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
