#!/bin/bash
# This script runs once when the postgres container is first created.
# It creates a non-superuser database role for the application.
# The postgres image automatically executes scripts in /docker-entrypoint-initdb.d/.

set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER "$APP_DB_USER" WITH PASSWORD '$APP_DB_PASSWORD';
    GRANT ALL PRIVILEGES ON DATABASE "$POSTGRES_DB" TO "$APP_DB_USER";
    GRANT ALL ON SCHEMA public TO "$APP_DB_USER";
EOSQL
