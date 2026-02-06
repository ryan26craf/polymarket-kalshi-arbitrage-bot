# Architecture Documentation

## Overview

The Polymarket-Kalshi Arbitrage Bot is designed as a modular, event-driven system that monitors prediction markets for arbitrage opportunities.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Main Process                          │
│  ┌────────────┐  ┌────────────┐  ┌────────────────────────┐ │
│  │   Config   │  │  Database  │  │  Arbitrage Engine      │ │
│  └────────────┘  └────────────┘  └────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
   ┌────▼────┐        ┌────▼────┐        ┌────▼────┐
   │Polymarket│        │  Kalshi │        │ Database│
   │  Client  │        │  Client │        │         │
   └─────────┘        └─────────┘        └─────────┘
        │                   │                   │
        └───────────────────┼───────────────────┘
                            │
                    ┌───────▼────────┐
                    │ Market Monitor │
                    └────────────────┘
                            │
                    ┌───────▼────────┐
                    │Opportunity     │
                    │Detection       │
                    └────────────────┘
                            │
                    ┌───────▼────────┐
                    │Execution       │
                    │Engine          │
                    └────────────────┘
```

## Core Components

### 1. Configuration (`src/config/`)
- Loads configuration from TOML files and environment variables
- Manages API credentials and bot parameters
- Supports multiple deployment environments

### 2. API Clients (`src/api/`)
- **Polymarket Client**: Handles Polymarket API interactions
- **Kalshi Client**: Handles Kalshi API interactions
- Implements retry logic and rate limiting
- Provides unified interface via traits

### 3. Arbitrage Engine (`src/arbitrage/`)
- Main orchestration logic
- Market monitoring loop
- Opportunity detection algorithm
- Position sizing and risk management
- Execution coordination

### 4. Database (`src/database/`)
- SQLite for persistence
- Stores opportunities and trades
- Provides audit trail
- Supports analytics

### 5. Models (`src/models/`)
- Core data structures
- Market representation
- Opportunity definition
- Trade tracking

## Data Flow

1. **Market Data Collection**
   - Fetch markets from Polymarket
   - Fetch markets from Kalshi
   - Parse and normalize data

2. **Market Matching**
   - Compare market questions
   - Use similarity algorithms
   - Match equivalent markets

3. **Arbitrage Detection**
   - Calculate price differences
   - Compute profit percentages
   - Filter by minimum threshold
   - Account for fees

4. **Execution** (if enabled)
   - Size positions appropriately
   - Place simultaneous orders
   - Monitor fills
   - Update database

5. **Risk Management**
   - Track open positions
   - Monitor P&L
   - Enforce position limits
   - Circuit breakers

## Concurrency Model

- **tokio** runtime for async I/O
- Non-blocking API calls
- Concurrent market fetching
- Rate limiting via semaphores

## Error Handling

- **anyhow** for error propagation
- **thiserror** for custom errors
- Graceful degradation
- Comprehensive logging

## Security

- API keys via environment variables
- No hardcoded credentials
- Platform-specific key storage (macOS Keychain, Windows Credential Manager)
- SQL injection prevention via parameterized queries

## Testing Strategy

- Unit tests for core logic
- Integration tests for API clients
- Mock servers for external APIs
- Property-based testing for calculations

## Deployment

### Development
```bash
cargo run -- --mode monitor
```

### Production
```bash
cargo build --release
./target/release/polymarket-kalshi-arbitrage-bot --mode execute
```

### Docker
```bash
docker-compose up -d
```

## Performance Considerations

- Connection pooling for database
- HTTP client reuse
- Efficient market matching algorithms
- Minimal allocations in hot paths
- Batch database operations

## Monitoring

- Structured logging
- Metrics collection
- Database audit trail
- External monitoring hooks (future)

## Future Enhancements

- [ ] Web dashboard
- [ ] Prometheus metrics
- [ ] Multiple exchange support
- [ ] Advanced matching algorithms
- [ ] Machine learning for opportunity scoring
- [ ] Telegram/Discord notifications
- [ ] Backtesting framework
