# Development Guide

This guide helps you set up a development environment for pglance, a PostgreSQL extension for reading Lance format tables.

## Prerequisites

Install the following tools and dependencies:

### Required Tools
- **Rust** (latest stable) - https://rustup.rs/
- **PostgreSQL** (13-17) with development headers
- **Protocol Buffers compiler** (protoc) - required for Lance dependencies
- **Git** - for version control

### Optional Tools
- **just** (command runner) - https://github.com/casey/just#installation
- **cargo-audit** - for security auditing
- **cargo-outdated** - for dependency management

### System Dependencies

#### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    protobuf-compiler \
    postgresql-16 \
    postgresql-server-dev-16 \
    libreadline-dev \
    zlib1g-dev \
    libssl-dev \
    libicu-dev
```

#### macOS
```bash
brew install postgresql@16 protobuf
```

#### Arch Linux
```bash
sudo pacman -S postgresql postgresql-libs protobuf
```

## Quick Start

### 1. Clone and Setup

```bash
# Clone the repository
git clone <repository-url>
cd pglance

# Install pgrx with the exact version used by the project
cargo install cargo-pgrx --version=0.14.3 --locked

# Initialize pgrx (downloads and configures PostgreSQL)
cargo pgrx init
```

### 2. Development Workflow

```bash
# Check code formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --features pg16 -- -D warnings

# Run all tests
cargo test --features pg16

# Build extension
cargo pgrx package --features pg16

# Install and run PostgreSQL with extension
cargo pgrx install --features pg16
cargo pgrx run --features pg16
```

### 3. Using Just Commands (Recommended)

If you have `just` installed:

```bash
# Show all available commands
just

# Run all quality checks (format + lint + test)
just check

# Auto-format all code
just fmt

# Build extension
just build

# Run tests
just test

# Install and start PostgreSQL with extension
just run

# Simulate CI pipeline locally
just ci
```

## PostgreSQL Version Support

The extension supports PostgreSQL versions 13-17. Specify the version using feature flags:

```bash
# Using cargo directly
cargo test --features pg15
cargo pgrx install --features pg17

# Using just commands
just build pg=15
just test pg=17
just run pg=14
```

Default version is PostgreSQL 16.

## Project Structure

```
pglance/
├── src/
│   ├── lib.rs                  # Main extension entry point
│   ├── types/                  # Type conversion utilities
│   │   ├── mod.rs              # Module exports
│   │   ├── conversion.rs       # Arrow ↔ PostgreSQL type mapping
│   │   └── arrow_convert.rs    # Arrow value conversion logic
│   └── scanner/                # Lance table scanning
│       ├── mod.rs              # Module exports
│       └── lance_scanner.rs    # Core scanning implementation
├── .github/workflows/          # CI/CD pipelines
├── Cargo.toml                  # Dependencies and metadata
├── justfile                    # Development commands
├── pglance.control             # PostgreSQL extension metadata
└── docs/                       # Documentation files
```

## Key Components

### 1. Core Functions (`src/lib.rs`)
- `hello_pglance()` - Extension verification
- `lance_table_info()` - Schema inspection
- `lance_table_stats()` - Table statistics
- `lance_scan_jsonb()` - Data scanning with JSONB output

### 2. Type System (`src/types/`)
- Arrow to PostgreSQL type mapping
- Value conversion utilities
- JSONB serialization for complex types

### 3. Scanner (`src/scanner/`)
- Lance table reading logic
- Async/sync integration
- Error handling and recovery

## Development Workflow

### Daily Development
1. **Make changes** to Rust code
2. **Format code**: `just fmt` or `cargo fmt --all`
3. **Check compilation**: `cargo check --features pg16`
4. **Run linter**: `just clippy` or `cargo clippy --features pg16`
5. **Run tests**: `just test` or `cargo test --features pg16`
6. **Test manually**: `just run` to start PostgreSQL

### Before Committing
```bash
# Run all quality checks
just ci

