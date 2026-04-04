import { DataSource } from './data-source';
export interface Migration {
    name: string;
    up(): Promise<void>;
    down(): Promise<void>;
}
export declare class MigrationExecutor {
    private dataSource;
    private tableName;
    constructor(dataSource: DataSource);
    executeMigrations(migrations: Migration[]): Promise<void>;
    private ensureMigrationsTable;
    private getExecutedMigrations;
    private recordMigration;
    revertMigration(migration: Migration): Promise<void>;
    private removeMigrationRecord;
}
export declare abstract class BaseMigration implements Migration {
    abstract name: string;
    abstract up(): Promise<void>;
    abstract down(): Promise<void>;
}
export declare class CreateTableMigration extends BaseMigration {
    name: string;
    private tableName;
    private columns;
    private foreignKeys?;
    constructor(tableName: string, columns: {
        name: string;
        type: string;
        nullable?: boolean;
        primary?: boolean;
        autoIncrement?: boolean;
    }[], foreignKeys?: {
        column: string;
        referencedTable: string;
        referencedColumn: string;
        onDelete?: string;
    }[]);
    up(): Promise<void>;
    down(): Promise<void>;
    private getColumnType;
}
//# sourceMappingURL=migration.d.ts.map