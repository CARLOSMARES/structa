import { DataSource } from './data-source';
import { BaseEntity } from './entity';
import { QueryBuilder } from './query-builder';
export declare class Repository<T extends BaseEntity> {
    private dataSource;
    private entityClass;
    metadata: any;
    constructor(dataSource: DataSource, entityClass: new () => T);
    create(plainObject: Partial<T>): T;
    createMany(plainObjects: Partial<T>[]): T[];
    save(entity: T): Promise<T>;
    insert(entity: T): Promise<T>;
    update(id: Record<string, any>, entity: T): Promise<T>;
    delete(id: Record<string, any>): Promise<boolean>;
    findOne(id: any): Promise<T | null>;
    findById(id: any): Promise<T | null>;
    find(options?: FindOptions<T>): Promise<T[]>;
    findAll(): Promise<T[]>;
    findAndCount(options?: FindOptions<T>): Promise<[T[], number]>;
    count(where?: Record<string, any>): Promise<number>;
    exists(where: Record<string, any>): Promise<boolean>;
    createQueryBuilder(): QueryBuilder<T>;
    private getAlias;
    clear(): Promise<void>;
    remove(entity: T): Promise<T>;
    softDelete(id: Record<string, any>): Promise<boolean>;
}
export interface FindOptions<T> {
    where?: Partial<Record<keyof T, any>>;
    orderBy?: Partial<Record<keyof T, 'ASC' | 'DESC'>>;
    skip?: number;
    take?: number;
    limit?: number;
    offset?: number;
}
//# sourceMappingURL=repository.d.ts.map