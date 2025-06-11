# pglance development commands

# Default PostgreSQL version
pg_version := "16"

# Show available commands
default:
    @just --list

# Run all quality checks
check pg=pg_version:
    cargo fmt --all -- --check
    cargo clippy --features pg{{pg}} -- -D warnings
    cargo test --features pg{{pg}}
    cd integration_tests && uv run ruff format --check .
    cd integration_tests && uv run ruff check .

# Auto-format all code
fmt:
    cargo fmt --all
    cd integration_tests && uv run ruff format .
    cd integration_tests && uv run ruff check . --fix

# Run Rust tests
test pg=pg_version:
    cargo test --features pg{{pg}}

# Build extension
build pg=pg_version:
    cargo pgrx package --features pg{{pg}}

# Build release version
build-release pg=pg_version:
    cargo pgrx package --features pg{{pg}} --release

# Install extension locally
install pg=pg_version: (build pg)
    cargo pgrx install --features pg{{pg}}

# Run integration tests
e2e:
    cd integration_tests && ./run_tests.sh

# Run clippy
clippy pg=pg_version:
    cargo clippy --features pg{{pg}} -- -D warnings

# Setup development environment
setup:
    cargo install cargo-pgrx --version=0.14.3 --locked
    cargo pgrx init --pg{{pg_version}}
    cd integration_tests && uv sync

# Clean build artifacts
clean:
    cargo clean
    find . -name "*.pyc" -delete
    find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true

# Start PostgreSQL with extension
run pg=pg_version: (install pg)
    cargo pgrx run --features pg{{pg}}

# Security audit
audit:
    cargo audit

# Check for outdated dependencies
deps:
    cargo outdated
    cd integration_tests && uv tree

# Quick fix for common issues
fix:
    cargo fmt --all
    cd integration_tests && uv run ruff format .
    cd integration_tests && uv run ruff check . --fix

# Simulate CI locally
ci pg=pg_version:
    @echo "🦀 Rust checks..."
    cargo fmt --all -- --check
    cargo clippy --features pg{{pg}} -- -D warnings
    cargo test --features pg{{pg}}
    @echo "🐍 Python checks..."
    cd integration_tests && uv run ruff format --check .
    cd integration_tests && uv run ruff check .
    @echo "✅ All checks passed!"