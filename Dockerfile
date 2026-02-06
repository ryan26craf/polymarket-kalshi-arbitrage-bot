# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY config ./config

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/polymarket-kalshi-arbitrage-bot /usr/local/bin/

# Copy config
COPY config ./config

# Create data directory
RUN mkdir -p /app/data /app/logs

# Set environment variables
ENV DATABASE_URL=sqlite:///app/data/arbitrage.db
ENV RUST_LOG=info

# Volume for persistent data
VOLUME ["/app/data", "/app/logs"]

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD pgrep -f polymarket-kalshi-arbitrage-bot || exit 1

# Run the bot
ENTRYPOINT ["polymarket-kalshi-arbitrage-bot"]
CMD ["--mode", "monitor"]
