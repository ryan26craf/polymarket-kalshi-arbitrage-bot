# API Integration Guide

## Polymarket API

### Authentication
Polymarket uses API key authentication:
```rust
headers: {
    "Authorization": "Bearer YOUR_API_KEY"
}
```

### Endpoints

#### Get Markets
```
GET /markets
```

Response:
```json
{
  "markets": [
    {
      "id": "market_id",
      "question": "Will X happen?",
      "bestBid": "0.45",
      "bestAsk": "0.55",
      "volume": "100000",
      "liquidity": "50000",
      "endDate": "2025-12-31T23:59:59Z"
    }
  ]
}
```

#### Place Order
```
POST /orders
```

Request:
```json
{
  "market_id": "market_id",
  "side": "buy",
  "price": "0.50",
  "amount": "100"
}
```

### Rate Limits
- 100 requests per minute
- Burst limit: 200 requests

### Error Codes
- `400`: Bad Request
- `401`: Unauthorized
- `429`: Rate Limited
- `500`: Internal Server Error

## Kalshi API

### Authentication
Kalshi uses API key + secret with HMAC signatures:
```rust
headers: {
    "Authorization": "Bearer YOUR_API_KEY",
    "X-Signature": hmac_sha256(path + body, secret)
}
```

### Endpoints

#### Get Markets
```
GET /trade-api/v2/markets
```

Response:
```json
{
  "markets": [
    {
      "ticker": "MARKET-TICKER",
      "title": "Will X happen?",
      "yes_bid": 45,
      "yes_ask": 55,
      "volume": 1000,
      "open_interest": 500,
      "close_time": "2025-12-31T23:59:59Z"
    }
  ]
}
```

#### Place Order
```
POST /trade-api/v2/portfolio/orders
```

Request:
```json
{
  "ticker": "MARKET-TICKER",
  "action": "buy",
  "side": "yes",
  "count": 10,
  "type": "limit",
  "yes_price": 50
}
```

### Price Format
- Kalshi uses cents (0-100)
- 45 = $0.45
- 100 = $1.00

### Rate Limits
- 60 requests per minute
- Order rate: 10 per second

### Error Codes
- `400`: Invalid request
- `401`: Authentication failed
- `403`: Insufficient permissions
- `429`: Rate limit exceeded

## Best Practices

### Connection Management
```rust
// Reuse HTTP clients
let client = Client::builder()
    .timeout(Duration::from_secs(30))
    .pool_max_idle_per_host(10)
    .build()?;
```

### Error Handling
```rust
match api_call().await {
    Ok(response) => process(response),
    Err(e) if e.is_timeout() => retry(),
    Err(e) if e.is_rate_limit() => backoff(),
    Err(e) => log_error(e),
}
```

### Rate Limiting
```rust
use tokio::time::{sleep, Duration};

let mut last_request = Instant::now();
let min_interval = Duration::from_millis(100);

loop {
    let elapsed = last_request.elapsed();
    if elapsed < min_interval {
        sleep(min_interval - elapsed).await;
    }
    
    make_request().await?;
    last_request = Instant::now();
}
```

### Retry Strategy
```rust
use backoff::{ExponentialBackoff, Operation};

let mut backoff = ExponentialBackoff::default();

retry_with_backoff(|| async {
    api_call().await
}, &mut backoff).await?;
```

## Testing API Integration

### Mock Servers
```rust
#[cfg(test)]
mod tests {
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_get_markets() {
        let _m = mock("GET", "/markets")
            .with_status(200)
            .with_body(r#"{"markets": []}"#)
            .create();

        let client = PolymarketClient::new(
            "test_key".to_string(),
            server_url(),
        );

        let markets = client.get_markets().await.unwrap();
        assert_eq!(markets.len(), 0);
    }
}
```

## Security Considerations

1. **Never log API keys**
2. **Use environment variables for secrets**
3. **Rotate keys regularly**
4. **Monitor for unusual activity**
5. **Implement request signing**
6. **Validate all responses**

## Common Issues

### Issue: 401 Unauthorized
**Solution**: Check API key validity and format

### Issue: 429 Rate Limited
**Solution**: Implement exponential backoff

### Issue: Timeout
**Solution**: Increase timeout or check network connectivity

### Issue: Invalid Signature (Kalshi)
**Solution**: Verify HMAC implementation and time sync
