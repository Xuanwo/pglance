
CREATE EXTENSION IF NOT EXISTS pglance;


SELECT hello_pglance();


/*
-- Assuming you have a Lance table at /tmp/sample.lance
-- View table structure
SELECT 
    column_name,
    data_type,
    nullable
FROM lance_table_info('/tmp/sample.lance')
ORDER BY column_name;

-- Get table statistics
SELECT 
    version,
    num_rows,
    num_columns
FROM lance_table_stats('/tmp/sample.lance');

-- Scan first 5 rows of data
SELECT 
    row_data
FROM lance_scan_jsonb('/tmp/sample.lance', 5);
*/


SELECT 'pglance extension loaded successfully!' as status;
SELECT 'To use Lance table scanning features, please provide actual Lance table path' as note;
SELECT 'Example: SELECT * FROM lance_table_info(''/path/to/your/table'');' as example;


SELECT 
    proname as function_name,
    pg_get_function_result(oid) as return_type,
    pg_get_function_arguments(oid) as arguments
FROM pg_proc 
WHERE pronamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'public')
  AND proname LIKE 'lance_%'
  OR proname = 'hello_pglance'
ORDER BY proname;