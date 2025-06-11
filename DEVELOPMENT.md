# Development Setup

This guide helps you set up a development environment for pglance.

## Prerequisites

Install the following tools:

- **Rust** (latest stable) - https://rustup.rs/
- **uv** (Python package manager) - https://docs.astral.sh/uv/getting-started/installation/
- **just** (command runner) - https://github.com/casey/just#installation
- **PostgreSQL** (13-17) with development headers

### Installing PostgreSQL on Ubuntu/Debian

```bash
sudo apt-get update
sudo apt-get install postgresql-16 postgresql-server-dev-16
```

### Installing PostgreSQL on macOS

```bash
brew install postgresql@16
```

## Quick Start

1. **Clone and setup:**
   ```bash
   git clone <repository-url>
   cd pglance
   just setup
   ```

2. **Run checks:**
   ```bash
   just check
   ```

3. **Build and test:**
   ```bash
   just build
   just test
   ```

4. **Start PostgreSQL with extension:**
   ```bash
   just run
   ```

## Available Commands

Run `just` to see all available commands:

```bash
just                    # Show all commands
just check              # Run all quality checks
just fmt                # Auto-format code
just build              # Build extension
just test               # Run tests
just e2e                # Run integration tests
just run                # Start PostgreSQL with extension
just ci                 # Simulate CI locally
```

## PostgreSQL Version Support

Specify PostgreSQL version for commands:

```bash
just build pg=15        # Build for PostgreSQL 15
just test pg=17         # Test with PostgreSQL 17
```

Supported versions: 13, 14, 15, 16, 17

## Development Workflow

1. **Make changes** to Rust or Python code
2. **Format code:** `just fmt`
3. **Run checks:** `just check`
4. **Test locally:** `just test`
5. **Integration test:** `just e2e`
6. **Commit changes**

## Python Development

Python integration tests are in `integration_tests/`:

```bash
cd integration_tests
uv sync                 # Install dependencies
uv run python integration_test.py  # Run tests
```

## CI Pipeline

The CI runs automatically on push/PR to main branch:
- Rust formatting, clippy, tests
- Python formatting, linting
- Integration tests
- Security audits

Simulate CI locally: `just ci`
