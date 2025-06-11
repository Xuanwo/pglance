#!/bin/bash

set -e

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🧪 Starting pglance integration tests..."
echo "📁 Script directory: $SCRIPT_DIR"
echo "📁 Project root: $PROJECT_ROOT"

check_dependencies() {
    echo "📋 Checking dependencies..."

    if ! command -v uv &> /dev/null; then
        echo "❌ Error: uv not installed. Please install uv: https://docs.astral.sh/uv/getting-started/installation/"
        exit 1
    fi

    if ! command -v psql &> /dev/null; then
        echo "❌ Error: psql not found. Please ensure PostgreSQL client is installed."
        exit 1
    fi

    if ! command -v pg_config &> /dev/null; then
        echo "❌ Error: pg_config not found. Please ensure PostgreSQL development packages are installed."
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

run_integration_tests() {
    echo "🔄 Running integration tests..."

    cd "$SCRIPT_DIR"

    if [ ! -f "integration_test.py" ]; then
        echo "❌ Error: integration_test.py not found"
        exit 1
    fi

    echo "Starting integration test..."
    TESTDATA_ABSOLUTE_PATH="$(realpath ./testdata)"
    uv run python integration_test.py --host-data-dir ./testdata --pglance-data-prefix "$TESTDATA_ABSOLUTE_PATH" --cleanup

    echo "✅ Integration tests completed"
}

cleanup() {
    echo "🧹 Cleaning up test data..."

    cd "$SCRIPT_DIR"

    if [ -d "testdata" ]; then
        find testdata -name "*.lance" -type d -exec rm -rf {} + 2>/dev/null || true
        echo "✅ Test data cleaned up"
    fi
}

show_usage() {
    echo ""
    echo "📚 Usage Instructions:"
    echo "   ./run_tests.sh                    # Run integration tests"
    echo "   ./run_tests.sh --cleanup         # Clean up test data and exit"
    echo ""
    echo "🔧 Test Components:"
    echo "   - Integration tests (integration_test.py)"
    echo ""
    echo "📝 Prerequisites:"
    echo "   - pglance extension must be built and installed"
    echo "   - PostgreSQL server must be running"
    echo "   - uv package manager installed"
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
        --help|-h)
            show_usage
            exit 0
            ;;
        *)
            check_dependencies
            setup_python_env
            run_integration_tests
            ;;
    esac

    echo ""
    echo "🎉 Integration tests completed!"
    echo "=========================================="
    show_usage
}

main "$@"
