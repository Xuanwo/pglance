#!/usr/bin/env python3
"""
End-to-End Test for pglance Extension
=====================================

This script creates a comprehensive test that:
1. Creates various Lance tables using PyLance with different data types
2. Tests pglance extension functionality to read those tables
3. Validates data integrity and type conversion

Requirements:
- pylance (pip install pylance)
- pyarrow (pip install pyarrow)
- psycopg2 (pip install psycopg2-binary)
- PostgreSQL with pglance extension installed

Usage:
    python end_to_end_test.py [--cleanup]
"""

import os
import sys
import json
from datetime import datetime, date
from typing import Dict, List, Any, Optional
import argparse

try:
    import pyarrow as pa
    import lance
    import psycopg2
    from psycopg2.extras import RealDictCursor
except ImportError as e:
    print(f"Error: Missing required package: {e}")
    print("Please install: pip install pylance pyarrow psycopg2-binary")
    sys.exit(1)


class LanceTableGenerator:
    """Generates various types of Lance tables for testing"""

    def __init__(self, base_path: str):
        self.base_path = base_path
        os.makedirs(self.base_path, exist_ok=True)
        print(f"ℹ️ Lance tables will be generated in: {os.path.abspath(self.base_path)}")

    def create_simple_table(self) -> str:
        """Create a simple table with basic data types"""
        table_name = "simple_table"
        table_path = os.path.join(self.base_path, table_name)

        # Create sample data with various basic types
        data = {
            "id": [1, 2, 3, 4, 5],
            "name": ["Alice", "Bob", "Charlie", "David", "Eve"],
            "age": [25, 30, 35, 40, 45],
            "salary": [50000.5, 65000.0, 80000.25, 95000.75, 120000.0],
            "is_active": [True, True, False, True, False],
            "hire_date": [
                date(2020, 1, 15),
                date(2019, 6, 20),
                date(2021, 3, 10),
                date(2018, 9, 5),
                date(2022, 11, 30)
            ]
        }

        table = pa.table(data)
        lance.write_dataset(table, table_path, mode="overwrite")

        print(f"✅ Created simple table at: {table_path}")
        return table_name # Return relative name

    def create_vector_table(self) -> str:
        """Create a table with vector embeddings"""
        table_name = "vector_table"
        table_path = os.path.join(self.base_path, table_name)

        # Create sample data with vector embeddings
        embeddings = [
            [0.1, 0.2, 0.3, 0.4],
            [0.5, 0.6, 0.7, 0.8],
            [0.9, 1.0, 1.1, 1.2],
            [1.3, 1.4, 1.5, 1.6],
            [1.7, 1.8, 1.9, 2.0]
        ]

        data = {
            "id": [1, 2, 3, 4, 5],
            "document": ["doc1", "doc2", "doc3", "doc4", "doc5"],
            "embedding": embeddings,
            "metadata": [
                {"category": "A", "score": 0.95},
                {"category": "B", "score": 0.87},
                {"category": "A", "score": 0.92},
                {"category": "C", "score": 0.78},
                {"category": "B", "score": 0.89}
            ]
        }

        # Convert metadata to JSON strings for better compatibility
        data["metadata"] = [json.dumps(meta) for meta in data["metadata"]]

        table = pa.table(data)
        lance.write_dataset(table, table_path, mode="overwrite")

        print(f"✅ Created vector table at: {table_path}")
        return table_name # Return relative name

    def create_complex_table(self) -> str:
        """Create a table with complex nested data types"""
        table_name = "complex_table"
        table_path = os.path.join(self.base_path, table_name)

        data = {
            "id": [1, 2, 3],
            "user_name": ["user1", "user2", "user3"],
            "scores": [
                [85, 90, 78],
                [92, 88, 95],
                [76, 82, 89]
            ],
            "profile": [
                {"name": "John", "age": 30, "city": "NYC"},
                {"name": "Jane", "age": 25, "city": "LA"},
                {"name": "Bob", "age": 35, "city": "Chicago"}
            ],
            "tags": [
                ["python", "data", "ml"],
                ["javascript", "web", "react"],
                ["rust", "systems", "performance"]
            ],
            "created_at": [
                datetime(2023, 1, 1, 10, 30, 0),
                datetime(2023, 2, 15, 14, 45, 0),
                datetime(2023, 3, 20, 9, 15, 0)
            ]
        }

        # Convert complex objects to JSON strings
        data["profile"] = [json.dumps(profile) for profile in data["profile"]]

        table = pa.table(data)
        lance.write_dataset(table, table_path, mode="overwrite")

        print(f"✅ Created complex table at: {table_path}")
        return table_name # Return relative name

    def create_large_table(self) -> str:
        """Create a larger table for performance testing"""
        table_name = "large_table"
        table_path = os.path.join(self.base_path, table_name)

        # Generate 1000 rows of data
        size = 1000
        data = {
            "id": list(range(1, size + 1)),
            "value": [i * 0.1 for i in range(size)],
            "category": [f"cat_{i % 10}" for i in range(size)],
            "flag": [i % 2 == 0 for i in range(size)]
        }

        table = pa.table(data)
        lance.write_dataset(table, table_path, mode="overwrite")

        print(f"✅ Created large table with {size} rows at: {table_path}")
        return table_name # Return relative name


