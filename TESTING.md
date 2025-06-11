# Testing Guide

This document describes the comprehensive testing approach for pglance, a PostgreSQL extension for reading Lance format tables.

## Overview

pglance uses a **pure Rust testing approach** that eliminates external dependencies and provides fast, reliable testing. All tests (unit and integration) are written in Rust and executed using the pgrx testing framework.

## Test Architecture

### Pure Rust Testing Benefits
✅ **No Docker Required** - Tests run directly with `cargo test`  
✅ **Fast Execution** - No container startup overhead  
✅ **Simple Setup** - Only requires Rust and pgrx installation  
✅ **Unified Testing** - Unit and integration tests in one command  
✅ **Comprehensive Coverage** - Tests the complete data pipeline  
✅ **Easy Debugging** - Standard Rust debugging tools work seamlessly  
✅ **CI/CD Friendly** - Runs in any environment with Rust support  

## Test Structure

### 1. Unit Tests
Located in `src/lib.rs` under the `tests` module:

- **`test_hello_pglance()`** - Verifies basic extension functionality
- **`test_error_handling()`** - Tests error handling with invalid file paths

### 2. Integration Tests  
Comprehensive end-to-end tests that create real Lance datasets:

- **`test_simple_table_integration()`** - Tests basic data types (int, string, float, boolean)
- **`test_vector_table_integration()`** - Tests vector embeddings and complex data structures

### 3. Test Data Generation
The `LanceTestDataGenerator` creates temporary Lance datasets with:

- **Simple Table**: Scalar data types (ID, name, age, salary, status)
- **Vector Table**: Embeddings as Arrow List arrays with metadata
- **Automatic Cleanup**: Temporary directories are cleaned up after tests

## Running Tests

### Quick Test Commands

```bash
# Run all tests
cargo test --features pg16

# Run with just (if installed)
just test

# Run specific test
cargo test --features pg16 test_hello_pglance

# Verbose output
cargo test --features pg16 -- --nocapture
```

### Full Quality Checks

```bash
# Run all quality checks (format + lint + test)
just ci

# Or manually:
cargo fmt --all -- --check
cargo clippy --features pg16 -- -D warnings
cargo test --features pg16
```

### PostgreSQL Version Testing

```bash
# Test with different PostgreSQL versions
cargo test --features pg15
cargo test --features pg17

# Using just
just test pg=15
just test pg=17
```

## Test Categories

### 1. Basic Functionality Tests

```rust
#[pg_test]
fn test_hello_pglance() {
    assert_eq!("Hello, pglance", crate::hello_pglance());
}
```

Tests the most basic extension functionality to ensure the extension loads correctly.

### 2. Error Handling Tests

```rust
#[pg_test]
fn test_error_handling() {
    // Test with invalid path
    let result = std::panic::catch_unwind(|| {
        crate::lance_table_info("/nonexistent/path")
    });
    assert!(result.is_err());
}
```

Validates that the extension handles error conditions gracefully.

### 3. Schema Detection Tests

Tests that `lance_table_info()` correctly identifies:
- Column names and types
- Nullability constraints
- Data type mappings from Arrow to PostgreSQL

### 4. Data Scanning Tests

Tests that `lance_scan_jsonb()` correctly:
- Reads Lance table data
- Converts Arrow values to JSONB
- Handles different data types
- Respects limit parameters

### 5. Statistics Tests

Tests that `lance_table_stats()` returns accurate:
- Row counts
- Column counts
- Table version information

## Test Data Scenarios

### Simple Table Structure
```sql
-- Generated test data includes:
id: int64
name: string
age: int32
salary: float64
active: boolean
```

### Vector Table Structure  
```sql
-- Generated test data includes:
id: int64
embedding: float32[] (fixed-size list)
metadata: struct (converted to JSONB)
```

## Key Features Tested

| Feature | Test Coverage |
|---------|---------------|
| **Schema Detection** | Column types, nullability, data type mapping |
| **Data Scanning** | Full table scans, limited scans, type conversion |
| **Statistics** | Row/column counts, version info |
| **Error Handling** | Invalid paths, malformed data |
| **Type Conversion** | Arrow to PostgreSQL type mapping |
| **JSONB Output** | Complex data structure serialization |
| **Performance** | Memory usage, scan efficiency |

## Debugging Tests

### Enable Debug Logging
```bash
# Run tests with debug output
RUST_LOG=debug cargo test --features pg16 -- --nocapture

# Specific test with logging
RUST_LOG=debug cargo test --features pg16 test_simple_table_integration -- --nocapture
```

