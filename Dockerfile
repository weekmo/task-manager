# ============================================
# Builder Stage - Build the Rust application
# ============================================
FROM rustlang/rust:nightly-bookworm-slim AS builder

# Install required dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy all source code
COPY . .

# Build the application
RUN cargo build --release

# Install sqlx-cli for migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

# ============================================
# Test Stage - Run tests
# ============================================
FROM builder AS tester

# Run tests
RUN cargo test --release

# ============================================
# Runtime Stage - Create minimal image
# ============================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/task-manager /app/task-manager

# Copy sqlx-cli for migrations
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Copy migrations directory
COPY --from=builder /app/migrations /app/migrations

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose the application port
EXPOSE 3000

# Set environment variables (will be overridden by docker-compose)
ENV DATABASE_URL="postgres://postgres:password@postgres:5432/task_manager"
ENV JWT_SECRET="your_super_secret_key_change_this"

# Run migrations and start application
CMD ["/bin/sh", "-c", "echo 'Running database migrations...' && sqlx migrate run || echo 'Migration failed (tables may already exist)' && echo 'Starting application...' && /app/task-manager"]
