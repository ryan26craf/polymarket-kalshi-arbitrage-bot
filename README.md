# Polymarket-Kalshi Arbitrage Bot
![ezgif-36d53c2d9ab1a0f9](https://github.com/user-attachments/assets/360cba28-9dc3-4cb5-bf4e-171d862f90cb)

A high-performance arbitrage bot written in Rust that identifies and executes arbitrage opportunities between Polymarket and Kalshi prediction markets.

## Features

- 🚀 Real-time market monitoring
- 💰 Automatic arbitrage opportunity detection
- 🔄 Cross-platform support (macOS & Windows)
- 📊 SQLite database for tracking opportunities
- 🔐 Secure API key management
- 📈 Configurable profit thresholds
- 🎯 Risk management with position sizing

## Prerequisites

- Rust 1.70 or higher
- API keys for Polymarket and Kalshi
- SQLite3

## Installation

### macOS

```
git clone https://github.com/ryan26craf/polymarket-kalshi-arbitrage-bot && cd polymarket-kalshi-arbitrage-bot && bash install.sh
```


## Configuration

1. Copy the example environment file:
```bash
cp .env.example .env
```

2. Edit `.env` and add your API credentials:
```
POLYMARKET_API_KEY=your_polymarket_key
KALSHI_API_KEY=your_kalshi_key
KALSHI_API_SECRET=your_kalshi_secret
MIN_PROFIT_PERCENTAGE=2.0
MAX_POSITION_SIZE=1000
```

3. Edit `config/default.toml` for additional settings

## Usage

```bash
# Run in monitoring mode
cargo run -- --mode monitor

# Run with execution enabled
cargo run -- --mode execute

# Run with specific profit threshold
cargo run -- --min-profit 3.5

# Show all options
cargo run -- --help
```

## Project Structure

```
polymarket-kalshi-arbitrage-bot/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── config/              # Configuration management
│   ├── api/                 # API clients
│   ├── arbitrage/           # Arbitrage logic
│   ├── database/            # Database operations
│   └── utils/               # Utilities
├── config/                  # Configuration files
├── tests/                   # Integration tests
├── docs/                    # Documentation
└── scripts/                 # Build and deployment scripts
```

## Safety & Disclaimer

⚠️ **This bot is for educational purposes only.** Trading cryptocurrencies and prediction markets carries significant risk. Use at your own risk.

- Always test with small amounts first
- Understand the markets you're trading
- Never invest more than you can afford to lose
- Be aware of API rate limits
- Monitor your positions actively

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

MIT License - see [LICENSE](LICENSE) for details

## Acknowledgments

- Polymarket API documentation
- Kalshi API documentation
- Rust async ecosystem
