import { getEntityMetadata } from './entity';
export class QueryBuilder {
    dataSource;
    entityClass;
    alias = 't';
    selectColumns = [];
    joinClauses = [];
    whereClauses = [];
    whereParams = [];
    orderByClauses = [];
    groupByClauses = [];
    havingClauses = [];
    limitValue;
    offsetValue;
    condition = 'AND';
    constructor(dataSource, entityClass) {
        this.dataSource = dataSource;
        this.entityClass = entityClass;
        this.selectColumns = ['*'];
    }
    select(...columns) {
        if (columns.length === 0) {
            this.selectColumns = ['*'];
        }
        else {
            this.selectColumns = columns.map(col => `${this.alias}.${col}`);
        }
        return this;
    }
    distinct() {
        this.selectColumns = ['DISTINCT ' + this.selectColumns.join(', ')];
        return this;
    }
    innerJoin(relation, alias, condition) {
        const joinAlias = alias || 'r';
        this.joinClauses.push(`INNER JOIN ${joinAlias} ON ${condition || '1=1'}`);
        return this;
    }
    leftJoin(relation, alias, condition) {
        const joinAlias = alias || 'r';
        this.joinClauses.push(`LEFT JOIN ${joinAlias} ON ${condition || '1=1'}`);
        return this;
    }
    rightJoin(relation, alias, condition) {
        const joinAlias = alias || 'r';
        this.joinClauses.push(`RIGHT JOIN ${joinAlias} ON ${condition || '1=1'}`);
        return this;
    }
    where(condition, ...params) {
        this.whereClauses.push(`(${condition})`);
        this.whereParams.push(...params);
        return this;
    }
    andWhere(condition, ...params) {
        this.condition = 'AND';
        this.whereClauses.push(`(${condition})`);
        this.whereParams.push(...params);
        return this;
    }
    orWhere(condition, ...params) {
        this.condition = 'OR';
        this.whereClauses.push(`(${condition})`);
        this.whereParams.push(...params);
        return this;
    }
    whereIn(column, values) {
        const placeholders = values.map(() => '?').join(', ');
        this.whereClauses.push(`${this.alias}.${column} IN (${placeholders})`);
        this.whereParams.push(...values);
        return this;
    }
    whereNull(column) {
        this.whereClauses.push(`${this.alias}.${column} IS NULL`);
        return this;
    }
    whereNotNull(column) {
        this.whereClauses.push(`${this.alias}.${column} IS NOT NULL`);
        return this;
    }
    whereBetween(column, start, end) {
        this.whereClauses.push(`${this.alias}.${column} BETWEEN ? AND ?`);
        this.whereParams.push(start, end);
        return this;
    }
    whereLike(column, value) {
        this.whereClauses.push(`${this.alias}.${column} LIKE ?`);
        this.whereParams.push(`%${value}%`);
        return this;
    }
    orderBy(column, order = 'ASC') {
        this.orderByClauses.push(`${this.alias}.${column} ${order}`);
        return this;
    }
    orderById(order = 'ASC') {
        this.orderByClauses.push(`${this.alias}.id ${order}`);
        return this;
    }
    groupBy(...columns) {
        this.groupByClauses.push(...columns.map(col => `${this.alias}.${col}`));
        return this;
    }
    having(condition, ...params) {
        this.havingClauses.push(`(${condition})`);
        this.whereParams.push(...params);
        return this;
    }
    limit(limit) {
        this.limitValue = limit;
        return this;
    }
    offset(offset) {
        this.offsetValue = offset;
        return this;
    }
    take(take) {
        return this.limit(take);
    }
    skip(skip) {
        return this.offset(skip);
    }
    async getOne() {
        this.limit(1);
        const results = await this.execute();
        return results[0] || null;
    }
    async getMany() {
        return this.execute();
    }
    async getCount() {
        const originalSelect = this.selectColumns;
        this.selectColumns = ['COUNT(*) as count'];
        const sql = this.buildSQL();
        const results = await this.dataSource.execute(sql, this.whereParams);
        this.selectColumns = originalSelect;
        return results[0]?.count || 0;
    }
    async execute() {
        const sql = this.buildSQL();
        const results = await this.dataSource.execute(sql, this.whereParams);
        return results.map((row) => {
            const entity = new this.entityClass();
            for (const [key, value] of Object.entries(row)) {
                entity[key] = value;
            }
            return entity;
        });
    }
    buildSQL() {
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
    clone() {
        const cloned = new QueryBuilder(this.dataSource, this.entityClass);
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
//# sourceMappingURL=query-builder.js.map