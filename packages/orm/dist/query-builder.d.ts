import { DataSource } from './data-source';
import { BaseEntity } from './entity';
export type SelectCondition = 'AND' | 'OR';
export interface JoinOptions {
    type: 'inner' | 'left' | 'right' | 'full';
    alias?: string;
    condition?: string;
}
export declare class QueryBuilder<T extends BaseEntity> {
    private dataSource;
    private entityClass;
    private alias;
    private selectColumns;
    private joinClauses;
    private whereClauses;
    private whereParams;
    private orderByClauses;
    private groupByClauses;
    private havingClauses;
    private limitValue?;
    private offsetValue?;
    private condition;
    constructor(dataSource: DataSource, entityClass: new () => T);
    select(...columns: string[]): this;
    distinct(): this;
    innerJoin(relation: string | ((e: T) => any), alias?: string, condition?: string): this;
    leftJoin(relation: string | ((e: T) => any), alias?: string, condition?: string): this;
    rightJoin(relation: string | ((e: T) => any), alias?: string, condition?: string): this;
    where(condition: string, ...params: any[]): this;
    andWhere(condition: string, ...params: any[]): this;
    orWhere(condition: string, ...params: any[]): this;
    whereIn(column: string, values: any[]): this;
    whereNull(column: string): this;
    whereNotNull(column: string): this;
    whereBetween(column: string, start: any, end: any): this;
    whereLike(column: string, value: string): this;
    orderBy(column: string, order?: 'ASC' | 'DESC'): this;
    orderById(order?: 'ASC' | 'DESC'): this;
    groupBy(...columns: string[]): this;
    having(condition: string, ...params: any[]): this;
    limit(limit: number): this;
    offset(offset: number): this;
    take(take: number): this;
    skip(skip: number): this;
    getOne(): Promise<T | null>;
    getMany(): Promise<T[]>;
    getCount(): Promise<number>;
    execute(): Promise<T[]>;
    private buildSQL;
    clone(): QueryBuilder<T>;
}
//# sourceMappingURL=query-builder.d.ts.map