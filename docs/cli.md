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

**Examples:**
```bash
structa init my-api
structa init api --template api
```

---

### dev

Run development server with hot reload.

```bash
structa dev [options]
```

**Options:**
- `-p, --port <port>` - Port to listen on (default: 3000)
- `--hot-reload` - Enable hot reload (default: true)

**Examples:**
```bash
structa dev
structa dev --port 8080
```

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
- `-p, --package <name>` - Install a specific package

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
structa generate dto CreateUserDto
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
- `--create <name>` - Create a new migration

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
