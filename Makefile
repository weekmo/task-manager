.PHONY: help build up down restart logs test clean migration-run dev

# Variables
DOCKER_COMPOSE = sudo docker-compose
DOCKER = sudo docker

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build Docker images
	$(DOCKER_COMPOSE) build

build-no-cache: ## Build Docker images without cache
	$(DOCKER_COMPOSE) build --no-cache

up: ## Start all services (postgres + app)
	$(DOCKER_COMPOSE) up -d

up-build: build up ## Build and start all services

down: ## Stop all services
	$(DOCKER_COMPOSE) down

down-volumes: ## Stop all services and remove volumes (deletes database)
	$(DOCKER_COMPOSE) down -v

restart: ## Restart all services
	$(DOCKER_COMPOSE) restart

restart-app: ## Restart only the app service
	$(DOCKER_COMPOSE) restart app

logs: ## Show logs for all services
	$(DOCKER_COMPOSE) logs -f

logs-app: ## Show logs for app service only
	$(DOCKER_COMPOSE) logs -f app

logs-db: ## Show logs for postgres service only
	$(DOCKER_COMPOSE) logs -f postgres

test: ## Run all tests locally (requires DB for integration tests)
	@echo "==================================="
	@echo "Running Task Manager Tests"
	@echo "==================================="
	@echo ""
	@echo "Checking database connection..."
	@psql "postgres://postgres:password@localhost:5432/task_manager" -c "SELECT 1" > /dev/null 2>&1 || \
		echo "⚠️  Warning: Cannot connect to database. Integration tests may fail. Run 'make dev' first."
	@echo ""
	JWT_SECRET=test_secret_key DATABASE_URL=postgres://postgres:password@localhost:5432/task_manager cargo test --quiet -- --test-threads=1
	@echo ""
	@echo "==================================="
	@echo "✅ All tests passed!"
	@echo "==================================="

test-unit: ## Run only unit tests (no DB required)
	@echo "Running unit tests (no database required)..."
	JWT_SECRET=test_secret_key cargo test --test error_tests --test auth_tests --test user_model_tests --test task_model_tests

test-integration: ## Run only integration tests (requires DB)
	@echo "Running integration tests (requires database)..."
	@echo "Make sure database is running: make dev"
	JWT_SECRET=test_secret_key DATABASE_URL=postgres://postgres:password@localhost:5432/task_manager cargo test --test integration_tests -- --test-threads=1

test-all: ## Run all tests with verbose output
	JWT_SECRET=test_secret_key DATABASE_URL=postgres://postgres:password@localhost:5432/task_manager cargo test -- --test-threads=1

test-docker: ## Run tests in Docker
	$(DOCKER) build --target tester -t task-manager-test .

test-watch: ## Run tests in watch mode
	cargo watch -x test

dev: ## Start services for local development (postgres only)
	$(DOCKER_COMPOSE) up -d postgres

stop-dev: ## Stop development services
	$(DOCKER_COMPOSE) stop postgres

migration-run: ## Run database migrations (requires app to be running)
	$(DOCKER_COMPOSE) exec app sqlx migrate run

migration-run-local: ## Run database migrations locally
	sqlx migrate run

shell-app: ## Open shell in app container
	$(DOCKER_COMPOSE) exec app /bin/sh

shell-db: ## Open PostgreSQL shell
	$(DOCKER_COMPOSE) exec postgres psql -U postgres -d task_manager

clean: down-volumes ## Clean up Docker resources
	$(DOCKER) system prune -f

ps: ## Show running containers
	$(DOCKER_COMPOSE) ps

health: ## Check health of services
	@echo "Checking postgres health..."
	@$(DOCKER_COMPOSE) exec postgres pg_isready -U postgres || echo "Postgres not ready"
	@echo ""
	@echo "Checking app health..."
	@curl -s http://localhost:3000 > /dev/null && echo "App is running" || echo "App is not responding"

rebuild: down build up ## Rebuild and restart all services
