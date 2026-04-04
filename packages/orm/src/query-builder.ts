import { DataSource } from './data-source';
import { BaseEntity, getEntityMetadata } from './entity';

export type SelectCondition = 'AND' | 'OR';

export interface JoinOptions {
  type: 'inner' | 'left' | 'right' | 'full';
  alias?: string;
  condition?: string;
}

export class QueryBuilder<T extends BaseEntity> {
  private dataSource: DataSource;
  private entityClass: new () => T;
  private alias: string = 't';
  
  private selectColumns: string[] = [];
  private joinClauses: string[] = [];
  private whereClauses: string[] = [];
  private whereParams: any[] = [];
  private orderByClauses: string[] = [];
  private groupByClauses: string[] = [];
  private havingClauses: string[] = [];
  private limitValue?: number;
  private offsetValue?: number;
  private condition: SelectCondition = 'AND';

  constructor(dataSource: DataSource, entityClass: new () => T) {
    this.dataSource = dataSource;
    this.entityClass = entityClass;
    this.selectColumns = ['*'];
  }

  select(...columns: string[]): this {
    if (columns.length === 0) {
      this.selectColumns = ['*'];
    } else {
      this.selectColumns = columns.map(col => `${this.alias}.${col}`);
    }
    return this;
  }

  distinct(): this {
    this.selectColumns = ['DISTINCT ' + this.selectColumns.join(', ')];
    return this;
  }

  innerJoin(
    relation: string | ((e: T) => any),
    alias?: string,
    condition?: string
  ): this {
    const joinAlias = alias || 'r';
    this.joinClauses.push(`INNER JOIN ${joinAlias} ON ${condition || '1=1'}`);
    return this;
  }

  leftJoin(
    relation: string | ((e: T) => any),
    alias?: string,
    condition?: string
  ): this {
    const joinAlias = alias || 'r';
    this.joinClauses.push(`LEFT JOIN ${joinAlias} ON ${condition || '1=1'}`);
    return this;
  }

  rightJoin(
    relation: string | ((e: T) => any),
    alias?: string,
    condition?: string
  ): this {
    const joinAlias = alias || 'r';
    this.joinClauses.push(`RIGHT JOIN ${joinAlias} ON ${condition || '1=1'}`);
    return this;
  }

  where(condition: string, ...params: any[]): this {
    this.whereClauses.push(`(${condition})`);
    this.whereParams.push(...params);
    return this;
  }

  andWhere(condition: string, ...params: any[]): this {
    this.condition = 'AND';
    this.whereClauses.push(`(${condition})`);
    this.whereParams.push(...params);
    return this;
  }

  orWhere(condition: string, ...params: any[]): this {
    this.condition = 'OR';
    this.whereClauses.push(`(${condition})`);
    this.whereParams.push(...params);
    return this;
  }

  whereIn(column: string, values: any[]): this {
    const placeholders = values.map(() => '?').join(', ');
    this.whereClauses.push(`${this.alias}.${column} IN (${placeholders})`);
    this.whereParams.push(...values);
    return this;
  }

  whereNull(column: string): this {
    this.whereClauses.push(`${this.alias}.${column} IS NULL`);
    return this;
  }

  whereNotNull(column: string): this {
    this.whereClauses.push(`${this.alias}.${column} IS NOT NULL`);
    return this;
  }

  whereBetween(column: string, start: any, end: any): this {
    this.whereClauses.push(`${this.alias}.${column} BETWEEN ? AND ?`);
    this.whereParams.push(start, end);
    return this;
  }

  whereLike(column: string, value: string): this {
    this.whereClauses.push(`${this.alias}.${column} LIKE ?`);
    this.whereParams.push(`%${value}%`);
    return this;
  }

  orderBy(column: string, order: 'ASC' | 'DESC' = 'ASC'): this {
    this.orderByClauses.push(`${this.alias}.${column} ${order}`);
    return this;
  }

  orderById(order: 'ASC' | 'DESC' = 'ASC'): this {
    this.orderByClauses.push(`${this.alias}.id ${order}`);
    return this;
  }

  groupBy(...columns: string[]): this {
    this.groupByClauses.push(...columns.map(col => `${this.alias}.${col}`));
    return this;
  }

  having(condition: string, ...params: any[]): this {
    this.havingClauses.push(`(${condition})`);
    this.whereParams.push(...params);
    return this;
  }

  limit(limit: number): this {
    this.limitValue = limit;
    return this;
  }

  offset(offset: number): this {
    this.offsetValue = offset;
    return this;
  }

  take(take: number): this {
    return this.limit(take);
  }

  skip(skip: number): this {
    return this.offset(skip);
  }

  async getOne(): Promise<T | null> {
    this.limit(1);
    const results = await this.execute();
    return results[0] || null;
  }

  async getMany(): Promise<T[]> {
    return this.execute();
  }

  async getCount(): Promise<number> {
    const originalSelect = this.selectColumns;
    this.selectColumns = ['COUNT(*) as count'];
    
    const sql = this.buildSQL();
    const results = await this.dataSource.execute(sql, this.whereParams);
    
    this.selectColumns = originalSelect;
    
    return results[0]?.count || 0;
  }

  async execute(): Promise<T[]> {
    const sql = this.buildSQL();
    const results = await this.dataSource.execute(sql, this.whereParams);
    
    return results.map((row: any) => {
      const entity = new this.entityClass();
      for (const [key, value] of Object.entries(row)) {
        (entity as any)[key] = value;
      }
      return entity;
    });
  }

  private buildSQL(): string {
    const metadata = getEntityMetadata(this.entityClass);
    const tableName = metadata.tableName;
    
    let sql = `SELECT ${this.selectColumns.join(', ')} FROM ${tableName} ${this.alias}`;
    
    if (this.joinClauses.length > 0) {
      sql += ' ' + this.joinClauses.join(' ');
    }
    
    if (this.whereClauses.length > 0) {
      sql += ' WHERE ' + this.whereClauses.join(` ${this.condition} `);
    }
    
    if (this.groupByClauses.length > 0) {
      sql += ' GROUP BY ' + this.groupByClauses.join(', ');
    }
    
    if (this.havingClauses.length > 0) {
      sql += ' HAVING ' + this.havingClauses.join(' AND ');
    }
    
    if (this.orderByClauses.length > 0) {
      sql += ' ORDER BY ' + this.orderByClauses.join(', ');
    }
    
    if (this.limitValue !== undefined) {
      sql += ` LIMIT ${this.limitValue}`;
    }
    
    if (this.offsetValue !== undefined) {
      sql += ` OFFSET ${this.offsetValue}`;
    }
    
    return sql;
  }

  clone(): QueryBuilder<T> {
    const cloned = new QueryBuilder<T>(this.dataSource, this.entityClass);
    cloned.alias = this.alias;
    cloned.selectColumns = [...this.selectColumns];
    cloned.joinClauses = [...this.joinClauses];
    cloned.whereClauses = [...this.whereClauses];
    cloned.whereParams = [...this.whereParams];
    cloned.orderByClauses = [...this.orderByClauses];
    cloned.groupByClauses = [...this.groupByClauses];
    cloned.havingClauses = [...this.havingClauses];
    cloned.limitValue = this.limitValue;
    cloned.offsetValue = this.offsetValue;
    cloned.condition = this.condition;
    return cloned;
  }
}
