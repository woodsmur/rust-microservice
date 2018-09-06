#!/bin/sh -e

psql --variable=ON_ERROR_STOP=1 --username "postgres" <<-EOSQL
    CREATE ROLE diesel WITH LOGIN PASSWORD 'diesel';
    ALTER ROLE diesel CREATEDB;
    CREATE DATABASE rust_microservice;
    GRANT ALL PRIVILEGES ON DATABASE rust_microservice TO diesel;
EOSQL