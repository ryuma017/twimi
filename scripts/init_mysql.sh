#!/usr/bin/env bash
set -xeo pipefail

DB_USER="${MYSQL_USER:=docker}"
DB_PASSWORD="${MYSQL_PASSWORD:=docker}"
DB_HOST="${MYSQL_HOST:=127.0.0.1}"
DB_PORT="${MYSQL_TCP_PORT:=3306}"
DB_NAME="${MYSQL_DATABASE:=twimi}"

if [ -z "${SKIP_DOCKER_COMPOSE}" ]; then
    docker compose up -d
fi

# Install sqlx-cli: `cargo install sqlx-cli --no-default-features --features mysql`
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Run the following command to install sqlx-cli"
    echo >&2 "    cargo install sqlx-cli --no-default-features --features rustls,mysql"
    exit 1
fi

export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}

sqlx database create
if [ -r ./migrations/*.sql ]; then
    sqlx migrate run
fi