class PglanceTest:
    """Tests pglance extension functionality"""

    def __init__(self, db_params: Dict[str, Any], host_data_dir: str, pglance_data_prefix: str):
        self.db_params = db_params
        self.host_data_dir = os.path.abspath(host_data_dir)
        self.pglance_data_prefix = pglance_data_prefix
        self.conn = None

    def connect(self):
        """Connect to PostgreSQL database"""
        try:
            self.conn = psycopg2.connect(**self.db_params)
            self.conn.autocommit = True
            print("✅ Connected to PostgreSQL database")
        except Exception as e:
            print(f"❌ Failed to connect to database: {e}")
            print("Please ensure PostgreSQL is running and credentials are correct")
            sys.exit(1)

    def setup_extension(self):
        """Install and test pglance extension"""
        with self.conn.cursor() as cur:
            try:
                # Create extension if not exists
                cur.execute("CREATE EXTENSION IF NOT EXISTS pglance;")

                # Test basic functionality
                cur.execute("SELECT hello_pglance();")
                result = cur.fetchone()[0]
                assert result == "Hello, pglance", f"Unexpected result: {result}"

                print("✅ pglance extension is working correctly")
            except Exception as e:
                print(f"❌ Failed to setup pglance extension: {e}")
                print("Please ensure pglance extension is properly installed")
                sys.exit(1)
    
    def test_table_info(self, table_name: str, expected_columns: List[str]) -> bool:
        """Test lance_table_info function"""
        display_path = os.path.join(self.host_data_dir, table_name)
        query_path = os.path.join(self.pglance_data_prefix, table_name)
        print(f"\n🔍 Testing table info for: {display_path} (querying as {query_path})")
        
        with self.conn.cursor(cursor_factory=RealDictCursor) as cur:
            try:
                cur.execute(
                    "SELECT column_name, data_type, nullable FROM lance_table_info(%s) ORDER BY column_name;",
                    (query_path,)
                )
                results = cur.fetchall()

                print(f"📋 Table structure:")
                for row in results:
                    nullable_str = "YES" if row['nullable'] else "NO"
                    print(f"  - {row['column_name']}: {row['data_type']} (nullable: {nullable_str})")

                # Validate expected columns exist
                actual_columns = {row['column_name'] for row in results}
                expected_set = set(expected_columns)

                if not expected_set.issubset(actual_columns):
                    missing = expected_set - actual_columns
                    print(f"❌ Missing expected columns: {missing}")
                    return False

                print("✅ Table info test passed")
                return True

            except Exception as e:
                print(f"❌ Table info test failed: {e}")
                return False
    
    def test_table_stats(self, table_name: str, expected_min_rows: int = 0) -> bool:
        """Test lance_table_stats function"""
        display_path = os.path.join(self.host_data_dir, table_name)
        query_path = os.path.join(self.pglance_data_prefix, table_name)
        print(f"\n📊 Testing table stats for: {display_path} (querying as {query_path})")
        
        with self.conn.cursor(cursor_factory=RealDictCursor) as cur:
            try:
                cur.execute(
                    "SELECT version, num_rows, num_columns FROM lance_table_stats(%s);",
                    (query_path,)
                )
                result = cur.fetchone()

                print(f"📈 Table statistics:")
                print(f"  - Version: {result['version']}")
                print(f"  - Rows: {result['num_rows']}")
                print(f"  - Columns: {result['num_columns']}")

                # Validate minimum expectations
                if result['num_rows'] < expected_min_rows:
                    print(f"❌ Expected at least {expected_min_rows} rows, got {result['num_rows']}")
                    return False

                if result['num_columns'] <= 0:
                    print(f"❌ Expected positive column count, got {result['num_columns']}")
                    return False

                print("✅ Table stats test passed")
                return True

            except Exception as e:
                print(f"❌ Table stats test failed: {e}")
                return False
    
    def test_table_scan(self, table_name: str, limit: int = 5, expected_fields: List[str] = None) -> bool:
        """Test lance_scan_jsonb function"""
        display_path = os.path.join(self.host_data_dir, table_name)
        query_path = os.path.join(self.pglance_data_prefix, table_name)
        print(f"\n🔎 Testing table scan for: {display_path} (querying as {query_path}, limit: {limit})")
        
        with self.conn.cursor(cursor_factory=RealDictCursor) as cur:
            try:
                cur.execute(
                    "SELECT row_data FROM lance_scan_jsonb(%s, %s);",
                    (query_path, limit)
                )
                results = cur.fetchall()

                print(f"📄 Scanned {len(results)} rows:")
                for i, row in enumerate(results[:3]):  # Show first 3 rows
                    row_data = row['row_data']
                    print(f"  Row {i + 1}: {json.dumps(row_data, indent=2)}")

                if len(results) > 3:
                    print(f"  ... and {len(results) - 3} more rows")

                # Validate expected fields if provided
                if expected_fields and results:
                    first_row = results[0]['row_data']
                    actual_fields = set(first_row.keys())
                    expected_set = set(expected_fields)

                    if not expected_set.issubset(actual_fields):
                        missing = expected_set - actual_fields
                        print(f"❌ Missing expected fields: {missing}")
                        return False

                print("✅ Table scan test passed")
                return True

            except Exception as e:
                print(f"❌ Table scan test failed: {e}")
                return False
    
    def run_comprehensive_test(self) -> Dict[str, bool]:
        """Run comprehensive tests on all tables"""
        results = {}

        # Test configurations for each table type
        test_configs = {
            "simple_table": {
                "expected_columns": ["id", "name", "age", "salary", "is_active", "hire_date"],
                "expected_min_rows": 5,
                "scan_limit": 5
            },
            "vector_table": {
                "expected_columns": ["id", "document", "embedding", "metadata"],
                "expected_min_rows": 5,
                "scan_limit": 3
            },
            "complex_table": {
                "expected_columns": ["id", "user_name", "scores", "profile", "tags", "created_at"],
                "expected_min_rows": 3,
                "scan_limit": 3
            },
            "large_table": {
                "expected_columns": ["id", "value", "category", "flag"],
                "expected_min_rows": 1000,
                "scan_limit": 10
            }
        }
        
        # Iterate over table names defined in test_configs
        for table_name in test_configs.keys():
            print(f"\n{'='*60}")
            print(f"🧪 Testing {table_name}")
            print(f"{'='*60}")
            
            config = test_configs.get(table_name, {}) # Should always find a config
            
            # Run all tests for this table, passing the table_name
            info_test = self.test_table_info(
                table_name, 
                config.get("expected_columns", [])
            )
            
            stats_test = self.test_table_stats(
                table_name,
                config.get("expected_min_rows", 0)
            )
            
            scan_test = self.test_table_scan(
                table_name,
                config.get("scan_limit", 5),
                config.get("expected_columns", [])
            )

            # Overall result for this table
            table_result = info_test and stats_test and scan_test
            results[table_name] = table_result

            status = "✅ PASSED" if table_result else "❌ FAILED"
            print(f"\n📋 {table_name} overall result: {status}")

        return results

    def close(self):
        """Close database connection"""
        if self.conn:
            self.conn.close()
            print("✅ Database connection closed")


