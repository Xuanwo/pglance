# pglance CI/CD Pipeline

This document describes the Continuous Integration and Continuous Deployment setup for the pglance PostgreSQL extension project.

## Overview

The CI/CD pipeline is split into separate workflows for better organization and parallel execution:

- **Rust Checks** (`rust-checks.yml`) - Code formatting, linting, and testing for Rust code
- **Python Checks** (`python-checks.yml`) - Code quality checks for Python integration tests
- **Integration Tests** (`integration-tests.yml`) - End-to-end testing with PostgreSQL
- **Security Audit** (`security.yml`) - Security vulnerability scanning
- **Release Build** (`release.yml`) - Building and publishing release artifacts

## Workflows

### 🦀 Rust Checks (`rust-checks.yml`)

Runs on every push and pull request to `main` branch.

**Jobs:**
- **rust-formatting**: Checks Rust code formatting with `cargo fmt`
- **rust-clippy**: Runs Clippy linter across all PostgreSQL versions (13-17)
- **rust-tests**: Executes unit tests and builds the extension

**Matrix Strategy:** Tests against PostgreSQL versions 13, 14, 15, 16, and 17.

### 🐍 Python Checks (`python-checks.yml`)

Runs on every push and pull request to `main` branch.

**Jobs:**
- **python-formatting**: Checks Python code formatting with `ruff format`
- **python-linting**: Runs Python linting with `ruff check`
- **python-imports**: Validates import sorting with `ruff`

**Tools Used:**
- `uv` for Python package management
- `ruff` for formatting and linting

### 🧪 Integration Tests (`integration-tests.yml`)

Runs comprehensive end-to-end tests after code quality checks pass.

**Jobs:**
- **integration-tests**: Runs full integration test suite with Docker
- **smoke-tests**: Quick validation tests

**Matrix Strategy:** Tests against PostgreSQL versions 13, 15, and 16.

**Features:**
- Builds the extension for each PostgreSQL version
- Runs integration tests in Docker containers
- Uploads test artifacts on failure for debugging

### 🔒 Security Audit (`security.yml`)

Runs on pushes, pull requests, and daily at 2 AM UTC.

**Jobs:**
- **rust-security**: Runs `cargo audit` for Rust dependencies
- **python-security**: Checks Python dependencies with `safety`
- **dependency-review**: GitHub's dependency review for pull requests

### 🚀 Release Build (`release.yml`)

Runs on pushes to `main` branch and tagged releases.

**Jobs:**
- **build-release**: Creates release artifacts for all PostgreSQL versions
- **create-github-release**: Creates GitHub releases for tagged versions
- **check-dependencies**: Monitors dependency freshness

## Local Development

### Quick Quality Checks

Run all code quality checks locally:

```bash
just check
```

This command runs:
- Rust formatting check (`cargo fmt --check`)
- Rust linting (`cargo clippy`)
- Rust tests (`cargo test`)
- Python formatting check (`ruff format --check`)
- Python linting (`ruff check`)

### Auto-Format Code

Automatically format all code:

```bash
just fmt
```

This command:
- Formats Rust code (`cargo fmt`)
- Formats Python code (`ruff format`)
- Auto-fixes linting issues (`ruff check --fix`)

### Just Commands

```bash
# Show all available commands
just

# Run quality checks
just check

# Format code
just fmt

# Build extension
just build

# Run tests
just test

# Setup development environment
just setup

# Run integration tests
just e2e

# Start PostgreSQL with extension
just run

# Simulate CI locally
just ci
```

You can also specify PostgreSQL version:
```bash
just build pg=15
just test pg=17
```

## Configuration

### Rust Configuration

Rust formatting and linting use default settings from:
- `rustfmt.toml` (if present)
- `Cargo.toml` workspace configuration

### Python Configuration

Python tools are configured in `integration_tests/pyproject.toml`:

```toml
[tool.ruff]
target-version = "py38"
line-length = 88

[tool.ruff.lint]
select = ["E", "W", "F", "I", "B", "C4", "UP"]
ignore = ["E501"]  # Line length handled by formatter
```

## PostgreSQL Version Support

The CI pipeline tests against multiple PostgreSQL versions:

- **Full Testing**: PostgreSQL 13, 14, 15, 16, 17 (Rust checks)
- **Integration Testing**: PostgreSQL 13, 15, 16 (selected versions)
- **Smoke Testing**: PostgreSQL 16 (latest stable)

## Artifacts and Caching

**Caching Strategy:**
- Rust dependencies cached by `Cargo.lock` hash
- Separate cache keys for different PostgreSQL versions
- Cache restoration with fallback keys

**Artifacts:**
- Test logs uploaded on failure (7-day retention)
- Release builds uploaded for main branch pushes (30-day retention)
- GitHub releases created for tagged versions

## Security

**Automated Security Checks:**
- Daily security audits at 2 AM UTC
- Dependency vulnerability scanning
- GitHub dependency review for PRs

**Security Tools:**
- `cargo audit` for Rust dependencies
- `safety` for Python dependencies
- GitHub's dependency review action

## Troubleshooting

### Common Issues

**"PostgreSQL not found" errors:**
- Ensure PostgreSQL development packages are installed
- Check that `pg_config` is in PATH

**Python dependency issues:**
- Ensure `uv` is installed and up to date
- Run `uv sync` to update dependencies

**Cache issues:**
- Cache keys include `Cargo.lock` and PostgreSQL version
- Clear cache if persistent issues occur

### Debug Integration Tests

When integration tests fail:
1. Check uploaded artifacts for test logs
2. Run tests locally with `./integration_tests/run_tests.sh`
3. Check Docker container logs for PostgreSQL issues

### Performance

**Optimization Strategies:**
- Parallel job execution across workflows
- Dependency caching to reduce build times
- Matrix builds only for critical combinations
- Separate formatting checks for fast feedback

## Contributing

When contributing to the project:

1. Run `just check` before committing
2. Use `just fmt` to auto-fix formatting
3. Ensure all CI checks pass before merging
4. Add tests for new functionality

The CI pipeline will automatically run on pull requests and provide feedback on code quality and test results.