# Contributing to USSD Advanced

Thank you for your interest in contributing to USSD Advanced! This document provides guidelines and information for contributors.

## 🌟 Ways to Contribute

- **Code**: Implement new features, fix bugs, improve performance
- **Documentation**: Improve docs, add examples, write tutorials
- **Testing**: Write tests, report bugs, improve test coverage
- **Design**: Propose architecture improvements, design patterns
- **Community**: Help others, answer questions, share knowledge

## 🚀 Getting Started

### Prerequisites

- Rust 1.75+ with 2021 edition
- PostgreSQL 14+
- Redis 7+
- Docker & Docker Compose (optional but recommended)
- Git

### Setup Development Environment

```bash
# Clone the repository
git clone <repository-url>
cd ussd

# Copy environment configuration
cp .env.example .env

# Start dependencies
docker-compose up -d postgres redis

# Run database migrations
make migrate

# Run the application
make run

# Run tests
make test
```

## 📝 Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b bugfix/issue-number-description
```

### 2. Make Your Changes

- Write clean, idiomatic Rust code
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed

### 3. Code Quality Checks

Before committing, run:

```bash
# Format code
make fmt

# Run linter
make lint

# Run tests
make test

# Or run all checks
make pre-commit
```

### 4. Commit Your Changes

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git commit -m "feat: add user authentication"
git commit -m "fix: resolve session timeout issue"
git commit -m "docs: update API documentation"
git commit -m "test: add integration tests for transfers"
git commit -m "refactor: improve state machine performance"
```

**Commit Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear title and description
- Reference related issues
- List of changes
- Test coverage information

## 🧪 Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Arrange
        let input = "test";

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

### Async Tests

```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/integration_test.rs
use ussd_advanced::*;

#[tokio::test]
async fn test_full_ussd_flow() {
    // Test implementation
}
```

## 🏗️ Architecture Guidelines

### Project Structure

```
src/
├── api/           # API layer (HTTP, gRPC)
├── config/        # Configuration management
├── domain/        # Domain models
├── error/         # Error types
├── menu/          # Menu definitions
├── metrics/       # Metrics collection
├── middleware/    # HTTP middleware
├── plugin/        # Plugin system
├── service/       # Business logic
├── state/         # State machine
└── storage/       # Database & cache
```

### Design Principles

1. **Separation of Concerns**: Each module has a single responsibility
2. **Dependency Injection**: Pass dependencies explicitly
3. **Type Safety**: Leverage Rust's type system
4. **Error Handling**: Use `Result` types, never panic in production code
5. **Async First**: Use async/await for I/O operations
6. **Testability**: Write testable code with clear interfaces

### Code Style

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

- Use `snake_case` for functions and variables
- Use `PascalCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Document public APIs with `///` comments
- Keep functions small and focused
- Prefer composition over inheritance

### Error Handling

```rust
// Good ✓
pub async fn process_request(&self, req: Request) -> Result<Response> {
    let session = self.get_session(&req.session_id).await?;
    // ... rest of the logic
    Ok(response)
}

// Bad ✗
pub async fn process_request(&self, req: Request) -> Response {
    let session = self.get_session(&req.session_id).await.unwrap();
    // ... unwrap() can panic!
}
```

### Logging

Use structured logging:

```rust
use tracing::{info, warn, error, debug};

info!(session_id = %session.id, "Session created");
warn!(error = %e, "Failed to connect to database");
error!(user_id = %user.id, "Authentication failed");
debug!(state = ?session.state, "State transition");
```

## 📚 Documentation

### Code Documentation

```rust
/// Processes a USSD request and returns a response.
///
/// # Arguments
///
/// * `request` - The USSD request containing session info and user input
///
/// # Returns
///
/// Returns a `Result` containing the USSD response or an error.
///
/// # Examples
///
/// ```
/// let response = service.process_request(request).await?;
/// ```
pub async fn process_request(&self, request: UssdRequest) -> Result<UssdResponse> {
    // Implementation
}
```

### README and Docs

- Keep README.md up to date
- Add examples for new features
- Update ARCHITECTURE.md for architectural changes
- Add usage examples in `examples/`

## 🐛 Reporting Bugs

### Before Reporting

1. Check existing issues
2. Verify you're using the latest version
3. Try to reproduce the bug

### Bug Report Template

```markdown
**Description**
Clear description of the bug

**To Reproduce**
Steps to reproduce:
1. ...
2. ...

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Version: [e.g., 0.1.0]

**Additional Context**
Any other relevant information
```

## 💡 Feature Requests

### Feature Request Template

```markdown
**Problem**
What problem does this solve?

**Proposed Solution**
Describe your proposed solution

**Alternatives**
Other solutions you've considered

**Additional Context**
Any other relevant information
```

## 🔍 Code Review Process

### For Contributors

- Respond to review comments promptly
- Be open to feedback
- Make requested changes or discuss alternatives
- Keep PRs focused and small when possible

### For Reviewers

- Be respectful and constructive
- Focus on code quality and architecture
- Check for test coverage
- Verify documentation updates
- Test the changes locally

## 📋 Checklist for Pull Requests

- [ ] Code follows project style guidelines
- [ ] All tests pass (`make test`)
- [ ] New code has test coverage
- [ ] Documentation updated
- [ ] Commit messages follow conventions
- [ ] No compiler warnings
- [ ] Clippy lints pass
- [ ] Changes are backwards compatible (or breaking changes documented)

## 🎯 Areas for Contribution

### High Priority

- [ ] gRPC API implementation
- [ ] Advanced rate limiting strategies
- [ ] Circuit breaker pattern
- [ ] More comprehensive tests
- [ ] Performance benchmarks

### Medium Priority

- [ ] GraphQL API
- [ ] Webhook notifications
- [ ] Multi-tenancy support
- [ ] Advanced analytics
- [ ] A/B testing framework

### Good First Issues

- Documentation improvements
- Adding more menu examples
- Writing plugin examples
- Improving error messages
- Adding validation rules

## 🤝 Community

- Be respectful and inclusive
- Help others in discussions
- Share knowledge and learnings
- Follow the [Code of Conduct](CODE_OF_CONDUCT.md)

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## ❓ Questions?

If you have questions:
- Open a GitHub Discussion
- Check existing documentation
- Review architecture docs

Thank you for contributing to USSD Advanced! 🦀❤️
