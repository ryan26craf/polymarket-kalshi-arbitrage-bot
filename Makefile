.PHONY: build run test clean install help fmt clippy check release

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project in debug mode
	cargo build

release: ## Build the project in release mode
	cargo build --release

run: ## Run the bot in monitoring mode
	cargo run -- --mode monitor

execute: ## Run the bot in execution mode (WARNING: real trades)
	@echo "WARNING: This will execute real trades!"
	@read -p "Are you sure? (yes/no): " confirm && [ "$$confirm" = "yes" ] || exit 1
	cargo run -- --mode execute

test: ## Run all tests
	cargo test

test-verbose: ## Run tests with output
	cargo test -- --nocapture

fmt: ## Format code with rustfmt
	cargo fmt --all

clippy: ## Run clippy linter
	cargo clippy --all-targets --all-features

check: fmt clippy test ## Run all checks (fmt, clippy, test)

clean: ## Clean build artifacts
	cargo clean
	rm -rf dist/
	rm -f *.db *.db-shm *.db-wal

install: ## Install dependencies
	rustup update
	rustup component add rustfmt clippy

setup: ## Setup development environment
	cp .env.example .env
	@echo "Please edit .env with your API credentials"

db-reset: ## Reset the database
	rm -f arbitrage.db arbitrage.db-shm arbitrage.db-wal
	@echo "Database reset complete"

logs: ## Show recent logs
	tail -f logs/bot.log

watch: ## Run the bot with auto-reload on file changes
	cargo watch -x 'run -- --mode monitor'

docs: ## Generate and open documentation
	cargo doc --open

bench: ## Run benchmarks
	cargo bench

audit: ## Security audit
	cargo audit

update: ## Update dependencies
	cargo update

# Platform-specific builds
build-macos: ## Build for macOS
	./scripts/build-macos.sh

build-windows: ## Build for Windows
	./scripts/build-windows.bat

build-all: ## Build for all platforms (requires cross)
	cross build --release --target x86_64-unknown-linux-gnu
	cross build --release --target x86_64-apple-darwin
	cross build --release --target aarch64-apple-darwin
	cross build --release --target x86_64-pc-windows-msvc

docker-build: ## Build Docker image
	docker build -t polymarket-kalshi-bot .

docker-run: ## Run in Docker
	docker run -it --env-file .env polymarket-kalshi-bot
