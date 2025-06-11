# pglance - PostgreSQL Lance Table Extension

pglance is a PostgreSQL extension built with the [pgrx](https://github.com/pgcentralfoundation/pgrx) framework that implements full-table scanning functionality for directly reading and querying [Lance](https://lancedb.github.io/lance/) format tables within PostgreSQL.

This is the first open-source project to seamlessly integrate the modern columnar storage format Lance with PostgreSQL database.

## 🎯 Project Goals

Bring Lance's high-performance columnar storage and vector search capabilities into the PostgreSQL ecosystem, providing users with:
- Efficient large-scale data analytics capabilities
- Native vector search support (planned)
- Unified SQL interface for accessing Lance data

## ✨ Core Features

### Current Implementation
- **🔍 Lance Table Scanning**: Complete table data reading and traversal
- **📊 Schema Inspection**: Automatic parsing of Lance table structure and column types
- **📈 Statistics**: Get table metadata including version, row count, column count
- **🔄 Type Conversion**: Intelligent type mapping from Arrow/Lance to PostgreSQL
- **📦 JSONB Output**: JSON serialization for complex data structures
- **⚡ Async Processing**: Integration of async Lance APIs within sync PostgreSQL interface

### Planned Features
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

### Prerequisites

Install required tools:
- **Rust** (latest stable) - https://rustup.rs/
- **PostgreSQL** (13-17) with development headers
- **Protocol Buffers compiler** (protoc)

### Installation

```bash
# Clone the project
git clone <repository-url>
cd pglance

# Setup development environment
cargo install cargo-pgrx --version=0.14.3 --locked
cargo pgrx init

# Build and install extension
cargo pgrx install --features pg16

# Enable extension in PostgreSQL
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

## 📚 API Reference

### `hello_pglance()`

Returns a simple greeting to verify extension installation.

**Returns:** `TEXT` - "Hello, pglance"

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

## 🔄 Data Type Mapping

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

## 🛠️ Development

### Quick Development Setup

```bash
# Setup development environment
cargo install cargo-pgrx --version=0.14.3 --locked
cargo pgrx init

# Clone and setup project
git clone <repository-url>
cd pglance

# Run all quality checks
cargo fmt --all -- --check
cargo clippy --features pg16 -- -D warnings
cargo test --features pg16

# Build and install
cargo pgrx install --features pg16

# Start PostgreSQL with extension
cargo pgrx run --features pg16
```

### Using Just Commands

If you have [just](https://github.com/casey/just) installed:

```bash
# Show all available commands
just

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

# Simulate CI locally
just ci
```

### Supported PostgreSQL Versions

Specify PostgreSQL version for commands:
```bash
cargo pgrx install --features pg15  # PostgreSQL 15
cargo pgrx install --features pg17  # PostgreSQL 17
# Or with just:
just build pg=15
just test pg=17
```

Supported versions: 13, 14, 15, 16, 17 (default: 16)

For detailed development information, see [DEVELOPMENT.md](DEVELOPMENT.md).

## 🧪 Testing

pglance uses a pure Rust testing approach with comprehensive unit and integration tests.

```bash
# Run all tests
cargo test --features pg16
# Or with just:
just test
```

All tests are written in Rust using the pgrx testing framework. For detailed testing information, see [TESTING.md](TESTING.md).

## 🏗️ Architecture

```
pglance/
├── src/
│   ├── lib.rs              # Main entry, PostgreSQL function definitions
│   ├── types/              # Type conversion module
│   │   ├── mod.rs          # Module exports
│   │   ├── conversion.rs   # Arrow to PostgreSQL type mapping
│   │   └── arrow_convert.rs # Arrow value conversion utilities
│   └── scanner/            # Lance scanner implementation
│       ├── mod.rs          # Module exports
│       └── lance_scanner.rs # Lance table scanning logic
├── sql/                    # SQL scripts (if any)
├── .github/                # GitHub workflows
│   └── workflows/
│       ├── rust-checks.yml # CI/CD pipeline
│       └── release.yml     # Release automation
├── Cargo.toml             # Rust dependency configuration
├── justfile               # Development commands
├── pglance.control        # PostgreSQL extension metadata
├── README.md              # This file
├── DEVELOPMENT.md         # Development guide
└── TESTING.md             # Testing guide
```

## ⚠️ Limitations and Notes

1. **File Paths**: Currently requires full file system path to Lance tables
2. **Permissions**: PostgreSQL process needs read permissions for Lance files
3. **Memory Usage**: Large table scans may consume significant memory
4. **Type Support**: Complex nested types are converted to JSONB
5. **Concurrency**: Current implementation uses synchronous access

## 🔮 Future Plans

- [ ] Foreign Data Wrapper (FDW) support
- [ ] Vector search functionality (KNN/ANN)
- [ ] Write support (INSERT/UPDATE/DELETE)
- [ ] Partitioned table support
- [ ] Query pushdown optimization
- [ ] Streaming scans for large datasets
- [ ] Custom vector types
- [ ] Index creation and management

## 🤝 Contributing

Issues and Pull Requests are welcome! Please see our development guidelines in [DEVELOPMENT.md](DEVELOPMENT.m
d).

## 📄 License

Apache License 2.0

## 🔗 Related Projects

- [Lance](https://github.com/lancedb/lance) - Modern columnar data
 format
- [pgrx](https://github.com/pgcentralfoundation/pgrx) - PostgreSQL extension development framework
- [Apache Arrow](https://arrow.apache.org/) - In-memory columnar data format
- [LanceDB](https://lancedb.github.io/lancedb/) - Vector database built on Lance