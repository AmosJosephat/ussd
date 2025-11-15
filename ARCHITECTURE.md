# 🏗️ USSD Advanced - Architecture Documentation

## Overview

This is a next-generation USSD (Unstructured Supplementary Service Data) application built with Rust, designed for high performance, scalability, and extensibility.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        Client Layer                          │
│          (Telecom Gateway / HTTP Clients / gRPC)             │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                     API Gateway Layer                        │
│                                                               │
│  ┌─────────────────────┐      ┌──────────────────────┐      │
│  │   HTTP/REST API     │      │    gRPC API          │      │
│  │   (Axum)            │      │    (Tonic)           │      │
│  └─────────────────────┘      └──────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                   Middleware Pipeline                        │
│                                                               │
│  Logging → Metrics → Rate Limiting → Authentication          │
│     ↓         ↓           ↓                ↓                 │
│  (Tracing) (Prometheus) (Governor)      (JWT)                │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                    Service Layer                             │
│                                                               │
│              ┌──────────────────────┐                        │
│              │   UssdService        │                        │
│              │  (Business Logic)    │                        │
│              └──────────────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                   Core Components                            │
│                                                               │
│  ┌──────────────┬──────────────┬───────────────────────┐    │
│  │ State        │  Session     │    Plugin             │    │
│  │ Machine      │  Manager     │    Registry           │    │
│  │              │              │                       │    │
│  │ • Menu Tree  │ • Lifecycle  │ • Handler Registry    │    │
│  │ • Navigation │ • Context    │ • Dynamic Loading     │    │
│  │ • Validation │ • Timeouts   │ • Business Logic      │    │
│  └──────────────┴──────────────┴───────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                   Storage Layer                              │
│                                                               │
│  ┌──────────────────────┐      ┌──────────────────────┐     │
│  │   Redis              │      │   PostgreSQL         │     │
│  │                      │      │                      │     │
│  │ • Session Store      │      │ • Users              │     │
│  │ • Distributed Cache  │      │ • Transactions       │     │
│  │ • Auto-expiry        │      │ • Analytics          │     │
│  └──────────────────────┘      └──────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                  Observability Layer                         │
│                                                               │
│  Structured Logging    Metrics (Prometheus)   Distributed    │
│     (Tracing)              OpenTelemetry         Tracing     │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. API Layer (`src/api/`)

**Responsibility**: Handle external requests

- **HTTP API** (Axum framework)
  - RESTful endpoints for USSD requests
  - JSON request/response
  - Health checks and metrics endpoints

- **gRPC API** (Tonic framework) [Planned]
  - High-performance RPC
  - Protocol buffers
  - Bidirectional streaming support

**Key Files**:
- `src/api/http.rs` - HTTP router and handlers
- `src/api/grpc.rs` - gRPC service implementation [TODO]

### 2. Service Layer (`src/service/`)

**Responsibility**: Core business logic orchestration

- **UssdService**: Main service coordinator
  - Request processing
  - Session lifecycle management
  - Plugin execution
  - State transitions

**Key Features**:
- Session creation and management
- Input validation
- Menu navigation
- Plugin handler invocation
- Error handling

### 3. State Machine (`src/state/`)

**Responsibility**: Menu navigation and state transitions

**Design Pattern**: Hierarchical State Machine

**Features**:
- Type-safe state transitions
- Menu tree navigation
- Input validation (regex, length, custom)
- Multi-language support
- Dynamic menu loading

**States**:
```rust
START → MENU_SELECTION → SUB_MENU → INPUT → RESPONSE → END
```

**Menu Types**:
- `Menu` - Display options for selection
- `Input` - Free text input with validation
- `Response` - Terminal state (ends session)
- `Dynamic` - Generated by plugins

### 4. Session Management (`src/storage/session_store.rs`)

**Responsibility**: Distributed session storage

**Implementation**: Redis-backed with automatic expiry

**Features**:
- Session creation with unique IDs
- Context storage (key-value pairs)
- Automatic timeout and cleanup
- Phone number indexing
- Concurrent session support

**Session Lifecycle**:
```
CREATE → ACTIVE → [UPDATES] → EXPIRED/ENDED → DELETED
```

### 5. Plugin System (`src/plugin/`)

**Responsibility**: Extensible business logic

**Design Pattern**: Trait-based plugin architecture

**Plugin Interface**:
```rust
#[async_trait]
pub trait Plugin: Send + Sync {
    async fn handle(&self, ctx: &PluginContext) -> PluginResult;
    fn name(&self) -> &str;
    async fn init(&self) -> Result<()>;
    async fn shutdown(&self) -> Result<()>;
}
```

**Built-in Plugins**:
- `BalanceCheckPlugin` - Check account balance
- `TransferPlugin` - Money transfer
- `HelpPlugin` - User assistance

**Custom Plugins**: Implement the `Plugin` trait for custom logic

### 6. Storage Layer

#### Redis (`RedisSessionStore`)
- **Purpose**: High-speed session storage
- **Features**:
  - Key-value store with TTL
  - Atomic operations
  - Connection pooling
  - Auto-expiry

#### PostgreSQL (`Database`)
- **Purpose**: Persistent data storage
- **Tables**:
  - `users` - User profiles
  - `transactions` - Transaction history
  - `analytics` - Aggregated data

**Schema**:
```sql
users (id, phone_number, name, email, language, status, timestamps)
transactions (id, user_id, session_id, type, amount, status, metadata, timestamp)
```

### 7. Middleware Pipeline (`src/middleware/`)

