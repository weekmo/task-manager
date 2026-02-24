# Testing Guide

This document describes the testing strategy and how to run tests for the Task Manager API.

## Test Structure

The project includes three types of tests - all organized in the `tests/` folder:

### 1. Unit Tests
Located in separate test files in the `tests/` directory:

**Test Files:**
- `tests/error_tests.rs` - Error type creation and status code mapping (2 tests)
- `tests/auth_tests.rs` - JWT creation and validation (3 tests)
- `tests/user_model_tests.rs` - User model serialization/deserialization (4 tests)
- `tests/task_model_tests.rs` - Task model serialization/deserialization (6 tests)

**Total: 15 unit tests**

### 2. Integration Tests
Located in `tests/integration_tests.rs`, these test the full API workflows end-to-end.

**Test coverage:**
- ✅ User registration (success and duplicate email)
- ✅ User login (success and wrong password)
- ✅ Task creation (with auth and without auth)
- ✅ Get all tasks
- ✅ Update task
- ✅ Delete task
- ✅ Task isolation between users

**Total: 11 integration tests**

### 3. Manual API Tests
Located in `api-tests.http`, these are HTTP request files for manual testing with REST Client extension in VS Code.

## Running Tests

### Quick Start

```bash
# Run all tests (requires database)
make test

# Run only unit tests (no database required)
make test-unit

# Run only integration tests (requires database)
make test-integration

# Run tests in Docker
make test-docker
```

### Detailed Commands

#### 1. Unit Tests Only

Unit tests don't require a database connection:

```bash
cargo test --test error_tests --test auth_tests --test user_model_tests --test task_model_tests
```

Or:

```bash
make test-unit
```

#### 2. Integration Tests Only

Integration tests require PostgreSQL to be running:

```bash
# Start database
make dev

# Run migrations
make migration-run-local

# Run integration tests
cargo test --test integration_tests
```

Or:

```bash
make test-integration
```

#### 3. All Tests

Run the comprehensive test suite with database checking:

```bash
make test
```

This will:
- Check if the database is accessible
- Set required environment variables (JWT_SECRET, DATABASE_URL)
- Run all tests (unit + integration)
- Display a summary

#### 4. Watch Mode

Continuously run tests on file changes (requires `cargo-watch`):

```bash
# Install cargo-watch
cargo install cargo-watch

# Run tests in watch mode
make test-watch
```

#### 5. Docker Tests

Run tests inside a Docker container:

```bash
make test-docker
```

This builds the Docker image with the `tester` stage and runs all tests in an isolated environment.

## Test Environment Setup

### Prerequisites

1. **PostgreSQL Database**
   ```bash
   # Start with Docker Compose
   make dev
   
   # Or use your own PostgreSQL instance
   export DATABASE_URL="postgres://user:pass@localhost:5432/dbname"
   ```

2. **Run Migrations**
   ```bash
   make migration-run-local
   ```

3. **Set Environment Variables**
   ```bash
   export JWT_SECRET="test_secret_key"
   export DATABASE_URL="postgres://postgres:password@localhost:5432/task_manager"
   ```

### Test Database

Integration tests use the same database as development. Tests clean up data between runs by truncating tables.

**Important:** Don't run integration tests against a production database!

## Test Coverage

Current test coverage includes:

### Unit Tests (15 tests)
- ✅ Error handling and status codes
- ✅ JWT token creation and validation
- ✅ Model serialization/deserialization
- ✅ Request/Response DTOs

### Integration Tests (11 tests)
- ✅ User registration flow
- ✅ Authentication (login/logout)
- ✅ Task CRUD operations
- ✅ Authorization (user isolation)
- ✅ Error scenarios (wrong password, missing auth, etc.)

### Total: 26 automated tests

## Writing New Tests

### Adding Unit Tests

Create a new test file in the `tests/` directory:

```rust
// tests/my_feature_tests.rs
use task_manager::my_module::MyStruct;

#[test]
fn test_something() {
    // Your test code
    assert_eq!(1 + 1, 2);
}
```

### Adding Integration Tests

Add test functions to `tests/integration_tests.rs`:

```rust
#[tokio::test]
async fn test_new_feature() {
    let pool = setup_test_db().await;
    let app = create_test_app(pool).await;
    
    // Your test code
}
```

## Test Best Practices

1. **Isolation**: Each test should be independent and not rely on other tests
2. **Cleanup**: Integration tests clean up test data automatically
3. **Descriptive Names**: Use clear test names that describe what is being tested
4. **Arrange-Act-Assert**: Structure tests with clear setup, execution, and verification
5. **Edge Cases**: Test both success and failure scenarios

## Continuous Integration

Tests can be run in CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run tests
  run: |
    docker-compose up -d postgres
    cargo test
```

## Troubleshooting

### Database Connection Errors

```bash
# Check if PostgreSQL is running
make ps

# Start PostgreSQL
make dev

# Check database health
make health
```

### Migration Errors

```bash
# Reset database
make down-volumes
make dev
make migration-run-local
```

### Test Failures

```bash
# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests with debug output
RUST_LOG=debug cargo test
```

## Performance Testing

For performance and load testing, consider using:

- [k6](https://k6.io/) - Load testing tool
- [Apache Bench](https://httpd.apache.org/docs/2.4/programs/ab.html) - HTTP benchmarking
- [wrk](https://github.com/wg/wrk) - HTTP benchmarking tool

Example k6 script location: `tests/load-test.js` (to be created)

## Future Improvements

- [ ] Add code coverage reports
- [ ] Add benchmark tests
- [ ] Add mutation testing
- [ ] Add property-based testing
- [ ] Add performance tests
- [ ] Add E2E tests with real browser
- [ ] Add API contract tests
