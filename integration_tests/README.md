# pglance Integration Tests

This directory contains integration tests for the pglance PostgreSQL extension that verify the interaction between PostgreSQL, the pglance extension, and Lance data format.

## Overview

The integration tests validate the complete workflow of:
- Creating Lance tables with various data types using PyLance
- Reading those tables through the pglance PostgreSQL extension
- Verifying data integrity and type conversion between Lance and PostgreSQL

## Quick Start

### Prerequisites

1. **pglance extension must be built and installed**:
   ```bash
   cd ..
   ./build_and_test.sh --install
   ```

2. **PostgreSQL server must be running** and accessible via `psql`

3. **Python 3.8+** with pip or uv available

### Running Integration Tests

```bash
./run_tests.sh
```

This will:
- Set up Python virtual environment
- Install test dependencies
- Run integration tests
- Clean up test data

### Test Options

```bash
# Clean up test data only
./run_tests.sh --cleanup
```

## Integration Test Components

### Main Integration Test (`integration_test.py`)

Comprehensive Python-based integration tests that:

1. **Create various Lance tables** using PyLance with different data types:
   - Simple table (basic data types: int, string, float, boolean, date)
   - Vector table (embeddings and metadata)
   - Complex table (nested data, JSON, timestamps)
   - Large table (performance testing with 1000+ rows)

2. **Test pglance extension functionality**:
   - `lance_table_info()` - Verify table schema detection
   - `lance_table_stats()` - Validate table statistics
   - `lance_scan_jsonb()` - Test data retrieval and conversion

3. **Validate data integrity** between Lance and PostgreSQL representations

**Dependencies**: `pylance`, `pyarrow`, `psycopg2-binary`

**Usage**:
```bash
python integration_test.py [--cleanup] [--host-data-dir ./testdata]
```

### Test Data Directory (`testdata/`)

Directory where test Lance tables are created and stored during test execution. This directory is automatically cleaned up after tests complete (when using `--cleanup` flag).

## Configuration

### Python Dependencies

The `pyproject.toml` file defines the Python test environment:

```toml
[project]
name = "pglance-tests"
dependencies = [
    "pylance",
    "pyarrow", 
    "psycopg2-binary",
]
```

### Database Configuration

Integration tests support various database connection parameters:

```bash
python integration_test.py \
  --db-host localhost \
  --db-port 5432 \
  --db-name postgres \
  --db-user postgres \
  --db-password postgres
```

### Container Testing

For Docker/container environments, specify different paths for host and container:

```bash
python integration_test.py \
  --host-data-dir ./testdata \
  --pglance-data-prefix /test_data_in_container
```

## Development

### Adding New Integration Tests

1. **Extend the LanceTableGenerator class** to create new test data scenarios
2. **Add test cases to PglanceIntegrationTest class** for new pglance functionality
3. **Update test configurations** in the `run_comprehensive_test()` method

### Test Data Management

- Test Lance tables are created in `testdata/` during test execution
- Use `--cleanup` flag to automatically remove test data after completion
- For persistent test data, omit the `--cleanup` flag

## Troubleshooting

### Common Issues

1. **Extension not found**: Ensure pglance is built and installed (`../build_and_test.sh --install`)
2. **PostgreSQL connection failed**: Check that PostgreSQL is running and accessible
3. **Python dependencies missing**: Run the test script which will set up the virtual environment
4. **Permission denied**: Ensure test script is executable (`chmod +x run_tests.sh`)

### Debugging

For detailed debugging output, check:
- PostgreSQL logs for extension loading issues
- Python test output for detailed error messages
- Test data in `testdata/` directory for Lance table inspection

## Integration with CI/CD

The test runner script is designed to work in CI/CD environments:

```bash
# In CI pipeline
./integration_tests/run_tests.sh
```

All tests return appropriate exit codes for CI/CD integration.

## Test Coverage

The integration tests cover:

- **Data Type Support**: Various PostgreSQL and Lance data types
- **Schema Detection**: Table structure and column information
- **Data Retrieval**: Reading Lance data through PostgreSQL
- **Performance**: Large dataset handling
- **Error Handling**: Invalid paths and malformed data
- **Type Conversion**: Lance to PostgreSQL type mapping