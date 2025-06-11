# pglance - PostgreSQL Lance Table Extension

pglance is a PostgreSQL extension built with the [pgrx](https://github.com/pgcentralfoundation/pgrx) framework that implements basic full-table scanning functionality for directly reading and querying [Lance](https://lancedb.github.io/lance/) format tables within PostgreSQL.

This is the first open-source project to seamlessly integrate the modern columnar storage format Lance with PostgreSQL database.

## 🎯 Project Goals

Bring Lance's high-performance columnar storage and vector search capabilities into the PostgreSQL ecosystem, providing users with:
- Efficient large-scale data analytics capabilities
- Native vector search support (planned)
- Unified SQL interface for accessing Lance data

## ✨ Core Features

### Implemented (v0.1)
- **🔍 Lance Table Scanning**: Complete table data reading and traversal
- **📊 Schema Inspection**: Automatic parsing of Lance table structure and column types
- **📈 Statistics**: Get table metadata including version, row count, column count
- **🔄 Type Conversion**: Intelligent type mapping from Arrow/Lance to PostgreSQL
- **📦 JSONB Output**: JSON serialization for complex data structures
- **⚡ Async Processing**: Integration of async Lance APIs within sync PostgreSQL interface

### Planned (v0.2+)
- **🎯 Vector Search**: KNN and ANN search support
- **🔧 FDW Support**: Foreign Data Wrapper interface
- **✏️ Write Operations**: INSERT/UPDATE/DELETE support
- **🚀 Query Optimization**: Predicate pushdown and column projection optimization

## 🛠️ Tech Stack

| Component | Version | Description |
|-----------|---------|-------------|
| PostgreSQL | 13-17 | Support for all actively maintained versions |
| Rust | 1.70+ | Modern systems programming language |
| pgrx | 0.14.3 | PostgreSQL extension development framework |
| Lance | 0.29 | Latest version of Lance storage engine |
| Arrow | 55.1 | Latest version of Apache Arrow |

## 🚀 Quick Start

### Method 1: Using Build Script (Recommended)

```bash
# Clone the project
git clone <repository-url>
cd pglance

# Run one-click build script
chmod +x build_and_test.sh
./build_and_test.sh --install

# Enable in PostgreSQL
psql -c "CREATE EXTENSION pglance;"
```

### Method 2: Manual Build

```bash
# 1. Install pgrx
cargo install --locked cargo-pgrx@0.14.3
cargo pgrx init --pg13 /usr/bin/pg_config

# 2. Build project
git clone <repository-url>
cd pglance
cargo pgrx install

# 3. Enable extension
psql -c "CREATE EXTENSION pglance;"
```

### Verify Installation

```sql
-- Test basic functionality
SELECT hello_pglance();
-- Should return: "Hello, pglance"
```

## 📖 Usage Guide

### 🔍 Table Structure Exploration

```sql
-- View complete Lance table structure information
SELECT 
    column_name,
    data_type,
    CASE WHEN nullable THEN 'YES' ELSE 'NO' END as is_nullable
FROM lance_table_info('/path/to/your/lance/table')
ORDER BY column_name;
```

**Example Output:**
```
 column_name | data_type | is_nullable 
-------------+-----------+-------------
 id          | int8      | NO
 embedding   | float4[]  | YES  
 metadata    | jsonb     | YES
 name        | text      | YES
```

### 📊 Data Statistics Analysis

```sql
-- Get detailed table statistics
SELECT 
    'Lance Table Version: ' || version as info,
    'Total Rows: ' || num_rows as row_info,
    'Total Columns: ' || num_columns as col_info
FROM lance_table_stats('/path/to/your/lance/table');
```

### 📋 Data Content Viewing

```sql
-- View first 5 rows of data (recommended for large tables)
SELECT 
    (row_data->>'id')::bigint as id,
    row_data->>'name' as name,
    jsonb_array_length(row_data->'embedding') as embedding_dim
FROM lance_scan_jsonb('/path/to/your/lance/table', 5);

-- Data quality statistics
SELECT 
    COUNT(*) as total_rows,
    COUNT(CASE WHEN row_data ? 'id' THEN 1 END) as has_id,
    COUNT(CASE WHEN row_data ? 'embedding' THEN 1 END) as has_embedding
FROM lance_scan_jsonb('/path/to/your/lance/table', 1000);
```

## API Reference

### `lance_table_info(table_path TEXT)`

Returns Lance table structure information.

**Parameters:**
- `table_path`: File system path to the Lance table

**Returns:**
- `column_name`: Column name
- `data_type`: PostgreSQL data type
- `nullable`: Whether null values are allowed

### `lance_table_stats(table_path TEXT)`

Returns Lance table statistics.

**Parameters:**
- `table_path`: File system path to the Lance table

**Returns:**
- `version`: Lance table version
- `num_rows`: Total number of rows
- `num_columns`: Total number of columns

### `lance_scan_jsonb(table_path TEXT, limit INTEGER DEFAULT NULL)`

Scans Lance table and returns data in JSONB format.

**Parameters:**
- `table_path`: File system path to the Lance table
- `limit`: Limit number of rows returned (optional)

**Returns:**
- `row_data`: Row data in JSONB format

## Data Type Mapping

| Arrow/Lance Type | PostgreSQL Type |
|------------------|-----------------|
| Boolean          | boolean         |
| Int8             | char            |
| Int16            | int2            |
| Int32            | int4            |
| Int64            | int8            |
| Float32          | float4          |
| Float64          | float8          |
| Utf8/LargeUtf8   | text            |
| Binary           | bytea           |
| Date32/Date64    | date            |
| Timestamp        | timestamp       |
| List/Struct      | jsonb           |
| FixedSizeList(float) | float4[]/float8[] |

## Development

### Prerequisites

Install required tools:
- **Rust** (latest stable) - https://rustup.rs/
- **uv** (Python package manager) - https://docs.astral.sh/uv/getting-started/installation/
- **just** (command runner) - https://github.com/casey/just#installation
- **PostgreSQL** (13-17) with development headers

### Quick Development Setup

```bash
# Setup development environment
just setup

# Run all quality checks
just check

# Auto-format code
just fmt

# Build extension
just build

# Run tests
just test

# Start PostgreSQL with extension
just run
```

### Available Commands

```bash
just                    # Show all available commands
just check              # Run all quality checks
just fmt                # Auto-format all code
just build              # Build extension
just test               # Run Rust tests
just e2e                # Run integration tests
just run                # Start PostgreSQL with extension
just ci                 # Simulate CI locally
```

You can specify PostgreSQL version:
```bash
just build pg=15        # Build for PostgreSQL 15
just test pg=17         # Test with PostgreSQL 17
```

### Manual Commands (if needed)

```bash
# Unit tests
cargo test --features pg16

# PostgreSQL regression tests
cargo pgrx test --features pg16

# Integration tests
cd integration_tests && ./run_tests.sh
```

For detailed development information, see [DEVELOPMENT.md](DEVELOPMENT.md).

### Debugging

```bash
# Compile check
cargo check --features pg16

# Verbose logging
RUST_LOG=debug just run
```

## Architecture Design

```
pglance/
├── src/
│   ├── lib.rs              # Main entry, PostgreSQL function definitions
│   ├── types/              # Type conversion module
│   │   ├── conversion.rs   # Arrow to PostgreSQL type mapping
│   │   └── arrow_convert.rs # Arrow value conversion
│   └── scanner/            # Lance scanner
│       └── lance_scanner.rs # Lance table scanning implementation
├── sql/                    # SQL scripts
├── integration_tests/      # Integration and end-to-end tests
│   ├── end_to_end_test.py  # Python-based comprehensive tests
│   ├── demo.sql           # Demo script
│   ├── pg_regress/        # PostgreSQL regression tests
│   ├── testdata/          # Test data directory
│   ├── run_tests.sh       # Test runner script
│   └── pyproject.toml     # Python test dependencies
└── Cargo.toml             # Rust dependency configuration
```

## Limitations and Notes

1. **File Paths**: Currently requires full file system path to Lance tables
2. **Permissions**: PostgreSQL process needs read permissions for Lance files
3. **Memory Usage**: Large table scans may consume significant memory
4. **Type Support**: Complex nested types are converted to JSONB
5. **Concurrency**: Current implementation uses synchronous access, heavy concurrency may impact performance

## Future Plans

- [ ] Foreign Data Wrapper (FDW) support
- [ ] Vector search functionality (KNN)
- [ ] Write support (INSERT/UPDATE/DELETE)
- [ ] Partitioned table support
- [ ] Query pushdown optimization
- [ ] Streaming scans
- [ ] Custom vector types
- [ ] Index creation and management

## Contributing

Issues and Pull Requests are welcome!

## License

Apache License 2.0

## Related Projects

- [Lance](https://github.com/lancedb/lance) - Modern columnar data format
- [pgrx](https://github.com/pgcentralfoundation/pgrx) - PostgreSQL extension development framework
- [Apache Arrow](https://arrow.apache.org/) - In-memory columnar data format