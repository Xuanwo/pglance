#!/bin/bash
set -e

# This script is run when the Docker container for PostgreSQL starts for the first time.
# It creates the pglance extension in the specified database.

echo "Initializing pglance extension..."

# The base PostgreSQL Docker image sets POSTGRES_USER and POSTGRES_DB environment variables.
# We use these to connect to the correct database as the correct user.
# The -v ON_ERROR_STOP=1 option ensures that psql exits with an error code if the SQL command fails.
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE EXTENSION IF NOT EXISTS pglance;
EOSQL

echo "pglance extension initialization complete."