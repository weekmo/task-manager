#!/bin/sh
set -e

echo "Waiting for postgres to be ready..."
# The health check in docker-compose should handle this, but add small delay
sleep 2

echo "Running database migrations..."
sqlx migrate run || {
    echo "Migration failed, but continuing anyway (tables might already exist)"
}

echo "Starting application..."
exec "$@"
