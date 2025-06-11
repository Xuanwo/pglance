use arrow::array::*;
use arrow::datatypes::*;
use pgrx::prelude::*;

/// Convert values from Arrow Array to PostgreSQL Datum
pub fn arrow_value_to_datum(array: &dyn arrow::array::Array, row_idx: usize) -> Result<Option<pgrx::pg_sys::Datum>, pgrx::PgSqlErrorCode> {
    if array.is_null(row_idx) {
        return Ok(None);
    }

    let datum = match array.data_type() {
        DataType::Boolean => {
            let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
            let value = arr.value(row_idx);
            value.into_datum()
        }
        DataType::Int32 => {
            let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
            let value = arr.value(row_idx);
            value.into_datum()
        }
        DataType::Int64 => {
            let arr = array.as_any().downcast_ref::<Int64Array>().unwrap();
            let value = arr.value(row_idx);
            value.into_datum()
        }
        DataType::Float32 => {
            let arr = array.as_any().downcast_ref::<Float32Array>().unwrap();
            let value = arr.value(row_idx);
            value.into_datum()
        }
        DataType::Float64 => {
            let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
            let value = arr.value(row_idx);
            value.into_datum()
        }
        DataType::Utf8 => {
            let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
            let value = arr.value(row_idx);
            value.into_datum()
        }
        DataType::LargeUtf8 => {
            let arr = array.as_any().downcast_ref::<LargeStringArray>().unwrap();
            let value = arr.value(row_idx);
            value.into_datum()
        }
        _ => {
            // For other types, convert to string representation
            let string_value = format!("{:?}", array.data_type());
            string_value.into_datum()
        }
    };

    datum.ok_or(pgrx::PgSqlErrorCode::ERRCODE_NULL_VALUE_NOT_ALLOWED)
        .map(Some)
}

/// Get PostgreSQL column information corresponding to Arrow Schema
pub fn arrow_schema_to_pg_columns(schema: &Schema) -> Vec<(String, pgrx::PgOid, bool)> {
    schema.fields().iter().map(|field| {
        let name = field.name().clone();
        let pg_type = super::conversion::arrow_to_pg_type(field.data_type())
            .unwrap_or(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TEXTOID));
        let nullable = field.is_nullable();
        (name, pg_type, nullable)
    }).collect()
}