# 🚀 Advanced USSD Application - Next Generation Platform

A cutting-edge, production-grade USSD (Unstructured Supplementary Service Data) application built with Rust, featuring distributed session management, plugin architecture, and full observability.

## 🌟 Key Features

### Core Capabilities
- **⚡ High Performance**: Async-first architecture using Tokio runtime
- **🔄 Distributed Sessions**: Redis-backed session management for horizontal scaling
- **🎯 Type-Safe State Machine**: Zero-cost abstractions with compile-time guarantees
- **🔌 Plugin Architecture**: Hot-loadable business logic modules
- **📊 Full Observability**: Distributed tracing, metrics, and structured logging
- **🌐 Multi-Protocol**: gRPC and REST API support
- **🗄️ Robust Persistence**: PostgreSQL with async SQLx
- **🛡️ Production-Ready**: Rate limiting, auth, validation, error handling

### Advanced Features
- Session timeout and automatic cleanup
- Multi-language menu support
- Context-aware menu navigation
- Business logic isolation via plugins
- Prometheus metrics export
- OpenTelemetry distributed tracing
- JWT-based authentication
- Request rate limiting
- Comprehensive middleware pipeline
- Docker-ready deployment

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│            HTTP/gRPC API Gateway                     │
│         (Axum/Tonic - Multi-protocol)                │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│              Middleware Pipeline                     │
│   Auth • Logging • Rate Limiting • Validation        │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│           USSD Session Manager                       │
│    State Machine • Context • Lifecycle               │
└─────────────────────────────────────────────────────┘
                        ↓
┌──────────────┬────────────────┬─────────────────────┐
│   Menu       │    Session     │    Plugin           │
│   Engine     │    Store       │    System           │
│              │   (Redis)      │    (Business)       │
└──────────────┴────────────────┴─────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│         Data Layer (PostgreSQL/SQLx)                 │
│    Users • Transactions • Analytics • Audit          │
└─────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+ (2021 edition)
- PostgreSQL 14+
- Redis 7+
- Docker & Docker Compose (optional)

### Development Setup

```bash
# Clone repository
git clone <repository-url>
cd ussd

# Set up environment
cp .env.example .env

# Start dependencies (Redis, PostgreSQL)
docker-compose up -d

# Run database migrations
cargo sqlx migrate run

# Build and run
cargo build --release
cargo run --release
```

### Configuration

Create `.env` file:

```env
# Server
HOST=0.0.0.0
PORT=8080
GRPC_PORT=50051

# Database
DATABASE_URL=postgresql://user:password@localhost/ussd_db

# Redis
REDIS_URL=redis://localhost:6379

# Session
SESSION_TIMEOUT_SECS=300
MAX_SESSIONS_PER_USER=5

# Security
JWT_SECRET=your-secret-key-here
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW_SECS=60

# Observability
LOG_LEVEL=info
OTLP_ENDPOINT=http://localhost:4317
```

## 📡 API Usage

### REST API

```bash
# Start USSD session
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "unique-session-id",
    "phone_number": "+1234567890",
    "input": "",
    "service_code": "*123#"
  }'

# Continue session
curl -X POST http://localhost:8080/api/v1/ussd \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "unique-session-id",
    "phone_number": "+1234567890",
    "input": "1",
    "service_code": "*123#"
  }'
```

### gRPC API

See `proto/ussd.proto` for service definitions.

## 🔌 Plugin Development

Create custom business logic plugins:

```rust
use ussd_advanced::plugin::{Plugin, PluginContext, PluginResult};
use async_trait::async_trait;

pub struct MyCustomPlugin;

#[async_trait]
impl Plugin for MyCustomPlugin {
    async fn handle(&self, ctx: &PluginContext) -> PluginResult {
        // Your business logic here
        Ok("Response message".to_string())
    }

    fn name(&self) -> &str {
        "my_custom_plugin"
    }
}
```

## 📊 Monitoring

### Metrics

Prometheus metrics available at: `http://localhost:8080/metrics`

Key metrics:
- `ussd_sessions_total` - Total sessions created
- `ussd_requests_duration_seconds` - Request latency
- `ussd_active_sessions` - Current active sessions
- `ussd_errors_total` - Error counts by type

### Tracing

Distributed traces exported to OpenTelemetry collector.

### Logging

Structured JSON logs with configurable levels.

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*'

# Run benchmarks
cargo bench

# Code coverage
cargo tarpaulin --out Html
```

## 🐳 Docker Deployment

```bash
# Build image
docker build -t ussd-advanced .

# Run with docker-compose
docker-compose up -d
```

## 📚 Project Structure

```
ussd/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config/              # Configuration management
│   ├── api/                 # HTTP & gRPC APIs
│   ├── domain/              # Business domain models
│   ├── session/             # Session management
│   ├── state/               # State machine
│   ├── menu/                # Menu engine
│   ├── middleware/          # HTTP middleware
│   ├── plugin/              # Plugin system
│   ├── storage/             # Database & Redis
│   ├── metrics/             # Prometheus metrics
│   └── error/               # Error handling
├── migrations/              # Database migrations
├── proto/                   # gRPC proto files
├── plugins/                 # Plugin implementations
├── tests/                   # Integration tests
├── benches/                 # Benchmarks
└── docker/                  # Docker configs
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing`)
5. Open Pull Request

## 📝 License

MIT License - see LICENSE file for details

## 🙏 Acknowledgments

Built with incredible Rust ecosystem:
- [Tokio](https://tokio.rs) - Async runtime
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL
- [Redis-rs](https://github.com/redis-rs/redis-rs) - Redis client
- [Tonic](https://github.com/hyperium/tonic) - gRPC framework

---

**Built with ❤️ and Rust 🦀**
