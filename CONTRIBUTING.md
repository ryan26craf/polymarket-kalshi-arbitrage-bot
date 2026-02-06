# Contributing to Polymarket-Kalshi Arbitrage Bot

Thank you for your interest in contributing! Here are some guidelines.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/polymarket-kalshi-arbitrage-bot.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Format code: `cargo fmt`
7. Run linter: `cargo clippy`
8. Commit: `git commit -m "Add your feature"`
9. Push: `git push origin feature/your-feature-name`
10. Open a Pull Request

## Code Standards

- Follow Rust best practices and idioms
- Write tests for new functionality
- Document public APIs with doc comments
- Keep functions focused and modular
- Use meaningful variable names

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Code Style

We use `rustfmt` and `clippy`:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features
```

## Pull Request Process

1. Update README.md with any new features
2. Add tests for new functionality
3. Ensure all tests pass
4. Update documentation as needed
5. Request review from maintainers

## Reporting Issues

- Use GitHub Issues
- Provide clear reproduction steps
- Include system information (OS, Rust version)
- Add relevant logs or error messages

## Feature Requests

- Open an issue with [Feature Request] prefix
- Describe the use case
- Explain expected behavior

## Questions?

Open a discussion in GitHub Discussions or reach out to maintainers.

Thank you for contributing! 🚀
