import { BaseEntity, getEntityMetadata } from './entity';
import { QueryBuilder } from './query-builder';
import { Repository } from './repository';
import { Migration, MigrationExecutor } from './migration';

export interface DataSourceOptions {
  type: 'sqlite' | 'postgres' | 'mysql' | 'mssql';
  database: string;
  host?: string;
  port?: number;
  username?: string;
  password?: string;
  synchronize?: boolean;
  logging?: boolean;
  migrations?: Migration[];
}

export class DataSource {
  private options: DataSourceOptions;
  private connected: boolean = false;
  private repositories: Map<string, Repository<any>> = new Map();
  
  constructor(options: DataSourceOptions) {
    this.options = {
      synchronize: false,
      logging: false,
      ...options,
    };
  }

  async initialize(): Promise<void> {
    if (this.connected) {
      return;
    }

    if (this.options.logging) {
      console.log(`[Structa ORM] Connecting to ${this.options.type} database...`);
    }

    try {
      await this.connect();
      this.connected = true;
      
      if (this.options.synchronize) {
        await this.synchronize();
      }

      if (this.options.migrations?.length) {
        await this.runMigrations();
      }

      if (this.options.logging) {
        console.log(`[Structa ORM] Database connected successfully`);
      }
    } catch (error) {
      console.error(`[Structa ORM] Failed to connect to database:`, error);
      throw error;
    }
  }

  private async connect(): Promise<void> {
    switch (this.options.type) {
      case 'sqlite':
        await this.connectSqlite();
        break;
      case 'postgres':
        await this.connectPostgres();
        break;
      case 'mysql':
        await this.connectMysql();
        break;
      default:
        throw new Error(`Unsupported database type: ${this.options.type}`);
    }
  }

  private async connectSqlite(): Promise<void> {
    // SQLite connection using better-sqlite3 or sql.js
    // For now, we'll use a mock implementation
    console.log(`[Structa ORM] SQLite: ${this.options.database}`);
  }

  private async connectPostgres(): Promise<void> {
    // PostgreSQL connection using pg
    console.log(`[Structa ORM] PostgreSQL: ${this.options.host}:${this.options.port}`);
  }

  private async connectMysql(): Promise<void> {
    // MySQL connection using mysql2
    console.log(`[Structa ORM] MySQL: ${this.options.host}:${this.options.port}`);
  }

  async disconnect(): Promise<void> {
    if (!this.connected) {
      return;
    }

    for (const [, repo] of this.repositories) {
      await repo.clear();
    }

    this.repositories.clear();
    this.connected = false;
    
    if (this.options.logging) {
      console.log(`[Structa ORM] Database disconnected`);
    }
  }

  isConnected(): boolean {
    return this.connected;
  }

  getRepository<T extends BaseEntity>(entity: new () => T): Repository<T> {
    const entityName = entity.name;
    
    if (!this.repositories.has(entityName)) {
      this.repositories.set(entityName, new Repository<T>(this, entity));
    }
    
    return this.repositories.get(entityName) as Repository<T>;
  }

  createQueryBuilder<T extends BaseEntity>(entity: new () => T): QueryBuilder<T> {
    return new QueryBuilder<T>(this, entity);
  }

  private async synchronize(): Promise<void> {
    if (this.options.logging) {
      console.log(`[Structa ORM] Synchronizing database schema...`);
    }
    
    for (const [, repo] of this.repositories) {
      const entity = repo.metadata;
      await this.createTableIfNotExists(entity);
    }
  }

  private async createTableIfNotExists(entity: any): Promise<void> {
    const tableName = entity.tableName;
    const columns = entity.columns.map((col: any) => {
      let sql = `"${col.columnName}" ${this.getColumnTypeSQL(col)}`;
      
      if (col.primary) {
        sql += ' PRIMARY KEY';
        if (col.autoIncrement) {
          sql += ' AUTOINCREMENT';
        }
      }
      
      if (!col.nullable) {
        sql += ' NOT NULL';
      }
      
      if (col.unique) {
        sql += ' UNIQUE';
      }
      
      if (col.default !== undefined) {
        sql += ` DEFAULT ${col.default}`;
      }
      
      return sql;
    }).join(', ');

    const sql = `CREATE TABLE IF NOT EXISTS "${tableName}" (${columns})`;
    
    if (this.options.logging) {
      console.log(`[Structa ORM] Executing: ${sql}`);
    }
    
    await this.execute(sql);
  }

  private getColumnTypeSQL(column: any): string {
    switch (column.type) {
      case 'int':
      case 'integer':
        return 'INTEGER';
      case 'bigint':
        return 'BIGINT';
      case 'smallint':
        return 'SMALLINT';
      case 'tinyint':
        return 'TINYINT';
      case 'float':
        return 'REAL';
      case 'double':
        return 'DOUBLE';
      case 'decimal':
        return `DECIMAL(${column.precision || 10},${column.scale || 2})`;
      case 'boolean':
        return 'BOOLEAN';
      case 'string':
      case 'text':
        return 'TEXT';
      case 'varchar':
        return `VARCHAR(${column.length || 255})`;
      case 'char':
        return `CHAR(${column.length || 1})`;
      case 'date':
        return 'DATE';
      case 'time':
        return 'TIME';
      case 'datetime':
      case 'timestamp':
        return 'DATETIME';
      case 'json':
        return 'TEXT';
      case 'uuid':
        return 'TEXT';
      case 'enum':
        return 'TEXT';
      default:
        return 'TEXT';
    }
  }

  private async runMigrations(): Promise<void> {
    if (!this.options.migrations?.length) return;

    const executor = new MigrationExecutor(this);
    await executor.executeMigrations(this.options.migrations);
  }

  async execute(sql: string, params?: any[]): Promise<any> {
    if (this.options.logging) {
      console.log(`[Structa ORM] SQL: ${sql}`, params);
    }
    
    switch (this.options.type) {
      case 'sqlite':
        return this.executeSqlite(sql, params);
      case 'postgres':
        return this.executePostgres(sql, params);
      case 'mysql':
        return this.executeMysql(sql, params);
      default:
        throw new Error(`Unsupported database type: ${this.options.type}`);
    }
  }

  private async executeSqlite(sql: string, params?: any[]): Promise<any> {
    // Mock implementation
    return { rows: [], changes: 0 };
  }

  private async executePostgres(sql: string, params?: any[]): Promise<any> {
    // Mock implementation
    return { rows: [], rowCount: 0 };
  }

  private async executeMysql(sql: string, params?: any[]): Promise<any> {
    // Mock implementation
    return { rows: [], affectedRows: 0 };
  }

  async transaction<T>(callback: () => Promise<T>): Promise<T> {
    await this.execute('BEGIN TRANSACTION');
    try {
      const result = await callback();
      await this.execute('COMMIT');
      return result;
    } catch (error) {
      await this.execute('ROLLBACK');
      throw error;
    }
  }
}
