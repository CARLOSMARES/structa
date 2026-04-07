# Structa Framework

A TypeScript-like API framework built with Rust. Write `.structa` files that compile to JavaScript.

```
██████╗ ███████╗███████╗██╗███╗   ██╗██╗   ██╗███████╗
██╔══██╗██╔════╝██╔════╝██║████╗  ██║██║   ██║██╔════╝
██████╔╝█████╗  █████╗  ██║██╔██╗ ██║██║   ██║███████╗
██╔══██╗██╔══╝  ██╔══╝  ██║██║╚██╗██║██║   ██║╚════██║
██║  ██║███████╗███████╗██║██║ ╚████║╚██████╔╝███████║
╚═╝  ╚═╝╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
```

## Features

- **Rust Compiler** - Fast compilation of `.structa` files
- **Hot Reload** - Development server with auto-recompilation
- **Dependency Injection** - Built-in DI container
- **Modular Packages** - HTTP, ORM, Validation, Cache, Queue, Mail
- **Matrix-style CLI** - Beautiful terminal interface

## Quick Start

```bash
# Build CLI
cargo build --release

# Create project
structa init my-api
cd my-api

# Install dependencies
structa install

# Run development server
structa dev --port 3000
```

## CLI Commands

```bash
structa init <name>      # Initialize new project
structa dev [--port]     # Run development server
structa build [--release] # Build project
structa install          # Install dependencies
structa add <package>    # Add npm package
structa remove <package> # Remove package
structa generate <type> <name>  # Generate code
structa orm <command>    # Database operations
```

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

## DSL Syntax

```structa
controller UserController {
    path: "/users"
    
    @Inject("UserService")
    userService
    
    @Get("/")
    async getAll() {
        return await this.userService.findAll()
    }
    
    @Get("/:id")
    async getById(id) {
        return await this.userService.findById(id)
    }
    
    @Post("/")
    async create(data) {
        return await this.userService.create(data)
    }
}

service UserService {
    @Inject("UserRepository")
    userRepo
    
    async findAll() {
        return await this.userRepo.findAll()
    }
    
    async findById(id) {
        return await this.userRepo.findById(id)
    }
    
    async create(data) {
        return await this.userRepo.save(data)
    }
}

repository UserRepository {
    async findAll() {
        return [{ id: 1, name: "John", email: "john@example.com" }]
    }
    
    async findById(id) {
        return { id, name: "John", email: "john@example.com" }
    }
    
    async save(data) {
        return { id: Date.now(), ...data }
    }
}

dto CreateUserDto {
    name: string
    email: string
}
```

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
│  ┌─────────┐   ┌─────────┐   ┌───────────────────┐   │
│  │  Lexer  │ → │ Parser  │ → │ Code Generator     │   │
│  └─────────┘   └─────────┘   └───────────────────┘   │
└─────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────┐
│              JavaScript Output (Runtime)                 │
│  Controllers, Services, Repositories, DTOs              │
└─────────────────────────────────────────────────────────┘
```

## Project Structure

```
structa/
├── crates/
│   ├── structa-cli/        # CLI application
│   ├── structa-compiler/   # Rust compiler
│   └── structa-linker/     # Runtime generator
├── packages/               # npm packages
│   ├── http/
│   ├── orm/
│   ├── validation/
│   ├── cache/
│   ├── queue/
│   ├── mail/
│   ├── swagger/
│   ├── websockets/
│   ├── graphql/
│   └── testing/
└── docs/                  # Documentation
```

## Documentation

- [Getting Started](./docs/getting-started.md)
- [CLI Commands](./docs/cli.md)
- [DSL Syntax](./docs/dsl.md)
- [HTTP Package](./docs/packages/http.md)
- [ORM Package](./docs/packages/orm.md)
- [Validation Package](./docs/packages/validation.md)
- [Cache Package](./docs/packages/cache.md)
- [Queue Package](./docs/packages/queue.md)
- [Mail Package](./docs/packages/mail.md)

## License

MIT
