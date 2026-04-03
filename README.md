# Structa

A TypeScript framework for APIs built with Rust.

## Features

- **Rust Compiler**: Lexer → Parser → AST → Transformer → Codegen pipeline
- **TypeScript Runtime**: Lightweight runtime with no NestJS dependencies
- **Multiple Input Formats**: 
  - DSL propio minimalista
  - Decorators TypeScript
  - Schema-first (YAML/JSON)
- **Modular Architecture**: HTTP, WebSockets, GraphQL, Microservices, Guards, Interceptors

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Structa CLI                         │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                   Rust Compiler                          │
│  ┌─────────┐   ┌─────────┐   ┌───────────┐          │
│  │  Lexer  │ → │ Parser  │ → │Transformer│ → Codegen│
│  └─────────┘   └─────────┘   └───────────┘          │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│              TypeScript Output (Runtime)                 │
│  Controllers, Services, Guards, DTOs, etc.            │
└─────────────────────────────────────────────────────────┘
```

## Crates

| Crate | Description |
|-------|-------------|
| `structa-core` | DI Container, Plugin system, Server module |
| `structa-cli` | CLI with init, build, dev, generate commands |
| `structa-lexer` | Tokenizer for the DSL |
| `structa-ast` | AST node definitions |
| `structa-parser` | Recursive descent parser |
| `structa-transformer` | AST transformations |
| `structa-codegen` | TypeScript code generation |
| `structa-runtime` | TypeScript runtime library |

## DSL Syntax

```structa
// Service
service UserService {
    getUser(id: number)
    createUser(name: string)
}

// Controller
controller UserController "/users" {
    get "/:id" getUser
    post "/" createUser
}

// DTO
dto CreateUserDto {
    name: string
    email: string
    age?: number
}

// Guard
guard AuthGuard {
    canActivate(): boolean
}
```

## Getting Started

### Prerequisites

- Rust 1.70+
- Node.js 18+

### Installation

```bash
# Clone the repository
git clone https://github.com/CARLOSMARES/structa.git
cd structa

# Build the CLI
cargo build --release

# Install TypeScript runtime
cd runtime && npm install
```

### CLI Usage

```bash
# Initialize a new project
structa init my-project

# Build the project
structa build --source ./src --output ./dist

# Development mode with watch
structa dev --source ./src

# Generate a new component
structa generate service UserService
structa generate controller UserController
```

## Development

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## Project Structure

```
structa/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── structa-core/       # Core framework
│   ├── structa-cli/         # CLI application
│   ├── structa-lexer/      # Lexer
│   ├── structa-ast/         # AST definitions
│   ├── structa-parser/      # Parser
│   ├── structa-transformer/ # AST transformer
│   └── structa-codegen/    # Code generator
└── runtime/                 # TypeScript runtime
    └── src/
        ├── decorators/     # Runtime decorators
        ├── container.ts     # DI container
        ├── context.ts       # Request/Response context
        └── app.ts           # Application bootstrap
```

## License

MIT License - see [LICENSE](LICENSE)
