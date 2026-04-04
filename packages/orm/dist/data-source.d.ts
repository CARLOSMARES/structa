import { BaseEntity } from './entity';
import { QueryBuilder } from './query-builder';
import { Repository } from './repository';
import { Migration } from './migration';
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
export declare class DataSource {
    private options;
    private connected;
    private repositories;
    constructor(options: DataSourceOptions);
    initialize(): Promise<void>;
    private connect;
    private connectSqlite;
    private connectPostgres;
    private connectMysql;
    disconnect(): Promise<void>;
    isConnected(): boolean;
    getRepository<T extends BaseEntity>(entity: new () => T): Repository<T>;
    createQueryBuilder<T extends BaseEntity>(entity: new () => T): QueryBuilder<T>;
    private synchronize;
    private createTableIfNotExists;
    private getColumnTypeSQL;
    private runMigrations;
    execute(sql: string, params?: any[]): Promise<any>;
    private executeSqlite;
    private executePostgres;
    private executeMysql;
    transaction<T>(callback: () => Promise<T>): Promise<T>;
}
//# sourceMappingURL=data-source.d.ts.map