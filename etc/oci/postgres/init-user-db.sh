#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    
    CREATE USER "env-compare" WITH PASSWORD 'password';
    CREATE DATABASE "env-compare" IF NOT EXISTS;
    GRANT ALL PRIVILEGES ON DATABASE "env-compare" TO "env-compare";
    
EOSQL

