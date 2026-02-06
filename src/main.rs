use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use polymarket_kalshi_arbitrage_bot::{
    arbitrage::ArbitrageEngine,
    config::Config,
    database::Database,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Operating mode: monitor or execute
    #[arg(short, long, default_value = "monitor")]
    mode: String,

    /// Minimum profit percentage
    #[arg(short = 'p', long)]
    min_profit: Option<f64>,

    /// Configuration file path
    #[arg(short, long, default_value = "config/default.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting Polymarket-Kalshi Arbitrage Bot");

    // Parse command line arguments
    let args = Args::parse();

    // Load configuration
    let mut config = Config::load(&args.config)?;
    
    // Override config with CLI arguments
    if let Some(min_profit) = args.min_profit {
        config.bot.min_profit_percentage = min_profit;
    }

    let execution_enabled = args.mode == "execute";
    
    info!("Mode: {}", args.mode);
    info!("Execution: {}", if execution_enabled { "ENABLED" } else { "DISABLED" });
    info!("Min profit threshold: {}%", config.bot.min_profit_percentage);

    // Initialize database
    let database = Database::new(&config.database.url).await?;
    database.run_migrations().await?;

    // Create and run arbitrage engine
    let mut engine = ArbitrageEngine::new(config, database, execution_enabled).await?;

    // Handle shutdown gracefully
    let ctrl_c = tokio::signal::ctrl_c();
    
    tokio::select! {
        result = engine.run() => {
            if let Err(e) = result {
                error!("Engine error: {}", e);
                return Err(e);
            }
        }
        _ = ctrl_c => {
            info!("Received shutdown signal");
            engine.shutdown().await?;
        }
    }

    info!("Bot shutdown complete");
    Ok(())
}
