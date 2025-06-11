#!/bin/bash

set -e

echo "🚀 Starting to build pglance extension..."

check_dependencies() {
    echo "📋 Checking dependencies..."
    
    if ! command -v cargo &> /dev/null; then
        echo "❌ Error: cargo not installed. Please install Rust toolchain."
        exit 1
    fi
    
    if ! command -v cargo-pgrx &> /dev/null; then
        echo "❌ Error: cargo-pgrx not installed. Please run: cargo install --locked cargo-pgrx@0.14.3"
        exit 1
    fi
    
    if ! command -v pg_config &> /dev/null; then
        echo "❌ Error: pg_config not found. Please ensure PostgreSQL development packages are installed."
        exit 1
    fi
    
    echo "✅ Dependencies check passed"
}

build_extension() {
    echo "🔨 Building extension..."
    
    cargo clean
    
    echo "📝 Checking code..."
    cargo check
    
    echo "📄 Generating SQL schema..."
    cargo pgrx schema
    
    echo "✅ Build completed"
}

run_tests() {
    echo "🧪 Running tests..."
    
    echo "🦀 Running Rust unit tests..."
    cargo test
    
    echo "🐘 Running PostgreSQL integration tests..."
    cargo pgrx test pg13
    
    echo "✅ Tests completed"
}

install_extension() {
    if [[ "${1:-}" == "--install" ]]; then
        echo "📦 Installing extension to local PostgreSQL..."
        cargo pgrx install
        echo "✅ Installation completed"
        
        echo "💡 Now you can run in PostgreSQL:"
        echo "   CREATE EXTENSION pglance;"
    fi
}

show_usage() {
    echo ""
    echo "📚 Usage Instructions:"
    echo "   ./build_and_test.sh          # Build and test"
    echo "   ./build_and_test.sh --install # Build, test and install"
    echo ""
    echo "🔧 Available pglance functions:"
    echo "   - hello_pglance()                           # Test function"
    echo "   - lance_table_info(table_path)              # Get table structure"
    echo "   - lance_table_stats(table_path)             # Get table statistics"
    echo "   - lance_scan_jsonb(table_path, limit)       # Scan table data"
    echo ""
    echo "📝 Example usage:"
    echo "   SELECT hello_pglance();"
    echo "   SELECT * FROM lance_table_info('/path/to/your/lance/table');"
    echo "   SELECT * FROM lance_scan_jsonb('/path/to/your/lance/table', 10);"
    echo ""
}

create_demo_data() {
    if [[ "${1:-}" == "--demo" ]]; then
        echo "🎭 Creating demo data..."
        
        echo "💡 To create demo data, you need:"
        echo "   1. Install Lance Python package: pip install pylance"
        echo "   2. Create a simple Lance table"
        echo "   3. Use pglance functions to access the table"
        
        cat > demo_data.py << 'EOF'
import pyarrow as pa
import lance

table = pa.table({
    "id": [1, 2, 3, 4, 5],
    "name": ["Alice", "Bob", "Charlie", "David", "Eve"],
    "age": [25, 30, 35, 40, 45],
    "score": [85.5, 92.0, 78.5, 88.0, 95.5]
})

lance.write_dataset(table, "/tmp/demo_table.lance")
print("Demo table created at /tmp/demo_table.lance")
EOF
        
        echo "💡 Run 'python demo_data.py' to create demo table"
    fi
}

main() {
    echo "=========================================="
    echo "🔧 pglance PostgreSQL Lance Extension Builder"
    echo "=========================================="
    
    check_dependencies
    build_extension
    run_tests
    install_extension "$@"
    create_demo_data "$@"
    show_usage
    
    echo ""
    echo "🎉 Done! pglance extension is ready."
    echo "=========================================="
}

main "$@"