import { DataSource } from './data-source';

export interface Migration {
  name: string;
  up(): Promise<void>;
  down(): Promise<void>;
}

export class MigrationExecutor {
  private dataSource: DataSource;
  private tableName: string = 'structa_migrations';

  constructor(dataSource: DataSource) {
    this.dataSource = dataSource;
  }

  async executeMigrations(migrations: Migration[]): Promise<void> {
    await this.ensureMigrationsTable();
    
    const executedMigrations = await this.getExecutedMigrations();
    const pendingMigrations = migrations.filter(
      m => !executedMigrations.includes(m.name)
    );

    for (const migration of pendingMigrations) {
      console.log(`[Structa ORM] Running migration: ${migration.name}`);
      
      await this.dataSource.transaction(async () => {
        await migration.up();
        await this.recordMigration(migration.name);
      });
      
      console.log(`[Structa ORM] Migration completed: ${migration.name}`);
    }

    if (pendingMigrations.length === 0) {
      console.log(`[Structa ORM] No pending migrations`);
    }
  }

  private async ensureMigrationsTable(): Promise<void> {
    const sql = `
      CREATE TABLE IF NOT EXISTS ${this.tableName} (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        executed_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `;
    await this.dataSource.execute(sql);
  }

  private async getExecutedMigrations(): Promise<string[]> {
    const sql = `SELECT name FROM ${this.tableName}`;
    const results = await this.dataSource.execute(sql);
    return results.map((row: any) => row.name);
  }

  private async recordMigration(name: string): Promise<void> {
    const sql = `INSERT INTO ${this.tableName} (name) VALUES (?)`;
    await this.dataSource.execute(sql, [name]);
  }

  async revertMigration(migration: Migration): Promise<void> {
    console.log(`[Structa ORM] Reverting migration: ${migration.name}`);
    
    await this.dataSource.transaction(async () => {
      await migration.down();
      await this.removeMigrationRecord(migration.name);
    });
    
    console.log(`[Structa ORM] Migration reverted: ${migration.name}`);
  }

  private async removeMigrationRecord(name: string): Promise<void> {
    const sql = `DELETE FROM ${this.tableName} WHERE name = ?`;
    await this.dataSource.execute(sql, [name]);
  }
}

export abstract class BaseMigration implements Migration {
  abstract name: string;

  abstract up(): Promise<void>;

  abstract down(): Promise<void>;
}

export class CreateTableMigration extends BaseMigration {
  name: string;
  private tableName: string;
  private columns: { name: string; type: string; nullable?: boolean; primary?: boolean; autoIncrement?: boolean }[];
  private foreignKeys?: { column: string; referencedTable: string; referencedColumn: string; onDelete?: string }[];

  constructor(
    tableName: string,
    columns: { name: string; type: string; nullable?: boolean; primary?: boolean; autoIncrement?: boolean }[],
    foreignKeys?: { column: string; referencedTable: string; referencedColumn: string; onDelete?: string }[]
  ) {
    super();
    this.tableName = tableName;
    this.columns = columns;
    this.foreignKeys = foreignKeys;
    this.name = `Create_${tableName}_Table`;
  }

  async up(): Promise<void> {
    const dataSource = (this as any).dataSource as DataSource;
    const columnDefs = this.columns.map(col => {
      let def = `"${col.name}" ${this.getColumnType(col.type)}`;
      if (col.primary) def += ' PRIMARY KEY';
      if (col.autoIncrement) def += ' AUTOINCREMENT';
      if (!col.nullable) def += ' NOT NULL';
      return def;
    });

    if (this.foreignKeys) {
      for (const fk of this.foreignKeys) {
        columnDefs.push(
          `"${fk.column}" INTEGER, FOREIGN KEY ("${fk.column}") REFERENCES "${fk.referencedTable}"("${fk.referencedColumn}")${fk.onDelete ? ` ON DELETE ${fk.onDelete}` : ''}`
        );
      }
    }

    const sql = `CREATE TABLE "${this.tableName}" (${columnDefs.join(', ')})`;
    await dataSource.execute(sql);
  }

  async down(): Promise<void> {
    const dataSource = (this as any).dataSource as DataSource;
    const sql = `DROP TABLE IF EXISTS "${this.tableName}"`;
    await dataSource.execute(sql);
  }

  private getColumnType(type: string): string {
    const typeMap: Record<string, string> = {
      int: 'INTEGER',
      string: 'TEXT',
      text: 'TEXT',
      varchar: 'VARCHAR(255)',
      boolean: 'BOOLEAN',
      datetime: 'DATETIME',
      date: 'DATE',
      float: 'REAL',
      double: 'DOUBLE',
      json: 'TEXT',
      uuid: 'TEXT',
    };
    return typeMap[type] || 'TEXT';
  }
}
