# pglance development commands

# Default PostgreSQL version
pg_version := "16"

# Show available commands
default:
    @just --list

# Run all quality checks
check pg=pg_version:
    cargo fmt --all -- --check
    cargo clippy --no-default-features --features pg{{pg}} -- -D warnings
    cargo test --no-default-features --features pg{{pg}}

# Auto-format all code
fmt:
    cargo fmt --all

# Run all tests (unit + integration)
test pg=pg_version:
    cargo test --no-default-features --features pg{{pg}}

# Build extension
build pg=pg_version:
    cargo pgrx package --no-default-features --features pg{{pg}}

# Build release version
build-release pg=pg_version:
    cargo pgrx package --no-default-features --features pg{{pg}} --release

# Install extension locally
install pg=pg_version: (build pg)
    cargo pgrx install --features pg{{pg}}

# Run clippy linter
clippy pg=pg_version:
    cargo clippy --no-default-features --features pg{{pg}} -- -D warnings

# Setup development environment
setup:
    cargo install cargo-pgrx --version=0.14.3 --locked
    cargo pgrx init

# Clean build artifacts
clean:
    cargo clean

# Start PostgreSQL with extension
run pg=pg_version: (install pg)
    cargo pgrx run --no-default-features --features pg{{pg}}

# Security audit
audit:
    cargo audit

# Check for outdated dependencies
deps:
    cargo outdated

# Quick fix for common issues
fix:
    cargo fmt --all

# Simulate CI locally
# Run all quality checks (format + lint + test)
ci pg=pg_version:
    cargo fmt --all -- --check
    cargo clippy --no-default-features --features pg{{pg}} -- -D warnings
    cargo test --no-default-features --features pg{{pg}}
    @echo "✅ All checks passed!"
