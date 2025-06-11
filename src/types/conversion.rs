use arrow::datatypes::*;

/// Arrow to PostgreSQL data type mapping
pub fn arrow_to_pg_type(arrow_type: &DataType) -> Result<pgrx::PgOid, pgrx::PgSqlErrorCode> {
    match arrow_type {
        DataType::Boolean => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::BOOLOID)),
        DataType::Int8 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::CHAROID)),
        DataType::Int16 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INT2OID)),
        DataType::Int32 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INT4OID)),
        DataType::Int64 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INT8OID)),
        DataType::UInt8 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::CHAROID)),
        DataType::UInt16 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INT2OID)),
        DataType::UInt32 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INT4OID)),
        DataType::UInt64 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INT8OID)),
        DataType::Float16 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::FLOAT4OID)),
        DataType::Float32 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::FLOAT4OID)),
        DataType::Float64 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::FLOAT8OID)),
        DataType::Utf8 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TEXTOID)),
        DataType::LargeUtf8 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TEXTOID)),
        DataType::Binary => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::BYTEAOID)),
        DataType::LargeBinary => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::BYTEAOID)),
        DataType::Date32 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::DATEOID)),
        DataType::Date64 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::DATEOID)),
        DataType::Time32(_) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TIMEOID)),
        DataType::Time64(_) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TIMEOID)),
        DataType::Timestamp(_, _) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TIMESTAMPOID)),
        DataType::Interval(_) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INTERVALOID)),
        DataType::List(_) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::JSONBOID)),
        DataType::LargeList(_) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::JSONBOID)),
        DataType::FixedSizeList(field, _) => match field.data_type() {
            DataType::Float32 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::FLOAT4ARRAYOID)),
            DataType::Float64 => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::FLOAT8ARRAYOID)),
            _ => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::JSONBOID)),
        },
        DataType::Struct(_) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::JSONBOID)),
        DataType::Union(_, _) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::JSONBOID)),
        DataType::Dictionary(_, value_type) => arrow_to_pg_type(value_type),
        DataType::Decimal128(_, _) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::NUMERICOID)),
        DataType::Decimal256(_, _) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::NUMERICOID)),
        DataType::Map(_, _) => Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::JSONBOID)),
        _ => {
            pgrx::warning!(
                "Unsupported Arrow type: {:?}, converting to TEXT",
                arrow_type
            );
            Ok(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TEXTOID))
        }
    }
}

/// Get readable name for PostgreSQL type
pub fn pg_type_name(oid: pgrx::PgOid) -> &'static str {
    match oid {
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::BOOLOID) => "boolean",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::CHAROID) => "char",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INT2OID) => "int2",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INT4OID) => "int4",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INT8OID) => "int8",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::FLOAT4OID) => "float4",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::FLOAT8OID) => "float8",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TEXTOID) => "text",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::BYTEAOID) => "bytea",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::DATEOID) => "date",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TIMEOID) => "time",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TIMESTAMPOID) => "timestamp",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::INTERVALOID) => "interval",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::NUMERICOID) => "numeric",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::JSONBOID) => "jsonb",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::FLOAT4ARRAYOID) => "float4[]",
        pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::FLOAT8ARRAYOID) => "float8[]",
        _ => "unknown",
    }
}
