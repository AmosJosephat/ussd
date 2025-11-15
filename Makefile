.PHONY: help build run test clean docker-build docker-up docker-down migrate fmt lint check

# Default target
help:
	@echo "🦀 USSD Advanced - Makefile Commands"
	@echo ""
	@echo "Development:"
	@echo "  make build        - Build the project"
	@echo "  make run          - Run the application"
	@echo "  make test         - Run tests"
	@echo "  make check        - Run cargo check"
	@echo "  make fmt          - Format code"
	@echo "  make lint         - Run clippy linter"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build - Build Docker image"
	@echo "  make docker-up    - Start all services"
	@echo "  make docker-down  - Stop all services"
	@echo "  make docker-logs  - View logs"
	@echo ""
	@echo "Database:"
	@echo "  make migrate      - Run database migrations"
	@echo "  make db-reset     - Reset database"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean        - Clean build artifacts"

# Build
build:
	cargo build --release

# Run
run:
	cargo run

# Run in development mode
dev:
	cargo watch -x run

# Test
test:
	cargo test

# Test with output
test-verbose:
	cargo test -- --nocapture

# Integration tests
test-integration:
	cargo test --test '*'

# Check
check:
	cargo check

# Format
fmt:
	cargo fmt

# Lint
lint:
	cargo clippy -- -D warnings

# Clean
clean:
	cargo clean
	rm -rf target/

# Docker build
docker-build:
	docker build -t ussd-advanced:latest .

# Docker compose up
docker-up:
	docker-compose up -d

# Docker compose up with monitoring
docker-up-monitoring:
	docker-compose --profile monitoring up -d

# Docker compose down
docker-down:
	docker-compose down

# Docker logs
docker-logs:
	docker-compose logs -f ussd-app

# Run database migrations
migrate:
	@echo "Running migrations..."
	@docker-compose exec postgres psql -U ussd_user -d ussd_db -f /docker-entrypoint-initdb.d/001_init.sql

# Reset database
db-reset:
	docker-compose down -v
	docker-compose up -d postgres
	sleep 3
	make migrate

# Install dependencies
deps:
	cargo fetch

# Update dependencies
update:
	cargo update

# Run benchmarks
bench:
	cargo bench

# Generate documentation
docs:
	cargo doc --no-deps --open

# Security audit
audit:
	cargo audit

# Run all checks before commit
pre-commit: fmt lint test
	@echo "✅ All checks passed!"

# Production build
release:
	cargo build --release --locked

# Install cargo tools
install-tools:
	cargo install cargo-watch
	cargo install cargo-audit
	cargo install sqlx-cli
