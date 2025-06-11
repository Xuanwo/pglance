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
}

impl LanceScanIterator {
    fn new(batches: Vec<RecordBatch>) -> Self {
        Self { batches }
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
