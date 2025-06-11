use arrow::record_batch::RecordBatch;
use lance::Dataset;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Lance table scanner
pub struct LanceScanner {
    dataset: Dataset,
    runtime: Arc<Runtime>,
    batch_size: usize,
}

impl LanceScanner {
    /// Create a new Lance scanner
    pub fn new(table_path: &str) -> Result<Self, pgrx::PgSqlErrorCode> {
        // Create async runtime
        let runtime =
            Arc::new(Runtime::new().map_err(|_e| pgrx::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR)?);

        // Open dataset in async runtime
        let dataset = runtime.block_on(async {
            Dataset::open(table_path)
                .await
                .map_err(|_e| pgrx::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR)
        })?;

        Ok(Self {
            dataset,
            runtime,
            batch_size: 1024,
        })
    }

    /// Get table schema
    pub fn schema(&self) -> Arc<arrow::datatypes::Schema> {
        let lance_schema = self.dataset.schema();
        let arrow_fields: Vec<Arc<arrow::datatypes::Field>> = lance_schema
            .fields
            .iter()
            .map(|field| {
                Arc::new(arrow::datatypes::Field::new(
                    field.name.clone(),
                    field.data_type().clone(),
                    field.nullable,
                ))
            })
            .collect();
        Arc::new(arrow::datatypes::Schema::new(arrow_fields))
    }

    /// Scan with filter conditions
    pub fn scan_with_filter(
        &self,
        filter: Option<String>,
        limit: Option<i64>,
    ) -> Result<LanceScanIterator, pgrx::PgSqlErrorCode> {
        let runtime = Arc::clone(&self.runtime);
        let dataset = self.dataset.clone();
        let batch_size = self.batch_size;

        let batches = runtime.block_on(async move {
            let mut scan = dataset.scan();

            scan.batch_size(batch_size);

            if let Some(filter_expr) = filter {
                scan.filter(&filter_expr)
                    .map_err(|_e| pgrx::PgSqlErrorCode::ERRCODE_SYNTAX_ERROR)?;
            }

            if let Some(limit_val) = limit {
                let _ = scan.limit(Some(limit_val), None);
            }

            let stream = scan
                .try_into_stream()
                .await
                .map_err(|_e| pgrx::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR)?;

            let mut batches = Vec::new();
            use futures::StreamExt;

            let mut stream = Box::pin(stream);
            while let Some(batch_result) = stream.next().await {
                let batch =
                    batch_result.map_err(|_e| pgrx::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR)?;
                batches.push(batch);
            }

            Ok::<Vec<RecordBatch>, pgrx::PgSqlErrorCode>(batches)
        })?;

        Ok(LanceScanIterator::new(batches))
    }

    /// Get table statistics
    pub fn get_stats(&self) -> Result<LanceTableStats, pgrx::PgSqlErrorCode> {
        let dataset = &self.dataset;

        let version = dataset.version().version;
        let lance_schema = dataset.schema();
        let arrow_fields: Vec<Arc<arrow::datatypes::Field>> = lance_schema
            .fields
            .iter()
            .map(|field| {
                Arc::new(arrow::datatypes::Field::new(
                    field.name.clone(),
                    field.data_type().clone(),
                    field.nullable,
                ))
            })
            .collect();
        let schema = Arc::new(arrow::datatypes::Schema::new(arrow_fields));

        let num_rows = self.runtime.block_on(async {
            dataset
                .count_rows(None)
                .await
                .map_err(|_e| pgrx::PgSqlErrorCode::ERRCODE_INTERNAL_ERROR)
        })?;

        Ok(LanceTableStats {
            version,
            num_rows,
            schema,
        })
    }
}

/// Lance scan iterator
pub struct LanceScanIterator {
    pub batches: Vec<RecordBatch>,
    current_batch: usize,
    current_row: usize,
}

impl LanceScanIterator {
    fn new(batches: Vec<RecordBatch>) -> Self {
        Self {
            batches,
            current_batch: 0,
            current_row: 0,
        }
    }

    /// Get next row data
    pub fn next_row(&mut self) -> Option<Result<LanceRow, pgrx::PgSqlErrorCode>> {
        loop {
            if self.current_batch >= self.batches.len() {
                return None;
            }

            let batch = &self.batches[self.current_batch];

            if self.current_row >= batch.num_rows() {
                self.current_batch += 1;
                self.current_row = 0;
                continue;
            }

            let row = LanceRow {
                batch,
                row_index: self.current_row,
            };

            self.current_row += 1;
            return Some(Ok(row));
        }
    }
}

/// Lance row data reference
pub struct LanceRow<'a> {
    batch: &'a RecordBatch,
    row_index: usize,
}

impl<'a> LanceRow<'a> {
    /// Get value of specified column
    pub fn get_column_value(
        &self,
        column_index: usize,
    ) -> Result<Option<pgrx::pg_sys::Datum>, pgrx::PgSqlErrorCode> {
        if column_index >= self.batch.num_columns() {
            return Err(pgrx::PgSqlErrorCode::ERRCODE_INVALID_COLUMN_REFERENCE);
        }

        let column = self.batch.column(column_index);
        crate::types::arrow_value_to_datum(column.as_ref(), self.row_index)
    }

    /// Get all column values
    pub fn get_all_values(&self) -> Result<Vec<Option<pgrx::pg_sys::Datum>>, pgrx::PgSqlErrorCode> {
        let mut values = Vec::with_capacity(self.batch.num_columns());

        for i in 0..self.batch.num_columns() {
            values.push(self.get_column_value(i)?);
        }

        Ok(values)
    }
}

/// Lance table statistics
#[derive(Debug)]
pub struct LanceTableStats {
    pub version: u64,
    pub num_rows: usize,
    pub schema: Arc<arrow::datatypes::Schema>,
}

impl LanceTableStats {
    /// Get column count
    pub fn num_columns(&self) -> usize {
        self.schema.fields().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_lance_scanner_creation() {}
}
