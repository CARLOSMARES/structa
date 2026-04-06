# Structa Framework

A TypeScript-like API framework built with Rust. Write `.structa` files that compile to JavaScript with a lightweight runtime.

## Quick Start

```bash
# Install Structa CLI
cargo build --release

# Create a new project
structa init my-api
cd my-api

# Install dependencies
structa install

# Run development server
structa dev --port 3000
```

## Features

- **Rust Compiler** - Fast compilation of `.structa` files to JavaScript
- **TypeScript-like Syntax** - Familiar decorators and decorators
- **Hot Reload** - Development server with automatic recompilation
- **Modular Packages** - HTTP, ORM, Validation, Cache, Queue, Mail, etc.
- **Matrix-style CLI** - Beautiful terminal interface

## Packages

| Package | Description |
|---------|-------------|
| `@structa/http` | HTTP server with routing and middleware |
| `@structa/orm` | Database ORM (MySQL, PostgreSQL, SQLite) |
| `@structa/validation` | Input validation with decorators |
| `@structa/cache` | Caching (Memory, Redis, File) |
| `@structa/queue` | Job queues with retry support |
| `@structa/mail` | Email sending (SMTP, SendGrid) |
| `@structa/swagger` | OpenAPI documentation |
| `@structa/websockets` | WebSocket support |
| `@structa/graphql` | GraphQL integration |
| `@structa/testing` | Testing utilities |

## Documentation

- [Getting Started](./getting-started.md)
- [CLI Commands](./cli.md)
- [DSL Syntax](./dsl.md)
- [HTTP Package](./packages/http.md)
- [ORM Package](./packages/orm.md)
- [Validation Package](./packages/validation.md)
- [Cache Package](./packages/cache.md)
- [Queue Package](./packages/queue.md)
- [Mail Package](./packages/mail.md)

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Structa CLI                         │
│  init | dev | build | generate | install | orm         │
└─────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────┐
│                   Rust Compiler                          │
│  ┌─────────┐   ┌─────────┐   ┌───────────────────┐    │
│  │  Lexer  │ → │ Parser  │ → │ Code Generator    │    │
│  └─────────┘   └─────────┘   └───────────────────┘    │
└─────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────┐
│              JavaScript Output (Runtime)                 │
│  Controllers, Services, Middleware, DTOs                  │
└─────────────────────────────────────────────────────────┘
```

## License

MIT
