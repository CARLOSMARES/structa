# CLI Commands

Structa CLI provides commands for project management, development, and building.

## Usage

```bash
structa <command> [options]
```

## Commands

### init

Initialize a new Structa project.

```bash
structa init [name] [options]
```

**Options:**
- `-p, --path <path>` - Project directory (default: current directory)
- `-t, --template <template>` - Template to use (default: api)
- `-n, --name <name>` - Project name (default: my-api)

**Examples:**
```bash
structa init my-api
structa init api --template api
structa init --path ./projects/myapp
```

---

### dev

Run development server with hot reload.

```bash
structa dev [options]
```

**Options:**
- `-p, --port <port>` - Port to listen on (default: 3000)
- `--hot-reload` - Enable hot reload (default: enabled)

**Examples:**
```bash
structa dev
structa dev --port 8080
```

The dev server:
- Compiles `.structa` files to JavaScript
- Watches for file changes (300ms debounce)
- Automatically restarts the server
- Outputs colored Matrix-style logs

---

### build

Build the project to JavaScript.

```bash
structa build [options]
```

**Options:**
- `-r, --release` - Build in release mode
- `-o, --output <path>` - Output directory (default: dist)

**Examples:**
```bash
structa build
structa build --release
structa build --release -o dist
```

---

### install

Install dependencies from package.json.

```bash
structa install [options]
```

**Options:**
- `-p, --package <name>` - Install a specific package and add to dependencies

**Examples:**
```bash
structa install
structa install --package @structa/orm
```

---

### add

Add and install a package from npm.

```bash
structa add <package> [options]
```

**Options:**
- `-d, --dev` - Install as dev dependency
- `-g, --global` - Install globally

**Examples:**
```bash
structa add @structa/orm
structa add lodash
structa add typescript --dev
structa add @structa/cache --global
```

---

### remove

Remove a package from the project.

```bash
structa remove <package>
```

**Examples:**
```bash
structa remove lodash
structa remove @structa/orm
```

---

### generate

Generate code files (controllers, services, etc.).

```bash
structa generate <type> <name> [options]
```

**Types:**
- `controller` - HTTP controller
- `service` - Business logic service
- `repository` - Data access layer
- `module` - Module definition
- `middleware` - Custom middleware
- `guard` - Route guard
- `resolver` - GraphQL resolver
- `gateway` - WebSocket gateway
- `dto` - Data transfer object
- `entity` - Database entity
- `route` - API route definition

**Options:**
- `-p, --path <path>` - Output directory (default: src)

**Examples:**
```bash
structa generate controller User
structa generate service UserService
structa generate repository UserRepository
structa generate dto CreateUserDto
structa generate middleware Logger
structa generate module UserModule
structa generate entity Product --path src/models
```

---

### orm

Database operations via ORM.

```bash
structa orm <subcommand>
```

**Subcommands:**

#### migrate

Run pending database migrations.

```bash
structa orm migrate [options]
```

**Options:**
- `-c, --connection <string>` - Connection string
- `--rollback` - Revert last migration
- `--create <name>` - Create a new migration file

**Examples:**
```bash
structa orm migrate
structa orm migrate --create add_users_table
structa orm migrate --rollback
```

#### db

Database schema operations.

```bash
structa orm db <action>
```

**Actions:**
- `update` - Sync entities to database
- `drop` - Drop all tables
- `sql` - Generate SQL schema

**Examples:**
```bash
structa orm db update
structa orm db drop --force
structa orm db sql --output schema.sql
```

---

## Global Options

- `-v, --verbose` - Enable verbose logging
- `-h, --help` - Show help for a command
- `-V, --version` - Show version number

---

## Configuration

The CLI reads configuration from `structa.config.json`:

```json
{
  "name": "my-api",
  "version": "1.0.0",
  "port": 3000,
  "database": {
    "type": "sqlite",
    "filename": "./data.db"
  }
}
```

---

## Package Management Examples

```bash
# Start a new project
structa init my-api
cd my-api

# Install base dependencies
structa install

# Add Structa packages
structa add @structa/orm
structa add @structa/validation
structa add @structa/cache

# Add common npm packages
structa add lodash
structa add uuid
structa add bcrypt --save

# Add dev dependencies
structa add typescript --dev
structa add jest --dev

# Run development
structa dev --port 3000

# Build for production
structa build --release
```

---

## Workflow Example

```bash
# 1. Create new project
structa init todo-api
cd todo-api

# 2. Install dependencies
structa install

# 3. Add ORM
structa add @structa/orm

# 4. Generate components
structa generate controller Task
structa generate service TaskService
structa generate repository TaskRepository
structa generate dto CreateTaskDto

# 5. Create database migration
structa orm migrate --create tasks

# 6. Run development server
structa dev --port 3000

# 7. Build for production
structa build --release
```
