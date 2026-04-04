export class MigrationExecutor {
    dataSource;
    tableName = 'structa_migrations';
    constructor(dataSource) {
        this.dataSource = dataSource;
    }
    async executeMigrations(migrations) {
        await this.ensureMigrationsTable();
        const executedMigrations = await this.getExecutedMigrations();
        const pendingMigrations = migrations.filter(m => !executedMigrations.includes(m.name));
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
    async ensureMigrationsTable() {
        const sql = `
      CREATE TABLE IF NOT EXISTS ${this.tableName} (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        executed_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `;
        await this.dataSource.execute(sql);
    }
    async getExecutedMigrations() {
        const sql = `SELECT name FROM ${this.tableName}`;
        const results = await this.dataSource.execute(sql);
        return results.map((row) => row.name);
    }
    async recordMigration(name) {
        const sql = `INSERT INTO ${this.tableName} (name) VALUES (?)`;
        await this.dataSource.execute(sql, [name]);
    }
    async revertMigration(migration) {
        console.log(`[Structa ORM] Reverting migration: ${migration.name}`);
        await this.dataSource.transaction(async () => {
            await migration.down();
            await this.removeMigrationRecord(migration.name);
        });
        console.log(`[Structa ORM] Migration reverted: ${migration.name}`);
    }
    async removeMigrationRecord(name) {
        const sql = `DELETE FROM ${this.tableName} WHERE name = ?`;
        await this.dataSource.execute(sql, [name]);
    }
}
export class BaseMigration {
}
export class CreateTableMigration extends BaseMigration {
    name;
    tableName;
    columns;
    foreignKeys;
    constructor(tableName, columns, foreignKeys) {
        super();
        this.tableName = tableName;
        this.columns = columns;
        this.foreignKeys = foreignKeys;
        this.name = `Create_${tableName}_Table`;
    }
    async up() {
        const dataSource = this.dataSource;
        const columnDefs = this.columns.map(col => {
            let def = `"${col.name}" ${this.getColumnType(col.type)}`;
            if (col.primary)
                def += ' PRIMARY KEY';
            if (col.autoIncrement)
                def += ' AUTOINCREMENT';
            if (!col.nullable)
                def += ' NOT NULL';
            return def;
        });
        if (this.foreignKeys) {
            for (const fk of this.foreignKeys) {
                columnDefs.push(`"${fk.column}" INTEGER, FOREIGN KEY ("${fk.column}") REFERENCES "${fk.referencedTable}"("${fk.referencedColumn}")${fk.onDelete ? ` ON DELETE ${fk.onDelete}` : ''}`);
            }
        }
        const sql = `CREATE TABLE "${this.tableName}" (${columnDefs.join(', ')})`;
        await dataSource.execute(sql);
    }
    async down() {
        const dataSource = this.dataSource;
        const sql = `DROP TABLE IF EXISTS "${this.tableName}"`;
        await dataSource.execute(sql);
    }
    getColumnType(type) {
        const typeMap = {
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
//# sourceMappingURL=migration.js.map