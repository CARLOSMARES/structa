# @structa/orm

Lightweight ORM for Structa framework with support for MySQL, PostgreSQL, and SQLite.

## Installation

```bash
structa add @structa/orm
```

## Configuration

### structa.config.json

```json
{
  "database": {
    "type": "sqlite",
    "filename": "./data.db"
  }
}
```

### Supported Database Types

```javascript
// SQLite
{ "type": "sqlite", "filename": "./data.db" }

// MySQL
{ "type": "mysql", "host": "localhost", "port": 3306, "user": "root", "password": "password", "database": "mydb" }

// PostgreSQL
{ "type": "postgresql", "host": "localhost", "port": 5432, "user": "postgres", "password": "password", "database": "mydb" }
```

## Usage

### Define Entity

```javascript
import { Entity, Column, PrimaryGeneratedColumn, CreateDateColumn, UpdateDateColumn } from '@structa/orm';

@Entity('users')
class User {
  @PrimaryGeneratedColumn()
  id;

  @Column({ length: 100 })
  name;

  @Column({ unique: true })
  email;

  @Column()
  password;

  @Column({ default: true })
  active;

  @CreateDateColumn()
  createdAt;

  @UpdateDateColumn()
  updatedAt;
}

export { User };
```

### Repository Pattern

```javascript
import { DataSource } from '@structa/orm';
import { User } from './entities/User';

const dataSource = new DataSource({
  type: 'sqlite',
  database: './data.db',
  entities: [User],
  synchronize: true
});

await dataSource.initialize();

// Find all
const users = await dataSource.getRepository(User).find();

// Find one
const user = await dataSource.getRepository(User).findOne({
  where: { id: 1 }
});

// Create
const newUser = await dataSource.getRepository(User).save({
  name: 'John',
  email: 'john@example.com',
  password: 'hashed_password'
});

// Update
await dataSource.getRepository(User).update(1, { name: 'Jane' });

// Delete
await dataSource.getRepository(User).delete(1);
```

### Query Builder

```javascript
const users = await dataSource
  .createQueryBuilder(User, 'user')
  .select(['user.id', 'user.name', 'user.email'])
  .where('user.active = :active', { active: true })
  .orderBy('user.createdAt', 'DESC')
  .limit(10)
  .getMany();
```

## Column Types

| Type | Description |
|------|-------------|
| `int` | Integer |
| `bigint` | Big integer |
| `float` | Float |
| `double` | Double precision |
| `decimal(p,s)` | Decimal with precision |
| `varchar(l)` | String with length |
| `text` | Long text |
| `boolean` | Boolean |
| `date` | Date |
| `datetime` | Date and time |
| `timestamp` | Timestamp |
| `json` | JSON object |
| `uuid` | UUID |

## Column Options

```javascript
@Column({
  type: 'varchar',
  length: 255,
  nullable: false,
  default: 'unknown',
  unique: true,
  primary: false,
  select: true
})
name;
```

## Relations

### One-to-Many

```javascript
@Entity('users')
class User {
  @PrimaryGeneratedColumn()
  id;

  @OneToMany(() => Post, (post) => post.author)
  posts;
}

@Entity('posts')
class Post {
  @PrimaryGeneratedColumn()
  id;

  @Column()
  title;

  @ManyToOne(() => User, (user) => user.posts)
  author;
}
```

### Many-to-Many

```javascript
@Entity('posts')
class Post {
  @PrimaryGeneratedColumn()
  id;

  @ManyToMany(() => Tag)
  @JoinTable()
  tags;
}

@Entity('tags')
class Tag {
  @PrimaryGeneratedColumn()
  id;

  @Column()
  name;
}
```

## Migrations

### Create Migration

```bash
structa orm migrate --create add_users_table
```

### Migration File

```javascript
export default class AddUsersTable1701234567890 {
  async up(dataSource) {
    await dataSource.query(`
      CREATE TABLE users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT UNIQUE NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `);
  }

  async down(dataSource) {
    await dataSource.query('DROP TABLE users');
  }
}
```

### Run Migrations

```bash
structa orm migrate
```

### Rollback

```bash
structa orm migrate --rollback
```

## CLI Commands

```bash
# Run migrations
structa orm migrate

# Create migration
structa orm migrate --create migration_name

# Rollback last migration
structa orm migrate --rollback

# Sync schema
structa orm db update

# Drop all tables
structa orm db drop --force

# Generate SQL
structa orm db sql --output schema.sql
```
