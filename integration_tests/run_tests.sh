#!/bin/bash

set -e

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DOCKER_DIR="$SCRIPT_DIR/docker"

echo "🧪 Starting pglance integration tests..."
echo "📁 Script directory: $SCRIPT_DIR"
echo "📁 Project root: $PROJECT_ROOT"
echo "🐳 Docker directory: $DOCKER_DIR"

check_dependencies() {
    echo "📋 Checking dependencies..."

    if ! command -v uv &> /dev/null; then
        echo "❌ Error: uv not installed. Please install uv: https://docs.astral.sh/uv/getting-started/installation/"
        exit 1
    fi

    if ! command -v docker &> /dev/null; then
        echo "❌ Error: docker not found. Please ensure Docker is installed and running."
        exit 1
    fi

    if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
        echo "❌ Error: docker-compose not found. Please ensure Docker Compose is installed."
        exit 1
    fi

    echo "✅ Dependencies check passed"
}

setup_python_env() {
    echo "🐍 Setting up Python environment with uv..."

    cd "$SCRIPT_DIR"
    echo "Syncing dependencies with uv..."
    uv sync

    echo "✅ Python environment ready"
}

start_test_database() {
    echo "🐳 Starting PostgreSQL test container..."

    cd "$DOCKER_DIR"

    # Stop any existing containers
    if docker ps -q --filter "name=pglance-integration-test" | grep -q .; then
        echo "🛑 Stopping existing test container..."
        docker-compose down
    fi

    # Build and start the container
    echo "🔨 Building pglance Docker image..."
    docker-compose build

    echo "🚀 Starting PostgreSQL container with pglance extension..."
    docker-compose up -d

    # Wait for PostgreSQL to be ready
    echo "⏳ Waiting for PostgreSQL to be ready..."
    local max_attempts=30
    local attempt=1

    while [ $attempt -le $max_attempts ]; do
        if docker-compose exec -T pglance-db pg_isready -U postgres &> /dev/null; then
            echo "✅ PostgreSQL is ready!"
            break
        fi

        if [ $attempt -eq $max_attempts ]; then
            echo "❌ PostgreSQL failed to start within expected time"
            docker-compose logs pglance-db
            exit 1
        fi

        echo "⏳ Attempt $attempt/$max_attempts - waiting for PostgreSQL..."
        sleep 2
        ((attempt++))
    done

    # Verify pglance extension is available
    echo "🔍 Verifying pglance extension..."
    if docker-compose exec -T pglance-db psql -U postgres -c "SELECT hello_pglance();" &> /dev/null; then
        echo "✅ pglance extension is working!"
    else
        echo "❌ pglance extension verification failed"
        docker-compose logs pglance-db
        exit 1
    fi

    cd "$SCRIPT_DIR"
}

stop_test_database() {
    echo "🛑 Stopping PostgreSQL test container..."

    cd "$DOCKER_DIR"
    docker-compose down
    cd "$SCRIPT_DIR"

    echo "✅ Test container stopped"
}

run_integration_tests() {
    echo "🔄 Running integration tests..."

    cd "$SCRIPT_DIR"

    if [ ! -f "integration_test.py" ]; then
        echo "❌ Error: integration_test.py not found"
        exit 1
    fi

    echo "Starting integration test..."
    TESTDATA_ABSOLUTE_PATH="$(realpath ./testdata)"
    uv run python integration_test.py \
        --host-data-dir ./testdata \
        --pglance-data-prefix /test_data_in_container \
        --db-host localhost \
        --db-port 5432 \
        --db-name postgres \
        --db-user postgres \
        --db-password postgres \
        --cleanup

    echo "✅ Integration tests completed"
}

cleanup() {
    echo "🧹 Cleaning up test environment..."

    cd "$SCRIPT_DIR"

    # Stop Docker containers
    stop_test_database

    # Clean up test data
    if [ -d "testdata" ]; then
        find testdata -name "*.lance" -type d -exec rm -rf {} + 2>/dev/null || true
        echo "✅ Test data cleaned up"
    fi

    # Clean up Python environment if needed
    if [ -d ".venv" ]; then
        echo "🧹 Cleaning up Python virtual environment..."
        rm -rf .venv
    fi
}

show_usage() {
    echo ""
    echo "📚 Usage Instructions:"
    echo "   ./run_tests.sh                    # Run integration tests with Docker"
    echo "   ./run_tests.sh --cleanup         # Clean up test environment and exit"
    echo "   ./run_tests.sh --start-db        # Start test database only"
    echo "   ./run_tests.sh --stop-db         # Stop test database only"
    echo ""
    echo "🔧 Test Components:"
    echo "   - Docker PostgreSQL container with pglance extension"
    echo "   - Integration tests (integration_test.py)"
    echo ""
    echo "📝 Prerequisites:"
    echo "   - Docker and Docker Compose installed"
    echo "   - uv package manager installed"
    echo "   - Sufficient disk space for Docker images"
    echo ""
}

main() {
    echo "=========================================="
    echo "🧪 pglance Integration Test Runner"
    echo "=========================================="

    case "${1:-}" in
        --cleanup)
            cleanup
            exit 0
            ;;
        --start-db)
            check_dependencies
            start_test_database
            echo "💡 Database is running. Connect with: psql -h localhost -U postgres -d postgres"
            exit 0
            ;;
        --stop-db)
            stop_test_database
            exit 0
            ;;
        --help|-h)
            show_usage
            exit 0
            ;;
        *)
            check_dependencies
            setup_python_env
            start_test_database

            # Ensure cleanup happens even if tests fail
            trap cleanup EXIT

            run_integration_tests
            ;;
    esac

    echo ""
    echo "🎉 Integration tests completed!"
    echo "=========================================="
    show_usage
}

main "$@"
