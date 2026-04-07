#!/usr/bin/env node

import { readFileSync, existsSync, writeFileSync, readdirSync, mkdirSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const MIGRATIONS_DIR = 'migrations';
const CONFIG_FILE = 'structa.config.json';

const COLORS = {
  reset: '\x1b[0m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  red: '\x1b[31m',
  cyan: '\x1b[36m',
  dim: '\x1b[2m',
};

function log(message, color = 'reset') {
  console.log(`${COLORS[color]}${message}${COLORS.reset}`);
}

function loadConfig() {
  if (!existsSync(CONFIG_FILE)) {
    log(`Config file '${CONFIG_FILE}' not found`, 'red');
    process.exit(1);
  }
  
  try {
    const config = JSON.parse(readFileSync(CONFIG_FILE, 'utf-8'));
    return config.database || config;
  } catch (e) {
    log(`Failed to parse ${CONFIG_FILE}: ${e.message}`, 'red');
    process.exit(1);
  }
}

async function createDataSource(config) {
  const { type } = config;
  
  switch (type) {
    case 'mysql':
    case 'mariadb': {
      const mysql = await import('mysql2/promise');
      return mysql.createPool(config);
    }
    case 'postgresql': {
      const { Client } = await import('pg');
      const client = new Client(config);
      await client.connect();
      return client;
    }
    case 'sqlite': {
      const Database = (await import('better-sqlite3')).default;
      return new Database(config.filename || 'database.sqlite');
    }
    default:
      log(`Unsupported database type: ${type}`, 'red');
      process.exit(1);
  }
}

async function executeQuery(conn, type, sql, params = []) {
  switch (type) {
    case 'mysql':
    case 'mariadb': {
      const [rows] = await conn.execute(sql, params);
      return rows;
    }
    case 'postgresql': {
      const result = await conn.query(sql, params);
      return result.rows;
    }
    case 'sqlite': {
      if (sql.trim().toUpperCase().startsWith('SELECT')) {
        return conn.prepare(sql).all(...params);
      }
      return conn.prepare(sql).run(...params);
    }
  }
}

async function runMigrations() {
  log('Running migrations...', 'cyan');
  
  const config = loadConfig();
  const conn = await createDataSource(config);
  
  await executeQuery(conn, config.type, `
    CREATE TABLE IF NOT EXISTS structa_migrations (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL UNIQUE,
      executed_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
  `);
  
  const executed = await executeQuery(conn, config.type, 
    'SELECT name FROM structa_migrations'
  );
  const executedNames = executed.map(r => r.name);
  
  if (!existsSync(MIGRATIONS_DIR)) {
    mkdirSync(MIGRATIONS_DIR, { recursive: true });
    log(`Created ${MIGRATIONS_DIR}/ directory`, 'green');
  }
  
  const files = readdirSync(MIGRATIONS_DIR)
    .filter(f => f.endsWith('.js'))
    .sort();
  
  let ran = 0;
  for (const file of files) {
    if (executedNames.includes(file)) continue;
    
    log(`  Running: ${file}`, 'yellow');
    
    try {
      const migration = await import(`./${MIGRATIONS_DIR}/${file}`);
      const instance = new migration.default();
      
      if (instance.dataSource === undefined) {
        instance.dataSource = { conn, type: config.type };
      } else {
        instance.dataSource.conn = conn;
        instance.dataSource.type = config.type;
      }
      
      await instance.up();
      await executeQuery(conn, config.type, 
        'INSERT INTO structa_migrations (name) VALUES (?)',
        [file]
      );
      
      log(`  Completed: ${file}`, 'green');
      ran++;
    } catch (e) {
      log(`  Failed: ${file} - ${e.message}`, 'red');
      throw e;
    }
  }
  
  if (ran === 0) {
    log('No pending migrations', 'dim');
  } else {
    log(`Ran ${ran} migration(s)`, 'green');
  }
  
  await closeConnection(conn, config.type);
}

async function revertMigration() {
  log('Reverting last migration...', 'cyan');
  
  const config = loadConfig();
  const conn = await createDataSource(config);
  
  const executed = await executeQuery(conn, config.type, 
    'SELECT name FROM structa_migrations ORDER BY id DESC LIMIT 1'
  );
  
  if (executed.length === 0) {
    log('No migrations to revert', 'dim');
    await closeConnection(conn, config.type);
    return;
  }
  
  const lastMigration = executed[0].name;
  log(`Reverting: ${lastMigration}`, 'yellow');
  
  try {
    const migration = await import(`./${MIGRATIONS_DIR}/${lastMigration}`);
    const instance = new migration.default();
    
    if (instance.dataSource === undefined) {
      instance.dataSource = { conn, type: config.type };
    } else {
      instance.dataSource.conn = conn;
      instance.dataSource.type = config.type;
    }
    
    await instance.down();
    await executeQuery(conn, config.type, 
      'DELETE FROM structa_migrations WHERE name = ?',
      [lastMigration]
    );
    
    log('Migration reverted', 'green');
  } catch (e) {
    log(`Failed to revert: ${e.message}`, 'red');
    throw e;
  }
  
  await closeConnection(conn, config.type);
}

async function createMigration(name) {
  if (!existsSync(MIGRATIONS_DIR)) {
    mkdirSync(MIGRATIONS_DIR, { recursive: true });
  }
  
  const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
  const filename = `${timestamp}_${name}.js`;
  const filepath = join(MIGRATIONS_DIR, filename);
  
  const template = `export default class ${toPascalCase(name)}Migration {
  dataSource = undefined;
  
  get name() {
    return '${filename}';
  }
  
  async up() {
    // Add your migration logic here
    // Example:
    // await this.dataSource.conn.execute(\`
    //   CREATE TABLE users (
    //     id INTEGER PRIMARY KEY AUTOINCREMENT,
    //     email TEXT NOT NULL UNIQUE,
    //     password TEXT NOT NULL,
    //     created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    //   )
    // \`);
  }
  
  async down() {
    // Add rollback logic here
    // Example:
    // await this.dataSource.conn.execute('DROP TABLE IF EXISTS users');
  }
}

function toPascalCase(str) {
  return str
    .split(/[-_]/)
    .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join('');
}
`;
  
  writeFileSync(filepath, template);
  log(`Created migration: ${filepath}`, 'green');
}

async function updateSchema() {
  log('Updating database schema...', 'cyan');
  
  const config = loadConfig();
  const conn = await createDataSource(config);
  
  if (!existsSync('src')) {
    log('No src/ directory found', 'dim');
    await closeConnection(conn, config.type);
    return;
  }
  
  const files = readdirSync('src', { recursive: true })
    .filter(f => f.endsWith('.structa'));
  
  let tableCount = 0;
  for (const file of files) {
    const content = readFileSync(join('src', file), 'utf-8');
    const entityMatch = content.match(/@Entity\s*\(\s*['"]([^'"]+)['"]\s*\)/);
    
    if (entityMatch) {
      const tableName = entityMatch[1];
      log(`  Syncing entity: ${tableName}`, 'yellow');
      tableCount++;
    }
  }
  
  if (tableCount === 0) {
    log('No entities found', 'dim');
  } else {
    log(`Synced ${tableCount} entity/entities`, 'green');
  }
  
  await closeConnection(conn, config.type);
}

async function dropSchema() {
  log('Dropping all tables...', 'cyan');
  
  const config = loadConfig();
  const conn = await createDataSource(config);
  
  const tables = await executeQuery(conn, config.type,
    config.type === 'postgresql'
      ? "SELECT tablename FROM pg_tables WHERE schemaname = 'public'"
      : "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
  );
  
  const tableNames = tables.map(t => t.tablename || t.name);
  
  for (const table of tableNames) {
    if (table === 'structa_migrations') continue;
    log(`  Dropping: ${table}`, 'yellow');
    await executeQuery(conn, config.type, `DROP TABLE IF EXISTS "${table}"`);
  }
  
  log('All tables dropped', 'green');
  await closeConnection(conn, config.type);
}

async function generateSql() {
  log('Generating SQL schema...', 'cyan');
  
  const config = loadConfig();
  const conn = await createDataSource(config);
  
  const tables = await executeQuery(conn, config.type,
    config.type === 'postgresql'
      ? "SELECT tablename FROM pg_tables WHERE schemaname = 'public'"
      : "SELECT name FROM sqlite_master WHERE type='table'"
  );
  
  let sql = '-- Generated SQL Schema\n\n';
  
  for (const table of tables) {
    const tableName = table.tablename || table.name;
    if (tableName === 'structa_migrations') continue;
    
    sql += `-- Table: ${tableName}\n`;
    sql += `CREATE TABLE IF NOT EXISTS "${tableName}" (\n`;
    sql += `  -- TODO: Define columns\n`;
    sql += `);\n\n`;
  }
  
  console.log(sql);
  await closeConnection(conn, config.type);
}

async function closeConnection(conn, type) {
  if (type === 'postgresql') {
    await conn.end();
  } else if (type === 'mysql' || type === 'mariadb') {
    await conn.end();
  } else {
    conn.close();
  }
}

function toPascalCase(str) {
  return str
    .split(/[-_]/)
    .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join('');
}

const command = process.argv[2];

switch (command) {
  case 'migrate':
    runMigrations().catch(e => {
      log(e.message, 'red');
      process.exit(1);
    });
    break;
    
  case 'migrate:revert':
    revertMigration().catch(e => {
      log(e.message, 'red');
      process.exit(1);
    });
    break;
    
  case 'migration:create':
    const name = process.argv[3];
    if (!name) {
      log('Usage: orm migration:create <name>', 'yellow');
      process.exit(1);
    }
    createMigration(name);
    break;
    
  case 'schema:update':
    updateSchema().catch(e => {
      log(e.message, 'red');
      process.exit(1);
    });
    break;
    
  case 'schema:drop':
    dropSchema().catch(e => {
      log(e.message, 'red');
      process.exit(1);
    });
    break;
    
  case 'schema:sql':
    generateSql().catch(e => {
      log(e.message, 'red');
      process.exit(1);
    });
    break;
    
  default:
    console.log(`
${COLORS.green}Structa ORM CLI${COLORS.reset}

${COLORS.cyan}Commands:${COLORS.reset}
  migrate              Run pending migrations
  migrate:revert       Revert last migration
  migration:create     Create a new migration file
  schema:update        Sync entities to database
  schema:drop          Drop all tables
  schema:sql           Generate SQL schema

${COLORS.cyan}Usage:${COLORS.reset}
  npx tsx node_modules/@structa/orm/dist/cli.js migrate
  structa orm migrate
`);
    break;
}
