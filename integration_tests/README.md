# pglance Integration Tests

This directory contains integration tests for the pglance PostgreSQL extension that verify the interaction between PostgreSQL, the pglance extension, and Lance data format.

## Overview

The integration tests validate the complete workflow of:
- Creating Lance tables with various data types using PyLance
- Reading those tables through the pglance PostgreSQL extension
- Verifying data integrity and type conversion between Lance and PostgreSQL

## Quick Start

### Prerequisites

1. **Docker and Docker Compose** installed and running

2. **Python 3.8+** with uv package manager available

3. **Sufficient disk space** (~2-3 GB for Docker images)

### Running Integration Tests

```bash
./run_tests.sh
```

This will:
- Build PostgreSQL + pglance Docker container
- Start isolated test database
- Set up Python virtual environment
- Install test dependencies
- Run integration tests
- Clean up test data and containers

### Test Options

```bash
# Start test database only
./run_tests.sh --start-db

# Stop test database only
./run_tests.sh --stop-db

# Clean up test environment and exit
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
# With Docker (recommended)
./run_tests.sh

# Manual execution (requires running PostgreSQL with pglance)
python integration_test.py [--cleanup] [--host-data-dir ./testdata]
```

### Test Data Directory (`testdata/`)

Directory where test Lance tables are created and stored during test execution. This directory is mounted into the Docker container at `/test_data_in_container` and is automatically cleaned up after tests complete (when using `--cleanup` flag).

### Docker Environment (`docker/`)

Contains Docker configuration for the integration test environment:
- `Dockerfile` - PostgreSQL 16 + pglance extension image
- `docker-compose.yml` - Service orchestration
- `init-pglance.sh` - Database initialization script
- `README.md` - Docker-specific documentation

The Docker setup provides an isolated PostgreSQL instance with pglance pre-installed, eliminating the need for local PostgreSQL installation.

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

#### Docker Environment (Default)
The Docker setup automatically configures:
- Host: `localhost`
- Port: `5432`
- Database: `postgres`
- User: `postgres`
- Password: `postgres`

#### Manual Configuration
For custom PostgreSQL instances:

```bash
python integration_test.py \
  --db-host your-host \
  --db-port 5432 \
  --db-name your-db \
  --db-user your-user \
  --db-password your-password
```

### Container Testing

The Docker environment automatically handles path mapping:
- Host path: `./testdata`
- Container path: `/test_data_in_container`

For custom container setups:

```bash
python integration_test.py \
  --host-data-dir ./testdata \
  --pglance-data-prefix /your-container-path
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

1. **Docker not running**: Ensure Docker daemon is started (`docker info`)
2. **Container build fails**: Check Docker logs and ensure sufficient disk space
3. **PostgreSQL connection failed**: Verify container is healthy (`docker-compose ps`)
4. **Python dependencies missing**: Run `./run_tests.sh` which sets up the environment
5. **Permission denied**: Ensure test script is executable (`chmod +x run_tests.sh`)
6. **Port 5432 in use**: Stop local PostgreSQL or change Docker port mapping

### Debugging

For detailed debugging output, check:
- Docker container logs: `cd docker && docker-compose logs pglance-db`
- PostgreSQL logs for extension loading issues
- Python test output for detailed error messages
- Test data in `testdata/` directory for Lance table inspection
- Container health: `cd docker && docker-compose ps`

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
