# Task Manager API

A RESTful API for task management built with Rust, Axum, PostgreSQL, and JWT authentication.

## Features

- 🔐 JWT-based authentication
- 👤 User registration and login
- ✅ CRUD operations for tasks
- 🐳 Docker and Docker Compose support
- 🧪 Multi-stage Docker builds (builder, test, runtime)
- 📦 Automated database migrations
- 🔒 Secure password hashing with bcrypt

## Tech Stack

- **Framework**: Axum
- **Database**: PostgreSQL 16
- **Authentication**: JWT (jsonwebtoken)
- **Password Hashing**: bcrypt
- **ORM**: SQLx
- **Async Runtime**: Tokio

## Installation

### Pre-built Binaries (Recommended)

Download pre-built binaries for Linux or Windows from the [Releases page](../../releases).

**Linux:**
```bash
wget https://github.com/USERNAME/task-manager/releases/latest/download/task-manager-linux-x86_64.tar.gz
tar xzf task-manager-linux-x86_64.tar.gz
chmod +x task-manager
./task-manager
```

**Windows:**
Download `task-manager-windows-x86_64.zip`, extract, and run `task-manager.exe`

See [RELEASE.md](RELEASE.md) for detailed installation instructions.

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Build and start all services
docker-compose up --build

# Or use the Makefile
make up-build

# View logs
make logs
```

The API will be available at `http://localhost:3000`

### Local Development

```bash
# Start only the database
make dev

# Set environment variables
export DATABASE_URL=postgres://postgres:password@localhost:5432/task_manager
export JWT_SECRET=your_super_secret_key_change_this

# Run migrations
make migration-run-local

# Run the application
cargo run

# Run tests
cargo test
```

## Environment Variables

Create a `.env` file (already exists in the project):

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/task_manager
JWT_SECRET=your_super_secret_key_change_this
```

## API Endpoints

### Authentication

#### Register
```http
POST /auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}
```

#### Login
```http
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}
```

Response:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGc..."
}
```

### Tasks (Requires Authentication)

All task endpoints require the `Authorization: Bearer <token>` header.

#### Get All Tasks
```http
GET /tasks
Authorization: Bearer <your-jwt-token>
```

#### Create Task
```http
POST /tasks
Authorization: Bearer <your-jwt-token>
Content-Type: application/json

{
  "title": "Complete project",
  "description": "Finish the API documentation"
}
```

#### Update Task
```http
PUT /tasks/{task_id}
Authorization: Bearer <your-jwt-token>
Content-Type: application/json

{
  "title": "Updated title",
  "description": "Updated description",
  "done": true
}
```

#### Delete Task
```http
DELETE /tasks/{task_id}
Authorization: Bearer <your-jwt-token>
```

## Testing

The project includes comprehensive unit and integration tests.

### Quick Start

```bash
# Start database for integration tests
make dev

# Run all tests
make test

# Run only unit tests (no DB required)
make test-unit

# Run only integration tests (requires DB)
make test-integration

# Run tests in Docker
make test-docker
```

### Test Coverage

- ✅ **15 Unit Tests** - Testing models, errors, JWT, and serialization
- ✅ **11 Integration Tests** - Testing full API workflows
- ✅ **Total: 26 automated tests**

**Test scenarios covered:**
- User registration and authentication
- Task CRUD operations
- Authorization and user isolation
- Error handling and edge cases
- JWT token creation and validation

### Test Files

- `tests/integration_tests.rs` - Full API integration tests
- `tests/error_tests.rs` - Error handling unit tests
- `tests/auth_tests.rs` - JWT authentication unit tests
- `tests/user_model_tests.rs` - User model unit tests
- `tests/task_model_tests.rs` - Task model unit tests
- `api-tests.http` - Manual API tests for REST Client

See [TESTING.md](TESTING.md) for detailed testing documentation.

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_tests

# Run tests in Docker
make test-docker

# Run with output
cargo test -- --nocapture
```

## Docker

### Multi-Stage Build

The Dockerfile includes three stages:

1. **Builder**: Compiles the Rust application
2. **Tester**: Runs tests to ensure code quality
3. **Runtime**: Minimal production image (~100MB)

### Makefile Commands

```bash
make help              # Show all available commands
make build             # Build Docker images
make up                # Start services
make down              # Stop services
make logs              # View logs
make test              # Run tests
make restart           # Restart services
make clean             # Clean up Docker resources
make shell-app         # Open shell in app container
make shell-db          # Open PostgreSQL shell
```

See [DOCKER.md](DOCKER.md) for detailed Docker documentation.

## Project Structure

```
task-manager/
├── src/
│   ├── main.rs              # Application entry point
│   ├── db.rs                # Database connection pool
│   ├── errors.rs            # Custom error types
│   ├── handlers/            # HTTP request handlers
│   │   ├── mod.rs
│   │   ├── auth.rs          # Authentication handlers
│   │   └── tasks.rs         # Task CRUD handlers
│   ├── middleware/          # Custom middleware
│   │   ├── mod.rs
│   │   └── auth.rs          # JWT authentication middleware
│   └── models/              # Data models
│       ├── mod.rs
│       ├── user.rs          # User model and DTOs
│       └── task.rs          # Task model and DTOs
├── migrations/              # Database migrations
├── Dockerfile               # Multi-stage Docker build (includes migration logic)
├── docker-compose.yml       # Docker Compose configuration
├── api-tests.http           # HTTP API tests
├── Cargo.toml               # Rust dependencies
└── Makefile                 # Build automation with integrated test runner
```

## Database Schema

### Users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Tasks Table
```sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Security Features

- ✅ Passwords hashed with bcrypt
- ✅ JWT token-based authentication
- ✅ Token expiration (24 hours)
- ✅ Non-root user in Docker container
- ✅ SQL injection protection via SQLx
- ✅ CORS headers can be added as needed

## Troubleshooting

### Database Connection Issues

```bash
# Check if postgres is running
make health

# View postgres logs
make logs-db

# Restart postgres
docker-compose restart postgres
```

### Migration Issues

```bash
# Run migrations manually
make migration-run

# Or connect to database directly
make shell-db
```

### Build Issues

```bash
# Clean and rebuild
make clean
make build-no-cache
```

## Development

### Adding New Endpoints

1. Add handler in `src/handlers/`
2. Define models/DTOs in `src/models/`
3. Register route in `src/main.rs`
4. Add tests in `api-tests.http`

### Running Migrations

```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## License

MIT

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Releases

Releases are automatically built and published when a version tag is pushed:

```bash
git tag v1.0.0
git push origin v1.0.0
```

This triggers the [Release workflow](/.github/workflows/release.yml) which builds binaries for:
- Linux x86_64 (glibc and musl)
- Windows x86_64

See [RELEASE.md](RELEASE.md) for the complete release process.

## CI/CD

The project uses GitHub Actions for continuous integration:

- **CI Pipeline** ([.github/workflows/ci.yml](/.github/workflows/ci.yml))
  - Tests (unit + integration)
  - Code formatting (rustfmt)
  - Linting (clippy)
  - Security audit (cargo-audit)
  - Docker build verification

- **Release Pipeline** ([.github/workflows/release.yml](/.github/workflows/release.yml))
  - Multi-platform binary builds
  - Automated GitHub releases
  - Platform-specific packaging

