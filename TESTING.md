# pglance Testing Guide

This document describes the testing approach for pglance, a PostgreSQL extension for reading Lance data format files.

## Overview

pglance uses a **pure Rust testing approach** that eliminates the need for Docker or external dependencies. All tests (unit and integration) are written in Rust and executed using the pgrx testing framework with a single command.

## Test Structure

### 1. Unit Tests
Basic functionality tests located in `src/lib.rs`:
- `test_hello_pglance()` - Tests the basic extension functionality
- `test_error_handling()` - Tests error handling with invalid paths

### 2. Integration Tests
Comprehensive tests that:
1. **Create Lance datasets** using the Rust Lance library with various data types
2. **Test pglance functions** against these datasets through PostgreSQL
3. **Validate results** including schema detection, data retrieval, and type conversion

Integration tests include:
- `test_simple_table_integration()` - Basic data types (int, string, float, boolean)
- `test_vector_table_integration()` - Vector embeddings and list data types

## Running Tests

### All Tests (Unit + Integration)
```bash
just test
```

### All Quality Checks (Format + Lint + Test)
```bash
just ci
```

## Test Data Generation

The integration tests use `LanceTestDataGenerator` which creates temporary Lance datasets with:

- **Simple Table**: Basic scalar data types (ID, name, age, salary, status)
- **Vector Table**: Embeddings stored as Arrow List arrays with metadata

All test data is created in temporary directories and automatically cleaned up after tests complete.

## Key Features Tested

1. **Schema Detection** - `lance_table_info()` correctly identifies column types and nullability
2. **Table Statistics** - `lance_table_stats()` returns accurate row/column counts and version info
3. **Data Scanning** - `lance_scan_jsonb()` correctly converts Lance data to PostgreSQL JSONB
4. **Type Conversion** - Proper mapping between Arrow/Lance types and PostgreSQL types
5. **Error Handling** - Graceful handling of invalid file paths and malformed data
6. **Limit Support** - Correct behavior when limiting scan results

## Advantages of This Approach

✅ **No Docker Required** - Tests run directly with `cargo pgrx test`
✅ **Fast Execution** - No container startup overhead
✅ **Simple Setup** - Only requires Rust and pgrx installation
✅ **Unified Testing** - Unit and integration tests in one command
✅ **Comprehensive Coverage** - Tests the complete data pipeline from creation to retrieval
✅ **Easy Debugging** - Standard Rust debugging tools work seamlessly
✅ **CI/CD Friendly** - Runs in any environment with Rust support

## Previous Docker-Based Approach

The original testing approach used:
- Python scripts to create Lance datasets
- Docker containers for PostgreSQL + pglance
- Complex shell scripts for orchestration

This has been **completely replaced** by the pure Rust approach, eliminating external dependencies and providing a unified testing experience.

## Commands Summary

| Command | Description |
|---------|-------------|
| `just test` | Run all tests (unit + integration) |
| `just ci` | Run all quality checks (format + lint + test) |
| `just run` | Start PostgreSQL with pglance for manual testing |
| `just check` | Format + lint + test |

## Example Test Output

```
running 4 tests
test tests::pg_test_hello_pglance ... ok
test tests::pg_test_error_handling ... ok
test tests::pg_test_simple_table_integration ... ok
test tests::pg_test_vector_table_integration ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

This testing approach ensures reliable, fast, and maintainable tests for the pglance PostgreSQL extension.