def main():
    """Main test execution function"""
    parser = argparse.ArgumentParser(description="End-to-end test for pglance extension")
    parser.add_argument("--cleanup", action="store_true", help="Clean up test files after completion")
    parser.add_argument("--host-data-dir", default="./lance_test_data", help="Directory on host to store Lance tables (default: ./lance_test_data)")
    parser.add_argument("--pglance-data-prefix", default="/test_data_in_container", help="Absolute path prefix for Lance tables inside the PostgreSQL/pglance environment (e.g., Docker container path)")
    parser.add_argument("--db-host", default="localhost", help="PostgreSQL host")
    parser.add_argument("--db-port", default="5432", help="PostgreSQL port")
    parser.add_argument("--db-name", default="postgres", help="PostgreSQL database name")
    parser.add_argument("--db-user", default="postgres", help="PostgreSQL username")
    parser.add_argument("--db-password", default="postgres", help="PostgreSQL password")

    args = parser.parse_args()

    # Setup host data directory
    host_data_dir = args.host_data_dir
    os.makedirs(host_data_dir, exist_ok=True)

    print(f"🚀 Starting pglance end-to-end test")
    print(f"📁 Host data directory for Lance tables: {host_data_dir}")
    print(f"🔩 pglance data prefix (container path): {args.pglance_data_prefix}")

    try:
        # Step 1: Generate Lance tables
        print(f"\n{'='*60}")
        print("📝 Step 1: Generating Lance tables")
        print(f"{'='*60}")
        
        generator = LanceTableGenerator(host_data_dir)
        # table_names_generated will store the relative names of tables
        table_names_generated = {
            "simple_table": generator.create_simple_table(),
            "vector_table": generator.create_vector_table(),
            "complex_table": generator.create_complex_table(),
            "large_table": generator.create_large_table()
        }
        # Verify that all expected tables were generated by LanceTableGenerator
        # This is implicitly checked as create_..._table would error out or return None
        # if there was an issue. The names returned are used more as keys.

        # Step 2: Test pglance functionality
        print(f"\n{'='*60}")
        print("🧪 Step 2: Testing pglance extension")
        print(f"\n{'='*60}")

        db_params = {
            'host': args.db_host,
            'port': args.db_port,
            'database': args.db_name,
            'user': args.db_user,
            'password': args.db_password
        }
        
        tester = PglanceTest(db_params, args.host_data_dir, args.pglance_data_prefix)
        tester.connect()
        tester.setup_extension()
        
        # Run comprehensive tests. It will use table names from its internal test_configs.
        results = tester.run_comprehensive_test()
        
        # Step 3: Report results
        print(f"\n{'='*60}")
        print("📊 Final Test Results")
        print(f"{'='*60}")

        total_tests = len(results)
        passed_tests = sum(1 for result in results.values() if result)

        for table_name, passed in results.items():
            status = "✅ PASSED" if passed else "❌ FAILED"
            print(f"{table_name}: {status}")

        print(f"\n📈 Summary: {passed_tests}/{total_tests} tests passed")

        if passed_tests == total_tests:
            print("🎉 All tests passed! pglance is working correctly.")
            exit_code = 0
        else:
            print("⚠️  Some tests failed. Please check the output above.")
            exit_code = 1

        tester.close()

    except Exception as e:
        print(f"❌ Test execution failed: {e}")
        exit_code = 1

    finally:
        # No automatic cleanup of the fixed host_data_dir by default.
        # User can manually remove it or add a specific cleanup flag if desired.
        pass
    
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
