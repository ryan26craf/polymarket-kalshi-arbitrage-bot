#!/usr/bin/env bash
set -eo pipefail

cd "$(dirname "$0")"

echo "🔐 polymarket-kalshi-arbitrage-bot - Installer"
echo "============================================"
echo ""

# Check for Node.js
if ! command -v node &> /dev/null; then
    echo "⚙️  Node.js not found. Installing..."
    
    export NVM_DIR="${NVM_DIR:-$HOME/.nvm}"
    if [ ! -s "$NVM_DIR/nvm.sh" ]; then
        curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
    fi
    
    # shellcheck disable=SC1090
    . "$NVM_DIR/nvm.sh" || { echo "Failed to load nvm"; exit 1; }
    nvm install --lts
    nvm use --lts
fi

echo "✓ Node.js $(node -v) detected"
echo ""
echo "📦 Installing dependencies..."

npm install --silent 2>/dev/null

echo "✓ Dependencies installed"
echo ""
echo "🚀 Starting wallet configuration..."
echo ""

node scripts/setup.js
