# Docker Compose Setup for Task Manager

This project uses Docker Compose with multi-stage builds.

## Architecture

The `Dockerfile` contains three stages:
1. **Builder** - Compiles the Rust application with all dependencies
2. **Tester** - Runs unit tests to ensure code quality
3. **Runtime** - Minimal production image with only the compiled binary

## Quick Start

### 1. Build and Run with Docker Compose

```bash
# Build and start all services (postgres + app)
docker-compose up --build

# Or run in detached mode
docker-compose up -d --build
```

### 2. Run Database Migrations

Migrations need to be run once before the app can work:

```bash
# Install sqlx-cli if not already installed
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run
```

Or manually create the tables using the SQL in `migrations/20260223150457_create_users_and_tasks.sql`.

### 3. Test the API

The application will be available at `http://localhost:3000`

Use the `api-tests.http` file with VS Code REST Client extension to test endpoints.

## Docker Commands

```bash
# Build with test stage
docker-compose build

# Rebuild without cache
docker-compose build --no-cache

# View logs
docker-compose logs -f app

# Stop services
docker-compose down

# Stop and remove volumes (deletes database data)
docker-compose down -v

# Run only tests
docker build --target tester -t task-manager-test .
```

## Environment Variables

Required environment variables (set in docker-compose.yml):
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key for JWT token generation
- `RUST_LOG` - Log level (info, debug, error)

## Development

For local development without Docker:

```bash
# Start only the database
docker-compose up postgres

# Set environment variables
export DATABASE_URL=postgres://postgres:password@localhost:5432/task_manager
export JWT_SECRET=your_super_secret_key_change_this

# Run migrations
sqlx migrate run

# Run the app
cargo run

# Run tests
cargo test
```

## Production Deployment

The runtime stage creates a minimal Debian-based image (~100MB) with:
- Non-root user for security
- Only necessary runtime dependencies
- Health checks configured
- Automatic restart policy

## Troubleshooting

**App can't connect to database:**
- Ensure postgres is healthy: `docker-compose ps`
- Check logs: `docker-compose logs postgres`
- The app waits for postgres health check before starting

**Migrations not applied:**
- Run migrations manually: `sqlx migrate run`
- Or connect to postgres container and run SQL manually

**Build fails:**
- Clear cache: `docker-compose build --no-cache`
- Remove old images: `docker system prune -a`
