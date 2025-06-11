# Docker Integration Test Environment

This directory contains Docker configuration for running pglance integration tests in an isolated PostgreSQL environment.

## Overview

The Docker setup provides:
- PostgreSQL 16 with pglance extension pre-installed
- Isolated test environment that doesn't interfere with local PostgreSQL
- Automatic extension initialization
- Volume mounting for Lance test data
- Health checks and proper container lifecycle management

## Files

- `Dockerfile` - Multi-stage build for PostgreSQL + pglance extension
- `docker-compose.yml` - Service orchestration for integration tests
- `init-pglance.sh` - Database initialization script
- `README.md` - This documentation

## Quick Start

From the `integration_tests` directory:

```bash
# Run full integration test suite (builds container, runs tests, cleans up)
./run_tests.sh

# Start just the database container
./run_tests.sh --start-db

# Stop the database container
./run_tests.sh --stop-db

# Clean up everything
./run_tests.sh --cleanup
```

## Manual Docker Operations

If you need to manage the Docker environment manually:

```bash
cd docker/

# Build the image
docker-compose build

# Start the container
docker-compose up -d

# Check container health
docker-compose ps

# View logs
docker-compose logs pglance-db

# Connect to PostgreSQL
docker-compose exec pglance-db psql -U postgres

# Stop and remove containers
docker-compose down
```

## Container Details

### Image Build Process

The Dockerfile:
1. Starts with `postgres:16-bookworm` base image
2. Installs Rust toolchain and cargo-pgrx
3. Installs system dependencies (build tools, libraries)
4. Copies pglance source code
5. Compiles and installs pglance extension
6. Sets up initialization script

### Container Configuration

- **Image Name**: Built locally from Dockerfile
- **Container Name**: `pglance-integration-test`
- **PostgreSQL Port**: 5432 (mapped to host)
- **Database**: `postgres`
- **User**: `postgres`
- **Password**: `postgres`

### Volume Mounts

- `../testdata:/test_data_in_container` - Lance test data directory

### Environment Variables

- `POSTGRES_DB=postgres`
- `POSTGRES_USER=postgres`
- `POSTGRES_PASSWORD=postgres`
- `POSTGRES_HOST_AUTH_METHOD=trust`

## Integration Test Flow

1. **Container Startup**:
   - Build pglance-enabled PostgreSQL image
   - Start container with health checks
   - Wait for PostgreSQL to be ready
   - Verify pglance extension is available

2. **Test Execution**:
   - Python test script connects to containerized PostgreSQL
   - Creates Lance tables in mounted `testdata` directory
   - Tests pglance functions against these tables
   - Validates data integrity and type conversion

3. **Cleanup**:
   - Stop and remove containers
   - Clean up test data
   - Remove temporary files

## Troubleshooting

### Container Won't Start

```bash
# Check Docker daemon
docker info

# View build logs
docker-compose build --no-cache

# Check container logs
docker-compose logs pglance-db
```

### PostgreSQL Connection Issues

```bash
# Verify container is running
docker-compose ps

# Check PostgreSQL is accepting connections
docker-compose exec pglance-db pg_isready -U postgres

# Test connection manually
docker-compose exec pglance-db psql -U postgres -c "SELECT version();"
```

### pglance Extension Issues

```bash
# Verify extension is installed
docker-compose exec pglance-db psql -U postgres -c "\dx"

# Test pglance functions
docker-compose exec pglance-db psql -U postgres -c "SELECT hello_pglance();"

# Check extension files
docker-compose exec pglance-db find /usr -name "*pglance*" 2>/dev/null
```

### Build Issues

Common problems and solutions:

1. **Rust compilation errors**: Check system dependencies in Dockerfile
2. **cargo-pgrx version mismatch**: Verify cargo-pgrx version compatibility
3. **PostgreSQL version mismatch**: Ensure pg_config points to correct version
4. **Permission issues**: Check file permissions and Docker daemon access

### Performance Issues

```bash
# Check container resources
docker stats pglance-integration-test

# Monitor disk usage
docker system df

# Clean up unused resources
docker system prune
```

## Development

### Modifying the Container

1. Edit `Dockerfile` for system-level changes
2. Edit `docker-compose.yml` for service configuration
3. Edit `init-pglance.sh` for database initialization
4. Rebuild with `docker-compose build --no-cache`

### Testing Changes

```bash
# Test build only
docker-compose build

# Test container startup
docker-compose up -d && docker-compose logs -f

# Test pglance installation
docker-compose exec pglance-db psql -U postgres -c "CREATE EXTENSION pglance; SELECT hello_pglance();"
```

### Adding Dependencies

Add to Dockerfile in the appropriate section:
- System packages: Add to `apt-get install` command
- Rust dependencies: Will be built from Cargo.toml
- PostgreSQL extensions: Add to installation section

## Security Considerations

This Docker setup is designed for **testing only**:

- Uses default PostgreSQL credentials
- Trusts all connections
- Runs containers with elevated privileges
- Exposes PostgreSQL port to host

**Do not use in production environments.**

## Resource Requirements

- **Disk Space**: ~2-3 GB for full build
- **Memory**: ~512 MB minimum for container
- **CPU**: Multi-core recommended for faster builds
- **Network**: Internet access required for downloading dependencies

## CI/CD Integration

The Docker setup is designed to work in CI environments:

```yaml
# Example GitHub Actions usage
- name: Run Integration Tests
  run: |
    cd integration_tests
    ./run_tests.sh
```

All operations return proper exit codes for CI integration.