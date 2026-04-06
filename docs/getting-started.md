# Getting Started

## Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Node.js 18+** - [Install Node.js](https://nodejs.org/)

## Installation

### 1. Build the CLI

```bash
git clone https://github.com/CARLOSMARES/structa.git
cd structa
cargo build --release
```

The binary will be at `target/release/structa.exe` (Windows) or `target/release/structa` (Linux/Mac).

### 2. Add to PATH (optional)

```bash
# Linux/Mac
sudo cp target/release/structa /usr/local/bin/

# Windows - Add to PATH or use full path
```

## Create Your First Project

### Initialize

```bash
structa init my-api
cd my-api
```

This creates:

```
my-api/
├── src/
│   └── main.structa
├── structa.config.json
├── package.json
└── README.md
```

### Install Dependencies

```bash
structa install
```

### Start Development Server

```bash
structa dev --port 3000
```

## Your First Controller

Edit `src/main.structa`:

```structa
controller UserController {
    path: "/users"
    
    @Get("/")
    async getAll(): User[] {
        return [
            { id: 1, name: "John", email: "john@example.com" },
            { id: 2, name: "Jane", email: "jane@example.com" }
        ]
    }
    
    @Get("/:id")
    async getById(id: string): User | null {
        if (id === "1") {
            return { id: 1, name: "John", email: "john@example.com" }
        }
        return null
    }
    
    @Post("/")
    async create(data: CreateUserDto): User {
        return { id: Math.random(), ...data }
    }
}

dto CreateUserDto {
    name: string
    email: string
}
```

## Add a Package

```bash
# Add ORM for database support
structa add @structa/orm

# Add validation
structa add @structa/validation

# Add cache
structa add @structa/cache
```

## Build for Production

```bash
# Debug build
structa build

# Release build
structa build --release
```

## Project Structure

```
my-api/
├── src/
│   ├── main.structa        # Main application
│   ├── controllers/        # Controllers
│   ├── services/           # Business logic
│   ├── dto/                # Data transfer objects
│   └── middleware/          # Custom middleware
├── dist/                   # Compiled output
├── structa.config.json     # Framework config
└── package.json            # Node dependencies
```

## Next Steps

- [Learn the DSL Syntax](./dsl.md)
- [Explore HTTP Package](../packages/http.md)
- [Set up Database with ORM](../packages/orm.md)
