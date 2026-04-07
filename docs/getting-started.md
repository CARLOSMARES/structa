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
        return [
            { id: 1, name: "John", email: "john@example.com" },
            { id: 2, name: "Jane", email: "jane@example.com" }
        ]
    }
    
    async findById(id) {
        return { id, name: "John", email: "john@example.com" }
    }
    
    async save(data) {
        return { id: Date.now(), ...data }
    }
}
```

## CLI Commands

### Development

```bash
# Start dev server with hot reload
structa dev --port 3000

# Build for production
structa build --release
```

### Code Generation

```bash
# Generate components
structa generate controller User
structa generate service UserService
structa generate repository UserRepository
structa generate dto CreateUserDto
structa generate middleware Logger
```

### Package Management

```bash
# Install dependencies
structa install

# Add a package
structa add @structa/orm
structa add lodash

# Remove a package
structa remove lodash
```

### Database (ORM)

```bash
# Run migrations
structa orm migrate

# Create migration
structa orm migrate --create add_users

# Rollback last migration
structa orm migrate --rollback

# Sync schema
structa orm db update

# Drop all tables
structa orm db drop --force
```

## Project Structure

```
my-api/
├── src/
│   ├── main.structa        # Main application
│   ├── controllers/        # Controllers
│   ├── services/           # Business logic
│   ├── repositories/       # Data access
│   ├── dto/                # Data transfer objects
│   └── middleware/         # Custom middleware
├── dist/                   # Compiled output
├── structa.config.json     # Framework config
└── package.json            # Node dependencies
```

## Add Packages

```bash
# Add ORM for database support
structa add @structa/orm

# Add validation
structa add @structa/validation

# Add cache
structa add @structa/cache

# Add queue
structa add @structa/queue

# Add mail
structa add @structa/mail
```

## Next Steps

- [Learn the DSL Syntax](./dsl.md)
- [Explore CLI Commands](./cli.md)
- [Set up Database with ORM](./packages/orm.md)
