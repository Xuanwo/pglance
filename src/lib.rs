use pgrx::prelude::*;

use arrow::array::{
    Array, BinaryArray, BooleanArray, Date32Array, Date64Array, FixedSizeBinaryArray,
    FixedSizeListArray, Float16Array, Float32Array, Float64Array, GenericListArray, Int16Array,
    Int32Array, Int64Array, Int8Array, LargeBinaryArray, LargeStringArray, StringArray,
    StructArray, TimestampMicrosecondArray, TimestampMillisecondArray, TimestampNanosecondArray,
    TimestampSecondArray, UInt16Array, UInt32Array, UInt64Array, UInt8Array,
};
use arrow::datatypes::{DataType, TimeUnit as ArrowTimeUnit};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::NaiveDate;
use serde_json::{json, Map, Number, Value};

mod scanner;
mod types;

use scanner::LanceScanner;
use types::arrow_schema_to_pg_columns;

pgrx::pg_module_magic!();

// extension_sql_file!("./sql/bootstrap.sql", bootstrap);

fn arrow_value_to_serde_json(array: &dyn Array, row_idx: usize) -> Value {
    if array.is_null(row_idx) {
        return Value::Null;
    }

    match array.data_type() {
        DataType::Boolean => Value::Bool(
            array
                .as_any()
                .downcast_ref::<BooleanArray>()
                .unwrap()
                .value(row_idx),
        ),
        DataType::Int8 => json!(array
            .as_any()
            .downcast_ref::<Int8Array>()
            .unwrap()
            .value(row_idx)),
        DataType::Int16 => json!(array
            .as_any()
            .downcast_ref::<Int16Array>()
            .unwrap()
            .value(row_idx)),
        DataType::Int32 => json!(array
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap()
            .value(row_idx)),
        DataType::Int64 => json!(array
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap()
            .value(row_idx)),
        DataType::UInt8 => json!(array
            .as_any()
            .downcast_ref::<UInt8Array>()
            .unwrap()
            .value(row_idx)),
        DataType::UInt16 => json!(array
            .as_any()
            .downcast_ref::<UInt16Array>()
            .unwrap()
            .value(row_idx)),
        DataType::UInt32 => json!(array
            .as_any()
            .downcast_ref::<UInt32Array>()
            .unwrap()
            .value(row_idx)),
        DataType::UInt64 => json!(array
            .as_any()
            .downcast_ref::<UInt64Array>()
            .unwrap()
            .value(row_idx)),
        DataType::Float16 => {
            let val = array
                .as_any()
                .downcast_ref::<Float16Array>()
                .unwrap()
                .value(row_idx);
            Number::from_f64(val.to_f32() as f64)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
        DataType::Float32 => {
            let val = array
                .as_any()
                .downcast_ref::<Float32Array>()
                .unwrap()
                .value(row_idx);
            Number::from_f64(val as f64)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
        DataType::Float64 => {
            let val = array
                .as_any()
                .downcast_ref::<Float64Array>()
                .unwrap()
                .value(row_idx);
            Number::from_f64(val)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
        DataType::Utf8 => Value::String(
            array
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap()
                .value(row_idx)
                .to_string(),
        ),
        DataType::LargeUtf8 => Value::String(
            array
                .as_any()
                .downcast_ref::<LargeStringArray>()
                .unwrap()
                .value(row_idx)
                .to_string(),
        ),
        DataType::Date32 => {
            let days = array
                .as_any()
                .downcast_ref::<Date32Array>()
                .unwrap()
                .value(row_idx);
            NaiveDate::from_ymd_opt(1970, 1, 1)
                .and_then(|d| d.checked_add_signed(chrono::Duration::days(days as i64)))
                .map(|d| Value::String(d.to_string()))
                .unwrap_or(Value::Null)
        }
        DataType::Date64 => {
            let millis = array
                .as_any()
                .downcast_ref::<Date64Array>()
                .unwrap()
                .value(row_idx);
            chrono::DateTime::from_timestamp_millis(millis)
                .map(|dt| Value::String(dt.naive_utc().date().to_string()))
                .unwrap_or(Value::Null)
        }
        DataType::Timestamp(unit, tz_opt) => {
            let naive_dt_opt = match unit {
                ArrowTimeUnit::Second => {
                    let secs = array
                        .as_any()
                        .downcast_ref::<TimestampSecondArray>()
                        .unwrap()
                        .value(row_idx);
                    chrono::DateTime::from_timestamp(secs, 0).map(|dt| dt.naive_utc())
                }
                ArrowTimeUnit::Millisecond => {
                    let millis = array
                        .as_any()
                        .downcast_ref::<TimestampMillisecondArray>()
                        .unwrap()
                        .value(row_idx);
                    chrono::DateTime::from_timestamp_millis(millis).map(|dt| dt.naive_utc())
                }
                ArrowTimeUnit::Microsecond => {
                    let micros = array
                        .as_any()
                        .downcast_ref::<TimestampMicrosecondArray>()
                        .unwrap()
                        .value(row_idx);
                    chrono::DateTime::from_timestamp_micros(micros).map(|dt| dt.naive_utc())
                }
                ArrowTimeUnit::Nanosecond => {
                    let nanos = array
                        .as_any()
                        .downcast_ref::<TimestampNanosecondArray>()
                        .unwrap()
                        .value(row_idx);
                    chrono::DateTime::from_timestamp(
                        nanos / 1_000_000_000,
                        (nanos % 1_000_000_000) as u32,
                    )
                    .map(|dt| dt.naive_utc())
                }
            };
            let dt_str = naive_dt_opt
                .map(|dt| dt.to_string())
                .unwrap_or_else(|| "InvalidTimestamp".to_string());
            if let Some(tz) = tz_opt {
                Value::String(format!("{} {}", dt_str, tz))
            } else {
                Value::String(dt_str)
            }
        }
        DataType::List(_) | DataType::LargeList(_) | DataType::FixedSizeList(_, _) => {
            fn handle_list<OffsetSize: arrow::array::OffsetSizeTrait>(
                array: &dyn Array,
                row_idx: usize,
            ) -> Value {
                let list_array = array
                    .as_any()
                    .downcast_ref::<GenericListArray<OffsetSize>>()
                    .unwrap();
                let value_array_for_row = list_array.value(row_idx);
                let mut json_list = Vec::new();
                for i in 0..value_array_for_row.len() {
                    json_list.push(arrow_value_to_serde_json(value_array_for_row.as_ref(), i));
                }
                Value::Array(json_list)
            }
            fn handle_fixed_size_list(array: &dyn Array, row_idx: usize) -> Value {
                let list_array = array.as_any().downcast_ref::<FixedSizeListArray>().unwrap();
                let value_array_for_row = list_array.value(row_idx);
                let mut json_list = Vec::new();
                for i in 0..value_array_for_row.len() {
                    json_list.push(arrow_value_to_serde_json(value_array_for_row.as_ref(), i));
                }
                Value::Array(json_list)
            }

            match array.data_type() {
                DataType::List(_) => handle_list::<i32>(array, row_idx),
                DataType::LargeList(_) => handle_list::<i64>(array, row_idx),
                DataType::FixedSizeList(_, _) => handle_fixed_size_list(array, row_idx),
                _ => unreachable!(),
            }
        }
        DataType::Struct(fields) => {
            let struct_array = array.as_any().downcast_ref::<StructArray>().unwrap();
            let mut json_map = Map::new();
            for (i, field) in fields.iter().enumerate() {
                let field_array = struct_array.column(i);
                json_map.insert(
                    field.name().clone(),
                    arrow_value_to_serde_json(field_array.as_ref(), row_idx),
                );
            }
            Value::Object(json_map)
        }
        DataType::Binary => Value::String(
            STANDARD.encode(
                array
                    .as_any()
                    .downcast_ref::<BinaryArray>()
                    .unwrap()
                    .value(row_idx),
            ),
        ),
        DataType::LargeBinary => Value::String(
            STANDARD.encode(
                array
                    .as_any()
                    .downcast_ref::<LargeBinaryArray>()
                    .unwrap()
                    .value(row_idx),
            ),
        ),
        DataType::FixedSizeBinary(_) => Value::String(
            STANDARD.encode(
                array
                    .as_any()
                    .downcast_ref::<FixedSizeBinaryArray>()
                    .unwrap()
                    .value(row_idx),
            ),
        ),

        _ => Value::String(format!("<unsupported_type: {:?}>", array.data_type())),
    }
}

#[pg_extern]
fn hello_pglance() -> &'static str {
    "Hello, pglance"
}

/// Scan Lance table and return basic table information
#[pg_extern]
pub fn lance_table_info(
    table_path: &str,
) -> TableIterator<
    'static,
    (
        name!(column_name, String),
        name!(data_type, String),
        name!(nullable, bool),
    ),
> {
    let scanner = LanceScanner::new(table_path)
        .unwrap_or_else(|_| pgrx::error!("Failed to open Lance table at: {}", table_path));

    let schema = scanner.schema();
    let columns = arrow_schema_to_pg_columns(schema.as_ref());

    let rows: Vec<_> = columns
        .into_iter()
        .map(|(name, pg_type, nullable)| {
            let type_name = types::pg_type_name(pg_type).to_string();
            (name, type_name, nullable)
        })
        .collect();

    TableIterator::new(rows)
}

/// Get Lance table statistics
#[pg_extern]
pub fn lance_table_stats(
    table_path: &str,
) -> TableIterator<
    'static,
    (
        name!(version, i64),
        name!(num_rows, i64),
        name!(num_columns, i32),
    ),
> {
    let scanner = LanceScanner::new(table_path)
        .unwrap_or_else(|_| pgrx::error!("Failed to open Lance table at: {}", table_path));

    let stats = scanner
        .get_stats()
        .unwrap_or_else(|_| pgrx::error!("Failed to get table statistics"));

    let row = (
        stats.version as i64,
        stats.num_rows as i64,
        stats.num_columns() as i32,
    );

    TableIterator::new(std::iter::once(row))
}

/// Scan Lance table and return data in JSONB format
#[pg_extern]
pub fn lance_scan_jsonb(
    table_path: &str,
    limit: default!(Option<i64>, "NULL"),
) -> TableIterator<'static, (name!(row_data, pgrx::JsonB),)> {
    let scanner = LanceScanner::new(table_path)
        .unwrap_or_else(|_| pgrx::error!("Failed to open Lance table at: {}", table_path));

    let scan_iter = scanner
        .scan_with_filter(None, limit)
        .unwrap_or_else(|_| pgrx::error!("Failed to create scan iterator"));

    let schema = scanner.schema();

    let mut results = Vec::new();
    let mut rows_outputted_count = 0i64;

    'batch_loop: for record_batch in scan_iter.batches {
        for row_idx_in_batch in 0..record_batch.num_rows() {
            if let Some(l_pg) = limit {
                if rows_outputted_count >= l_pg {
                    break 'batch_loop;
                }
            }

            let mut json_map = Map::new();
            for (col_idx, field) in schema.fields().iter().enumerate() {
                let column_array = record_batch.column(col_idx);
                let value = arrow_value_to_serde_json(column_array.as_ref(), row_idx_in_batch);
                json_map.insert(field.name().clone(), value);
            }
            results.push((pgrx::JsonB(Value::Object(json_map)),));
            rows_outputted_count += 1;
        }
    }

    TableIterator::new(results)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use arrow::array::{BooleanArray, Float32Array, Int32Array, StringArray};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use lance::Dataset;
    use pgrx::prelude::*;
    use std::sync::Arc;
    use tempfile::TempDir;

    /// Test data generator for Lance tables using synchronous blocking operations
    struct LanceTestDataGenerator {
        temp_dir: TempDir,
    }

    impl LanceTestDataGenerator {
        fn new() -> Result<Self, Box<dyn std::error::Error>> {
            let temp_dir = TempDir::new()?;
            Ok(Self { temp_dir })
        }

        fn get_base_path(&self) -> &std::path::Path {
            self.temp_dir.path()
        }

        /// Create a simple table with basic data types
        fn create_simple_table(&self) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
            let table_path = self.get_base_path().join("simple_table");

            // Create sample data with various basic types
            let id_array = Int32Array::from(vec![1, 2, 3, 4, 5]);
            let name_array = StringArray::from(vec!["Alice", "Bob", "Charlie", "David", "Eve"]);
            let age_array = Int32Array::from(vec![25, 30, 35, 40, 45]);
            let salary_array =
                Float32Array::from(vec![50000.5, 65000.0, 80000.25, 95000.75, 120000.0]);
            let is_active_array = BooleanArray::from(vec![true, true, false, true, false]);

            let schema = Arc::new(Schema::new(vec![
                Field::new("id", DataType::Int32, false),
                Field::new("name", DataType::Utf8, false),
                Field::new("age", DataType::Int32, false),
                Field::new("salary", DataType::Float32, false),
                Field::new("is_active", DataType::Boolean, false),
            ]));

            let batch = RecordBatch::try_new(
                schema.clone(),
                vec![
                    Arc::new(id_array),
                    Arc::new(name_array),
                    Arc::new(age_array),
                    Arc::new(salary_array),
                    Arc::new(is_active_array),
                ],
            )?;

            // Use RecordBatchIterator for lance
            let reader = arrow::record_batch::RecordBatchIterator::new(vec![Ok(batch)], schema);

            // Use a new runtime for async operation
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                Dataset::write(reader, table_path.to_str().unwrap(), None).await
            })?;

            Ok(table_path)
        }

        /// Create a table with vector embeddings
        fn create_vector_table(&self) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
            let table_path = self.get_base_path().join("vector_table");

            let id_array = Int32Array::from(vec![1, 2, 3]);
            let document_array = StringArray::from(vec!["doc1", "doc2", "doc3"]);

            // Create vector embeddings as List array
            let mut list_builder =
                arrow::array::ListBuilder::new(arrow::array::Float32Builder::new());

            // Add each embedding vector
            for embedding in [
                vec![0.1, 0.2, 0.3, 0.4],
                vec![0.5, 0.6, 0.7, 0.8],
                vec![0.9, 1.0, 1.1, 1.2],
            ] {
                for value in embedding {
                    list_builder.values().append_value(value);
                }
                list_builder.append(true);
            }
            let list_array = list_builder.finish();

            let schema = Arc::new(Schema::new(vec![
                Field::new("id", DataType::Int32, false),
                Field::new("document", DataType::Utf8, false),
                Field::new(
                    "embedding",
                    DataType::List(Arc::new(Field::new("item", DataType::Float32, true))),
                    false,
                ),
            ]));

            let batch = RecordBatch::try_new(
                schema.clone(),
                vec![
                    Arc::new(id_array),
                    Arc::new(document_array),
                    Arc::new(list_array),
                ],
            )?;

            let reader = arrow::record_batch::RecordBatchIterator::new(vec![Ok(batch)], schema);

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                Dataset::write(reader, table_path.to_str().unwrap(), None).await
            })?;

            Ok(table_path)
        }
    }

    #[pg_test]
    fn test_hello_pglance() {
        assert_eq!("Hello, pglance", crate::hello_pglance());
    }

    #[pg_test]
    fn test_error_handling() {
        // Test with invalid path
        let result = std::panic::catch_unwind(|| {
            let _: Vec<(String, String, bool)> =
                crate::lance_table_info("/invalid/path/does/not/exist").collect::<Vec<_>>();
        });
        assert!(result.is_err());
    }

    #[pg_test]
    fn test_simple_table_integration() {
        let generator =
            LanceTestDataGenerator::new().expect("Failed to create test data generator");
        let table_path = generator
            .create_simple_table()
            .expect("Failed to create simple table");
        let table_path_str = table_path.to_str().unwrap();

        // Test table info
        let table_info: Vec<(String, String, bool)> =
            crate::lance_table_info(table_path_str).collect::<Vec<_>>();

        assert_eq!(table_info.len(), 5);

        // Check specific columns
        let id_column = table_info.iter().find(|(name, _, _)| name == "id").unwrap();
        assert_eq!(id_column.1, "int4");
        assert!(!id_column.2); // not nullable

        let name_column = table_info
            .iter()
            .find(|(name, _, _)| name == "name")
            .unwrap();
        assert_eq!(name_column.1, "text");

        let salary_column = table_info
            .iter()
            .find(|(name, _, _)| name == "salary")
            .unwrap();
        assert_eq!(salary_column.1, "float4");

        // Test table stats
        let stats: Vec<(i64, i64, i32)> =
            crate::lance_table_stats(table_path_str).collect::<Vec<_>>();

        assert_eq!(stats.len(), 1);
        let (version, num_rows, num_columns) = stats[0];
        assert!(version >= 1);
        assert_eq!(num_rows, 5);
        assert_eq!(num_columns, 5);

        // Test data scanning
        let data: Vec<(pgrx::JsonB,)> =
            crate::lance_scan_jsonb(table_path_str, Some(3)).collect::<Vec<_>>();

        assert_eq!(data.len(), 3);

        // Verify first row data
        let first_row = &data[0].0;
        let json_value = &first_row.0;
        assert_eq!(json_value["id"], 1);
        assert_eq!(json_value["name"], "Alice");
        assert_eq!(json_value["age"], 25);
        // Use approximate comparison for floating point
        let salary = json_value["salary"].as_f64().unwrap();
        assert!((salary - 50000.5).abs() < 0.1);
        assert_eq!(json_value["is_active"], true);
    }

    #[pg_test]
    fn test_vector_table_integration() {
        let generator =
            LanceTestDataGenerator::new().expect("Failed to create test data generator");
        let table_path = generator
            .create_vector_table()
            .expect("Failed to create vector table");
        let table_path_str = table_path.to_str().unwrap();

        // Test table info
        let table_info: Vec<(String, String, bool)> =
            crate::lance_table_info(table_path_str).collect::<Vec<_>>();

        assert_eq!(table_info.len(), 3);

        // Check embedding column (should be a list type)
        let embedding_column = table_info
            .iter()
            .find(|(name, _, _)| name == "embedding")
            .unwrap();
        assert!(embedding_column.1.contains("json")); // Lists are converted to JSON in PostgreSQL

        // Test data scanning with limit
        let data: Vec<(pgrx::JsonB,)> =
            crate::lance_scan_jsonb(table_path_str, Some(2)).collect::<Vec<_>>();

        assert_eq!(data.len(), 2);

        // Verify first row has vector data
        let first_row = &data[0].0;
        let json_value = &first_row.0;
        assert_eq!(json_value["id"], 1);
        assert_eq!(json_value["document"], "doc1");

        // Check that embedding is an array
        assert!(json_value["embedding"].is_array());
        let embedding = json_value["embedding"].as_array().unwrap();
        assert_eq!(embedding.len(), 4);
        // Use approximate comparison for floating point values
        let val0 = embedding[0].as_f64().unwrap();
        let val1 = embedding[1].as_f64().unwrap();
        assert!((val0 - 0.1).abs() < 0.01);
        assert!((val1 - 0.2).abs() < 0.01);
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