### Test with Different Log Levels
```bash
# Error level only
RUST_LOG=error cargo test --features pg16

# Full trace logging
RUST_LOG=trace cargo test --features pg16
```

### Manual Testing
```bash
# Start PostgreSQL with extension for manual testing
just run

# In another terminal
psql pglance

# Test functions manually
SELECT hello_pglance();
SELECT * FROM lance_table_info('/tmp/test_table');
```

## Test Performance

### Typical Test Execution Times
- **Unit Tests**: < 1 second
- **Integration Tests**: 2-5 seconds per test
- **Full Test Suite**: < 30 seconds

### Memory Usage
- Tests use temporary directories with automatic cleanup
- Memory usage scales with test data size
- Large dataset tests use limited row counts for efficiency

## CI/CD Integration

### GitHub Actions Workflow
Tests run automatically on:
- Push to main branch
- Pull requests
- Release tags

### Local CI Simulation
```bash
# Simulate full CI pipeline locally
just ci

# This runs:
# 1. cargo fmt --all -- --check
# 2. cargo clippy --features pg16 -- -D warnings  
# 3. cargo test --features pg16
```

## Test Data Management

### Temporary File Handling
```rust
// Tests create temporary directories automatically
let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
let table_path = temp_dir.path().join("test_table");
```

### Cleanup Strategy
- All test data is created in temporary directories
- Automatic cleanup via `tempfile` crate
- No persistent test files or external dependencies

## Adding New Tests

### Unit Test Template
```rust
#[pg_test]
fn test_new_functionality() {
    // Arrange
    let input = "test_data";
    
    // Act
    let result = crate::new_function(input);
    
    // Assert
    assert_eq!(expected_value, result);
}
```

### Integration Test Template
```rust
#[pg_test]
fn test_new_integration() {
    let generator = LanceTestDataGenerator::new()
        .expect("Failed to create test data generator");
    
    let table_path = generator.create_custom_table();
    
    // Test your functionality
    let result = crate::your_function(&table_path);
    
    // Validate results
    assert!(result.is_ok());
}
```

## Test Guidelines

### Best Practices
1. **Use descriptive test names** that explain what is being tested
2. **Include both positive and negative test cases**
3. **Test edge cases** (empty tables, large datasets, invalid data)
4. **Use temporary data** - never rely on external files
5. **Clean up resources** - let Rust's RAII handle cleanup
6. **Test error conditions** explicitly

### Code Coverage
- Aim for high code coverage of core functionality
- Focus on critical paths and error handling
- Use integration tests to cover end-to-end workflows

## Troubleshooting

### Common Test Issues

**"cargo-pgrx not found"**
```bash
cargo install cargo-pgrx --version=0.14.3 --locked
cargo pgrx init
```

**"PostgreSQL initialization failed"**
```bash
# Clean pgrx installation
rm -rf ~/.pgrx
cargo pgrx init
```

**"Test data creation failed"**
```bash
# Ensure sufficient disk space and permissions
df -h /tmp
ls -la /tmp
```

### Test Debugging

**Failed Assertions**
```bash
# Run single test with full output
cargo test --features pg16 test_name -- --nocapture --exact
```

**Memory Issues**
```bash
# Monitor memory usage during tests
cargo test --features pg16 & 
watch -n 1 'ps aux | grep postgres'
```

## Migration from Docker-Based Testing

### Previous Approach (Deprecated)
The original testing used:
- Python scripts for Lance data creation
- Docker containers for PostgreSQL + pglance  
- Complex shell script orchestration
- External dependencies and setup complexity

### Current Advantages
- **Eliminated Docker dependency** - No containers required
- **Faster test execution** - No startup overhead
- **Simplified CI/CD** - Runs anywhere Rust works
- **Better debugging** - Native Rust tooling support
- **Unified codebase** - Tests and code in same language

## Example Test Output

```
running 4 tests
test tests::pg_test_hello_pglance ... ok
test tests::pg_test_error_handling ... ok  
test tests::pg_test_simple_table_integration ... ok
test tests::pg_test_vector_table_integration ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.34s
```

## Commands Reference

| Command | Description |
|---------|-------------|
| `cargo test --features pg16` | Run all tests |
| `just test` | Run tests (using just) |
| `just ci` | Full quality check pipeline |
| `cargo test --features pg16 -- --nocapture` | Tests with output |
| `RUST_LOG=debug cargo test --features pg16` | Tests with debug logging |
| `cargo test --features pg15` | Test with PostgreSQL 15 |

This testing approach ensures reliable, fast, and maintainable tests for the pglance PostgreSQL extension while eliminating external dependencies and providing excellent developer experience.