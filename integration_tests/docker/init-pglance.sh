#!/bin/bash
set -e



echo "Initializing pglance extension..."


psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE EXTENSION IF NOT EXISTS pglance;
EOSQL

echo "pglance extension initialization complete."