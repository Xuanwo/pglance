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
use chrono::{NaiveDate, NaiveDateTime};
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
fn lance_table_info(
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
fn lance_table_stats(
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
fn lance_scan_jsonb(
    table_path: &str,
    limit: default!(Option<i64>, "NULL"),
) -> TableIterator<'static, (name!(row_data, pgrx::JsonB),)> {
    let scanner = LanceScanner::new(table_path)
        .unwrap_or_else(|_| pgrx::error!("Failed to open Lance table at: {}", table_path));

    let mut scan_iter = scanner
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
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pglance() {
        assert_eq!("Hello, pglance", crate::hello_pglance());
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
