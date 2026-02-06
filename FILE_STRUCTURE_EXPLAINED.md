# Complete File Structure

## Root Directory Files

### Configuration Files
- **Cargo.toml** - Rust project manifest, dependencies, and build config
- **.env.example** - Template for environment variables (copy to .env)
- **config/default.toml** - Default configuration for the bot
- **rustfmt.toml** - Code formatting rules
- **.clippy.toml** - Linter configuration
- **deny.toml** - Security and license policy for dependencies

### Build & Deployment
- **Dockerfile** - Container definition for Docker deployment
- **docker-compose.yml** - Multi-container orchestration
- **Makefile** - Build automation and common tasks

### Documentation
- **README.md** - Main project documentation
- **PROJECT_SUMMARY.md** - Quick project overview
- **QUICKSTART.md** - Getting started guide
- **CHANGELOG.md** - Version history and changes
- **CONTRIBUTING.md** - Contribution guidelines
- **LICENSE** - MIT license

### Source Control
- **.gitignore** - Git ignore patterns
- **.github/workflows/ci.yml** - GitHub Actions CI pipeline
- **.github/workflows/release.yml** - Automated release builds

### IDE Configuration
- **.vscode/settings.json** - VS Code workspace settings
- **.vscode/launch.json** - Debug configurations

## Source Code (`src/`)

### Core Application
```
src/
├── main.rs              # Application entry point with CLI
├── lib.rs               # Library root and public exports
```

### API Integration (`src/api/`)
```
src/api/
├── mod.rs               # Module exports
├── polymarket.rs        # Polymarket API client
└── kalshi.rs            # Kalshi API client
```

**Key Components:**
- HTTP client setup
- Authentication handling
- Market data fetching
- Order placement
- Rate limiting
- Error handling

### Arbitrage Engine (`src/arbitrage/`)
```
src/arbitrage/
└── mod.rs               # Main arbitrage detection and execution logic
```

**Responsibilities:**
- Market monitoring loop
- Market matching algorithm
- Profit calculation
- Opportunity detection
- Trade execution coordination
- Risk management

### Configuration (`src/config/`)
```
src/config/
└── mod.rs               # Configuration loading and management
```

**Features:**
- TOML file parsing
- Environment variable overrides
- Multi-environment support
- Validation

### Database (`src/database/`)
```
src/database/
└── mod.rs               # SQLite database operations
```

**Operations:**
- Schema migrations
- Opportunity persistence
- Trade tracking
- Analytics queries
- Audit trail

### Data Models (`src/models/`)
```
src/models/
└── mod.rs               # Core data structures
```

**Models:**
- Market (price, volume, metadata)
- ArbitrageOpportunity
- Trade
- Position
- Platform enum

### Utilities (`src/utils/`)
```
src/utils/
└── mod.rs               # Helper functions
```

**Utilities:**
- Formatting (currency, percentage)
- Calculations (ROI, profit)
- Common operations

## Configuration (`config/`)

```
config/
└── default.toml         # Default bot configuration
```

**Settings:**
- API endpoints
- Profit thresholds
- Position sizing
- Risk limits
- Database path

## Build Scripts (`scripts/`)

```
scripts/
├── build-macos.sh       # macOS build script (Intel + ARM)
└── build-windows.bat    # Windows build script
```

**Features:**
- Architecture detection
- Automated compilation
- Distribution packaging
- Platform-specific optimization

## Tests (`tests/`)

```
tests/
└── integration_tests.rs # Integration test suite
```

**Coverage:**
- Database operations
- Configuration loading
- Arbitrage detection
- API mocking

## Documentation (`docs/`)

```
docs/
├── ARCHITECTURE.md      # System architecture documentation
└── API.md              # API integration guide
```

**Topics:**
- System design
- Data flow
- API endpoints
- Best practices
- Troubleshooting

## GitHub Actions (`.github/workflows/`)

```
.github/workflows/
├── ci.yml              # Continuous integration
└── release.yml         # Automated releases
```

**CI Pipeline:**
- Multi-platform testing (Linux, macOS, Windows)
- Code formatting checks
- Linting with Clippy
- Security audits
- Automated builds

## Generated/Runtime Files (Not in Repo)

These files are created when you run the project:

```
# Build artifacts
target/                  # Compiled binaries and dependencies

# Runtime data
arbitrage.db            # SQLite database
arbitrage.db-shm        # Shared memory file
arbitrage.db-wal        # Write-ahead log

# Logs
logs/
└── bot.log             # Application logs

# Environment
.env                    # Your actual API keys (DO NOT COMMIT!)

# Distribution
dist/                   # Build output directory
├── macos-x86_64/
├── macos-arm64/
└── windows-x86_64/
```

## File Count Summary

- **Source files**: 8 Rust files
- **Configuration**: 6 files
- **Documentation**: 7 markdown files
- **Scripts**: 2 build scripts
- **Tests**: 1 test file
- **CI/CD**: 2 workflow files
- **Total tracked files**: ~35 files

## Dependencies (from Cargo.toml)

### Runtime Dependencies
- tokio (async runtime)
- reqwest (HTTP client)
- serde/serde_json (serialization)
- sqlx (database)
- rust_decimal (precise math)
- anyhow/thiserror (error handling)
- chrono (dates/times)
- log/env_logger (logging)
- clap (CLI)
- dotenv (environment)

### Development Dependencies
- mockito (API mocking)
- tokio-test (async testing)

### Platform-Specific
- security-framework (macOS keychain)
- winapi (Windows credential manager)

## Key File Relationships

```
main.rs
  ├── uses config/mod.rs for settings
  ├── uses database/mod.rs for persistence
  └── uses arbitrage/mod.rs for logic
      ├── uses api/polymarket.rs
      ├── uses api/kalshi.rs
      ├── uses models/mod.rs
      └── uses utils/mod.rs
```

## Build Outputs

### Debug Build
```
target/debug/
└── polymarket-kalshi-arbitrage-bot (or .exe on Windows)
```

### Release Build
```
target/release/
└── polymarket-kalshi-arbitrage-bot (optimized binary)
```

### Docker Image
```
polymarket-kalshi-bot:latest
```

---

**Note**: This structure follows Rust best practices and is organized for maintainability, testability, and scalability.
