#!/usr/bin/env sh
set -eu

DB_HOST="${DB_HOST:-postgres}"
DB_PORT="${DB_PORT:-5432}"
DB_USER="${DB_USER:-postgres}"
DB_PASSWORD="${DB_PASSWORD:-postgres}"
DB_NAME="${DB_NAME:-memo}"

: "${DATABASE_URL:="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"}"
export DATABASE_URL

echo "Waiting for PostgreSQL at ${DB_HOST}:${DB_PORT} ..."
until PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -d postgres -c "SELECT 1" >/dev/null 2>&1; do
  echo "PostgreSQL is not ready yet. Waiting..."
  sleep 2
done

echo "Creating database if it doesn't exist..."
sqlx database create --database-url "$DATABASE_URL" || true

echo "Running migrations..."
sqlx migrate run --source /app/db/migrations --database-url "$DATABASE_URL"

echo "Starting application..."
exec "$@"
