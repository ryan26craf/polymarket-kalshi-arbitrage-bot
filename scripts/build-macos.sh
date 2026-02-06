#!/bin/bash
set -e

echo "Building Polymarket-Kalshi Arbitrage Bot for macOS..."

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    TARGET="aarch64-apple-darwin"
    echo "Building for Apple Silicon (ARM64)..."
elif [ "$ARCH" = "x86_64" ]; then
    TARGET="x86_64-apple-darwin"
    echo "Building for Intel (x86_64)..."
else
    echo "Unknown architecture: $ARCH"
    exit 1
fi

# Install target if not present
rustup target add $TARGET

# Build release binary
echo "Compiling release build..."
cargo build --release --target $TARGET

# Create distribution directory
DIST_DIR="dist/macos-$ARCH"
mkdir -p $DIST_DIR

# Copy binary
cp target/$TARGET/release/polymarket-kalshi-arbitrage-bot $DIST_DIR/

# Copy configuration files
cp config/default.toml $DIST_DIR/
cp .env.example $DIST_DIR/.env

# Create README for distribution
cat > $DIST_DIR/README.txt << EOF
Polymarket-Kalshi Arbitrage Bot - macOS Distribution

Installation:
1. Copy .env to your home directory and configure your API keys
2. Run: ./polymarket-kalshi-arbitrage-bot --help

For full documentation, visit:
https://github.com/yourusername/polymarket-kalshi-arbitrage-bot
EOF

echo "Build complete! Binary located at: $DIST_DIR/polymarket-kalshi-arbitrage-bot"
echo "Run: cd $DIST_DIR && ./polymarket-kalshi-arbitrage-bot --help"
