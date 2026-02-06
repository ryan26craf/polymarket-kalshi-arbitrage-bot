# Quick Start Guide - Polymarket-Kalshi Arbitrage Bot

## Prerequisites Checklist

- [ ] Rust installed (1.70+)
- [ ] Git installed
- [ ] API keys from Polymarket
- [ ] API keys from Kalshi
- [ ] SQLite3 (comes with most systems)

## Installation Steps

### 1. Get the Code
```bash
# Navigate to the project directory
cd polymarket-kalshi-arbitrage-bot
```

### 2. Install Dependencies
```bash
# Rust will automatically download and compile dependencies
cargo build
```

### 3. Configure Environment
```bash
# Copy example environment file
cp .env.example .env

# Edit with your favorite editor
nano .env  # or vim, code, etc.
```

Add your credentials:
```
POLYMARKET_API_KEY=your_actual_key_here
KALSHI_API_KEY=your_actual_key_here
KALSHI_API_SECRET=your_actual_secret_here
```

### 4. Test Configuration
```bash
# Run in monitor mode (safe, no trading)
cargo run -- --mode monitor

# You should see logs like:
# [INFO] Starting Polymarket-Kalshi Arbitrage Bot
# [INFO] Mode: monitor
# [INFO] Fetched X Polymarket markets and Y Kalshi markets
```

### 5. Enable Trading (Optional)
```bash
# ONLY when you're ready for real trades!
cargo run -- --mode execute
```

## Common Commands

```bash
# Monitor only (safe)
cargo run -- --mode monitor

# Execute trades (real money!)
cargo run -- --mode execute

# Custom profit threshold
cargo run -- --mode monitor --min-profit 3.5

# View help
cargo run -- --help

# Run tests
cargo test

# Build release version
cargo build --release
```

## File Locations

- **Logs**: `logs/bot.log`
- **Database**: `arbitrage.db`
- **Config**: `config/default.toml`
- **Environment**: `.env`

## Verification Steps

1. **Check API Connection**
   ```bash
   cargo run -- --mode monitor
   # Look for "Fetched X markets" messages
   ```

2. **Verify Database**
   ```bash
   sqlite3 arbitrage.db "SELECT COUNT(*) FROM opportunities;"
   ```

3. **Check Logs**
   ```bash
   tail -f logs/bot.log
   ```

## Troubleshooting

### Error: "Failed to connect to API"
- Check your API keys in `.env`
- Verify internet connection
- Check API key permissions

### Error: "Database error"
- Ensure write permissions in project directory
- Delete `arbitrage.db` and restart

### Error: "Cargo build failed"
- Update Rust: `rustup update`
- Check Rust version: `rustc --version`

### No opportunities found
- This is normal! Real arbitrage opportunities are rare
- Lower `MIN_PROFIT_PERCENTAGE` in `.env` to see more
- Check that both APIs are working

## Safety Reminders

⚠️ **Before enabling execution mode:**
1. Test thoroughly in monitor mode
2. Start with small `MAX_POSITION_SIZE`
3. Set conservative `MIN_PROFIT_PERCENTAGE`
4. Monitor the first few hours closely
5. Understand the risks!

## Next Steps

1. Read [README.md](README.md) for full documentation
2. Review [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) to understand the system
3. Check [docs/API.md](docs/API.md) for API details
4. Join the community (if applicable)

## Getting Help

- Check the logs first
- Review documentation in `docs/`
- Open an issue on GitHub
- Read the source code (it's well-commented!)

---

Happy arbitraging! 🚀
