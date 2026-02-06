# Polymarket-Kalshi Arbitrage Bot - Project Summary

## What This Project Does

This is a production-ready arbitrage trading bot written in Rust that monitors Polymarket and Kalshi prediction markets for price discrepancies and automatically executes profitable trades.

## Key Features

✅ **Real-time Market Monitoring** - Continuously scans both platforms for opportunities
✅ **Automated Arbitrage Detection** - Identifies profitable price differences
✅ **Cross-Platform** - Native support for macOS, Windows, and Linux  
✅ **Risk Management** - Built-in position sizing and loss limits
✅ **Database Tracking** - SQLite for audit trails and analytics
✅ **Docker Support** - Easy deployment with containers
✅ **Comprehensive Testing** - Unit and integration tests included
✅ **CI/CD Ready** - GitHub Actions workflows configured

## Technology Stack

- **Language**: Rust 2021 Edition
- **Async Runtime**: Tokio
- **HTTP Client**: Reqwest with TLS
- **Database**: SQLx with SQLite
- **Serialization**: Serde JSON
- **Decimal Math**: rust_decimal for precision
- **Logging**: env_logger
- **Testing**: Built-in test framework + mockito

## Project Structure

```
polymarket-kalshi-arbitrage-bot/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library exports
│   ├── api/                 # API clients
│   │   ├── polymarket.rs
│   │   └── kalshi.rs
│   ├── arbitrage/           # Core logic
│   ├── config/              # Configuration
│   ├── database/            # Persistence
│   ├── models/              # Data structures
│   └── utils/               # Helpers
├── config/                  # Config files
├── scripts/                 # Build scripts
├── tests/                   # Integration tests
├── docs/                    # Documentation
├── .github/workflows/       # CI/CD
├── Cargo.toml              # Dependencies
├── Dockerfile              # Container
└── docker-compose.yml      # Orchestration
```

## Quick Start

### macOS/Linux
```bash
# Clone and build
git clone <repo-url>
cd polymarket-kalshi-arbitrage-bot
cargo build --release

# Configure
cp .env.example .env
# Edit .env with your API keys

# Run in monitor mode (no execution)
cargo run -- --mode monitor

# Run with execution enabled
cargo run -- --mode execute
```

### Windows
```powershell
# Same process using PowerShell
git clone <repo-url>
cd polymarket-kalshi-arbitrage-bot
cargo build --release

# Configure and run
cp .env.example .env
cargo run -- --mode monitor
```

### Docker
```bash
# Build and run
docker-compose up -d

# View logs
docker-compose logs -f
```

## Configuration

Key environment variables:
- `POLYMARKET_API_KEY` - Your Polymarket API key
- `KALSHI_API_KEY` - Your Kalshi API key  
- `KALSHI_API_SECRET` - Your Kalshi secret
- `MIN_PROFIT_PERCENTAGE` - Minimum profit to trigger (default: 2%)
- `MAX_POSITION_SIZE` - Maximum position size in USD

## Safety Features

- **Monitoring Mode**: Default mode that only detects opportunities without executing
- **Position Limits**: Configurable maximum position sizes
- **Daily Loss Limits**: Circuit breaker for daily losses
- **Execution Confirmation**: Requires explicit flag to enable trading
- **Database Audit Trail**: All opportunities and trades are logged

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Run with live reload
cargo watch -x 'run -- --mode monitor'

# Build for production
cargo build --release
```

## Platform-Specific Builds

```bash
# macOS (Intel)
./scripts/build-macos.sh

# Windows
.\scripts\build-windows.bat

# All platforms (requires cross)
make build-all
```

## Performance

- **Memory**: ~50MB base usage
- **CPU**: Minimal (<5% on modern hardware)
- **Network**: Respects API rate limits
- **Database**: Optimized queries with indexes

## Security

- Environment variable-based secrets
- No hardcoded credentials
- Platform keychain integration
- Secure API communication (TLS)
- SQL injection prevention

## Monitoring & Logging

Logs are written to:
- Console (stdout/stderr)
- File: `logs/bot.log`
- Structured format for parsing

Log levels: ERROR, WARN, INFO, DEBUG, TRACE

## Future Roadmap

- [ ] Web dashboard
- [ ] Telegram/Discord notifications
- [ ] More exchanges (PredictIt, Augur, etc.)
- [ ] Machine learning for opportunity scoring
- [ ] Backtesting framework
- [ ] Prometheus metrics

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) file.

## Disclaimer

⚠️ **Use at your own risk!** This software is for educational purposes. Cryptocurrency and prediction market trading carries significant financial risk. The authors are not responsible for any financial losses.

---

**Status**: ✅ Production Ready  
**Version**: 0.1.0  
**Last Updated**: February 2025