# Or manually:
cargo fmt --all -- --check
cargo clippy --features pg16 -- -D warnings
cargo test --features pg16
```

### Testing Your Changes
```bash
# Start PostgreSQL with your extension
just run

# In another terminal, connect to test database
psql pglance

# Test your functions
SELECT hello_pglance();
SELECT * FROM lance_table_info('/path/to/test/table');
```

## Available Commands

| Command | Description |
|---------|-------------|
| `just` | Show all available commands |
| `just check` | Run format check + clippy + tests |
| `just fmt` | Auto-format all code |
| `just build` | Build extension package |
| `just test` | Run all tests |
| `just run` | Start PostgreSQL with extension |
| `just install` | Install extension locally |
| `just ci` | Simulate full CI pipeline |
| `just clean` | Clean build artifacts |
| `just audit` | Run security audit |

## Testing

### Test Types
1. **Unit Tests** - Basic functionality tests in `src/lib.rs`
2. **Integration Tests** - Full end-to-end tests with Lance data
3. **PostgreSQL Tests** - pgrx framework tests

### Running Tests
```bash
# All tests
cargo test --features pg16

# Specific test
cargo test --features pg16 test_hello_pglance

# With verbose output
cargo test --features pg16 -- --nocapture
```

### Test Data
Tests create temporary Lance tables automatically using the `LanceTestDataGenerator`. No external test data files are required.

## Debugging

### Compilation Issues
```bash
# Check for basic errors
cargo check --features pg16

# Detailed error information
RUST_BACKTRACE=1 cargo build --features pg16
```

### Runtime Debugging
```bash
# Enable debug logging
RUST_LOG=debug just run

# Or in PostgreSQL:
SET log_min_messages = DEBUG1;
SELECT hello_pglance();
```

### Common Issues

**"cargo-pgrx not found"**
```bash
cargo install cargo-pgrx --version=0.14.3 --locked
```

**"PostgreSQL development headers missing"**
```bash
# Ubuntu/Debian
sudo apt-get install postgresql-server-dev-16

# macOS
brew install postgresql@16
```

**"protoc not found"**
```bash
# Ubuntu/Debian
sudo apt-get install protobuf-compiler

# macOS
brew install protobuf
```

## Contributing

### Code Style
- Follow Rust standard formatting: `cargo fmt`
- Address all clippy warnings: `cargo clippy`
- Add tests for new functionality
- Update documentation for API changes

### Pull Request Process
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `just ci` to ensure all checks pass
5. Submit a pull request with clear description

### Commit Messages
Use conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation updates
- `test:` for test additions/changes
- `refactor:` for code refactoring

## Release Process

Releases are automated through GitHub Actions:
1. Tag a new version: `git tag v0.x.y`
2. Push tags: `git push --tags`
3. GitHub Actions builds and publishes release artifacts

## Performance Considerations

### Development Mode
- Use `cargo check` for fast syntax checking
- Use `--features pg16` to avoid building all PostgreSQL versions
- Consider using `cargo build` instead of `cargo pgrx package` for faster iteration

### Production Builds
```bash
# Optimized release build
just build-release

# Or manually:
cargo pgrx package --features pg16 --release
```

## IDE Setup

### VS Code
Recommended extensions:
- rust-analyzer
- PostgreSQL syntax highlighting
- Even Better TOML

### Settings
```json
{
  "rust-analyzer.cargo.features": ["pg16"],
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

## Additional Resources

- [pgrx Documentation](https://github.com/pgcentralfoundation/pgrx)
- [Lance Documentation](https://lancedb.github.io/lance/)
- [PostgreSQL Extension Documentation](https://www.postgresql.org/docs/current/extend.html)
- [Rust Book](https://doc.rust-lang.org/book/)

## Getting Help

- Open an issue on GitHub for bugs or feature requests
- Check existing documentation in `README.md` and `TESTING.md`
- Review the source code - it's well-documented
- Ask questions in discussions or issues