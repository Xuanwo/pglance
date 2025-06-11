use arrow::datatypes::*;

pub fn arrow_schema_to_pg_columns(schema: &Schema) -> Vec<(String, pgrx::PgOid, bool)> {
    schema
        .fields()
        .iter()
        .map(|field| {
            let name = field.name().clone();
            let pg_type = super::conversion::arrow_to_pg_type(field.data_type())
                .unwrap_or(pgrx::PgOid::BuiltIn(pgrx::PgBuiltInOids::TEXTOID));
            let nullable = field.is_nullable();
            (name, pg_type, nullable)
        })
        .collect()
}
