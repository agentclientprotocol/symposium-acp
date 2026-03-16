# Build binaries needed for integration tests
prep-tests:
    cargo build -p sacp-conductor
    cargo build -p sacp-test --bin testy
    cargo build -p sacp-test --bin mcp-echo-server --example arrow_proxy

# Run all tests (requires prep-tests first)
test: prep-tests
    cargo test --all --workspace