**Execution Order**:
1. **Logging** - Request/response logging
2. **Metrics** - Prometheus metrics collection
3. **Rate Limiting** - Request throttling
4. **Authentication** - JWT validation [Planned]
5. **Compression** - Response compression
6. **CORS** - Cross-origin support
7. **Timeout** - Request timeout handling

### 8. Configuration (`src/config/`)

**Features**:
- Environment variable support
- `.env` file loading
- Type-safe configuration
- Validation
- Defaults

**Configuration Sections**:
- `server` - HTTP/gRPC settings
- `database` - PostgreSQL connection
- `redis` - Redis connection
- `session` - Session timeouts
- `security` - JWT, rate limiting
- `observability` - Logging, tracing

### 9. Error Handling (`src/error.rs`)

**Error Types**:
- `SessionNotFound` / `SessionExpired`
- `InvalidInput` / `InvalidStateTransition`
- `DatabaseError` / `RedisError`
- `PluginError`
- `RateLimitExceeded`
- `AuthenticationFailed` / `AuthorizationFailed`

**Error Response Format**:
```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable message",
    "status": 400
  }
}
```

### 10. Observability

#### Structured Logging
- **Framework**: `tracing` + `tracing-subscriber`
- **Format**: JSON
- **Levels**: trace, debug, info, warn, error

#### Metrics
- **System**: Prometheus
- **Metrics**:
  - `ussd_requests_total` - Request counter
  - `ussd_request_duration_seconds` - Latency histogram
  - `ussd_sessions_total` - Session counter
  - `ussd_active_sessions` - Active sessions gauge
  - `ussd_errors_total` - Error counter
  - `ussd_plugin_executions_total` - Plugin calls
  - Database and Redis operation counters

#### Distributed Tracing
- **Framework**: OpenTelemetry
- **Export**: OTLP protocol
- **Features**: Span correlation, context propagation

## Data Flow

### Typical USSD Request Flow

```
1. Client sends USSD request → HTTP API
2. Middleware processes (logging, metrics, auth)
3. Router dispatches to UssdService
4. Service retrieves/creates session from Redis
5. Service gets/creates user from PostgreSQL
6. State Machine processes input
7. If needed, execute Plugin handler
8. Update session state in Redis
9. Return formatted response
10. Log metrics and traces
```

### Session Creation Flow

```
1. Receive request with new session_id
2. Create Session object
3. Store in Redis with TTL
4. Add to phone number index
5. Increment metrics counters
6. Return initial menu
```

### Plugin Execution Flow

```
1. State Machine identifies handler
2. Build PluginContext with session + input
3. Registry looks up plugin by name
4. Execute plugin.handle(ctx)
5. Record execution metrics
6. Return plugin response
```

## Technology Stack

### Core
- **Language**: Rust (2021 edition)
- **Async Runtime**: Tokio
- **Web Framework**: Axum
- **gRPC**: Tonic + Prost

### Storage
- **Database**: PostgreSQL 14+ (via SQLx)
- **Cache**: Redis 7+ (via redis-rs)

### Observability
- **Logging**: tracing, tracing-subscriber
- **Metrics**: Prometheus
- **Tracing**: OpenTelemetry

### Utilities
- **Serialization**: serde, serde_json
- **Error Handling**: thiserror, anyhow
- **Validation**: validator, regex
- **Config**: config, dotenvy

## Scalability Considerations

### Horizontal Scaling
- **Stateless Design**: All state in Redis/PostgreSQL
- **Load Balancing**: Multiple app instances
- **Database**: Read replicas, connection pooling
- **Redis**: Cluster mode, replication

### Performance Optimizations
- **Async I/O**: Non-blocking operations
- **Connection Pooling**: Reuse DB/Redis connections
- **Caching**: Redis for hot data
- **Compression**: Response gzip/brotli
- **Binary Protocol**: gRPC for high-throughput

### Monitoring & Alerts
- **Metrics**: Prometheus → Grafana
- **Logs**: Centralized logging (ELK/Loki)
- **Tracing**: Jaeger/Tempo
- **Alerts**: Alert Manager rules

## Security

### Authentication & Authorization
- JWT tokens for API access [Planned]
- Phone number verification
- Session validation

### Rate Limiting
- Per-IP rate limiting
- Per-user session limits
- Configurable thresholds

### Input Validation
- Regex pattern matching
- Length constraints
- Type safety
- SQL injection prevention (parameterized queries)

### Network Security
- HTTPS/TLS for all APIs
- CORS configuration
- Request size limits

## Deployment

### Docker
- Multi-stage build for minimal image size
- Non-root user
- Health checks
- Resource limits

### Docker Compose
- Full stack deployment
- Service dependencies
- Volume persistence
- Network isolation

### Production Considerations
- Use production-grade secrets management
- Enable mTLS for gRPC
- Configure proper rate limits
- Set up monitoring and alerting
- Use managed PostgreSQL/Redis
- Implement backup strategy

## Future Enhancements

1. **gRPC API Implementation**
2. **Advanced Analytics Dashboard**
3. **Machine Learning for Fraud Detection**
4. **Multi-tenancy Support**
5. **Webhook Notifications**
6. **A/B Testing Framework**
7. **Caching Layer Optimization**
8. **Circuit Breakers**
9. **Feature Flags**
10. **GraphQL API**

## Development Workflow

```bash
# Local development
make dev             # Run with hot-reload
make test            # Run tests
make lint            # Check code quality

# Docker development
make docker-up       # Start all services
make docker-logs     # View logs
make docker-down     # Stop services

# Production
make release         # Build optimized binary
make docker-build    # Build production image
```

## Contributing

1. Follow Rust best practices
2. Add tests for new features
3. Update documentation
4. Run `make pre-commit` before pushing
5. Use conventional commits

---

**Built with ❤️ and Rust 🦀**
