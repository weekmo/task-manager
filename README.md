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

Use the provided `api-tests.http` file with the [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension for VS Code.

### Running Tests

```bash
# Run tests locally
cargo test

# Run tests in Docker
make test

# Run tests with coverage
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
├── Dockerfile               # Multi-stage Docker build
├── docker-compose.yml       # Docker Compose configuration
├── docker-entrypoint.sh     # Container entrypoint script
├── api-tests.http           # HTTP API tests
├── Cargo.toml               # Rust dependencies
└── Makefile                 # Build automation
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
