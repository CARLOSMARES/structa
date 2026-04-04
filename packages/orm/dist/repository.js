import { getEntityMetadata } from './entity';
import { QueryBuilder } from './query-builder';
export class Repository {
    dataSource;
    entityClass;
    metadata;
    constructor(dataSource, entityClass) {
        this.dataSource = dataSource;
        this.entityClass = entityClass;
        this.metadata = getEntityMetadata(entityClass);
    }
    create(plainObject) {
        const entity = new this.entityClass();
        Object.assign(entity, plainObject);
        return entity;
    }
    createMany(plainObjects) {
        return plainObjects.map(obj => this.create(obj));
    }
    async save(entity) {
        const id = entity.getId();
        const isUpdate = Object.values(id).some(v => v !== undefined);
        if (isUpdate) {
            return this.update(id, entity);
        }
        else {
            return this.insert(entity);
        }
    }
    async insert(entity) {
        const tableName = this.metadata.tableName;
        const columns = this.metadata.columns.filter((c) => !c.autoIncrement);
        const columnNames = columns.map((c) => c.columnName);
        const placeholders = columns.map(() => '?');
        const values = columns.map((c) => {
            let value = entity[c.propertyName];
            if (c.transformer?.to) {
                value = c.transformer.to(value);
            }
            return value;
        });
        const sql = `INSERT INTO ${tableName} (${columnNames.join(', ')}) VALUES (${placeholders.join(', ')})`;
        const result = await this.dataSource.execute(sql, values);
        if (this.metadata.primaryColumns.length === 1) {
            const primaryCol = this.metadata.primaryColumns[0];
            if (primaryCol.autoIncrement) {
                entity[primaryCol.propertyName] = result.insertId || result.id || 1;
            }
        }
        return entity;
    }
    async update(id, entity) {
        const tableName = this.metadata.tableName;
        const updates = [];
        const values = [];
        for (const col of this.metadata.columns) {
            if (!col.primary) {
                updates.push(`${col.columnName} = ?`);
                let value = entity[col.propertyName];
                if (col.transformer?.to) {
                    value = col.transformer.to(value);
                }
                values.push(value);
            }
        }
        const whereClause = Object.entries(id)
            .map(([key, _]) => `${key} = ?`)
            .join(' AND ');
        values.push(...Object.values(id));
        const sql = `UPDATE ${tableName} SET ${updates.join(', ')} WHERE ${whereClause}`;
        await this.dataSource.execute(sql, values);
        return entity;
    }
    async delete(id) {
        const tableName = this.metadata.tableName;
        const whereClause = Object.entries(id)
            .map(([key, _]) => `${key} = ?`)
            .join(' AND ');
        const sql = `DELETE FROM ${tableName} WHERE ${whereClause}`;
        const result = await this.dataSource.execute(sql, Object.values(id));
        return (result.affectedRows || result.changes || 0) > 0;
    }
    async findOne(id) {
        return this.createQueryBuilder()
            .where(`${this.getAlias()}.id = ?`, id)
            .getOne();
    }
    async findById(id) {
        return this.findOne(id);
    }
    async find(options) {
        const qb = this.createQueryBuilder();
        if (options?.where) {
            for (const [key, value] of Object.entries(options.where)) {
                if (value === null) {
                    qb.where(`${this.getAlias()}.${key} IS NULL`);
                }
                else if (Array.isArray(value)) {
                    qb.whereIn(key, value);
                }
                else {
                    qb.andWhere(`${this.getAlias()}.${key} = ?`, value);
                }
            }
        }
        if (options?.orderBy) {
            for (const [key, order] of Object.entries(options.orderBy)) {
                qb.orderBy(key, order);
            }
        }
        if (options?.skip) {
            qb.skip(options.skip);
        }
        if (options?.take) {
            qb.take(options.take);
        }
        if (options?.limit) {
            qb.limit(options.limit);
        }
        return qb.getMany();
    }
    async findAll() {
        return this.find();
    }
    async findAndCount(options) {
        const qb = this.createQueryBuilder();
        if (options?.where) {
            for (const [key, value] of Object.entries(options.where)) {
                if (value === null) {
                    qb.where(`${this.getAlias()}.${key} IS NULL`);
                }
                else if (Array.isArray(value)) {
                    qb.whereIn(key, value);
                }
                else {
                    qb.andWhere(`${this.getAlias()}.${key} = ?`, value);
                }
            }
        }
        if (options?.orderBy) {
            for (const [key, order] of Object.entries(options.orderBy)) {
                qb.orderBy(key, order);
            }
        }
        if (options?.skip) {
            qb.skip(options.skip);
        }
        if (options?.take) {
            qb.take(options.take);
        }
        const [entities, count] = await Promise.all([
            qb.getMany(),
            qb.getCount(),
        ]);
        return [entities, count];
    }
    async count(where) {
        const qb = this.createQueryBuilder();
        if (where) {
            for (const [key, value] of Object.entries(where)) {
                qb.andWhere(`${this.getAlias()}.${key} = ?`, value);
            }
        }
        return qb.getCount();
    }
    async exists(where) {
        const qb = this.createQueryBuilder();
        for (const [key, value] of Object.entries(where)) {
            qb.andWhere(`${this.getAlias()}.${key} = ?`, value);
        }
        qb.limit(1);
        const result = await qb.getOne();
        return result !== null;
    }
    createQueryBuilder() {
        return new QueryBuilder(this.dataSource, this.entityClass);
    }
    getAlias() {
        return 't';
    }
    clear() {
        return Promise.resolve();
    }
    async remove(entity) {
        const id = entity.getId();
        await this.delete(id);
        return entity;
    }
    async softDelete(id) {
        const tableName = this.metadata.tableName;
        const whereClause = Object.entries(id)
            .map(([key, _]) => `${key} = ?`)
            .join(' AND ');
        const sql = `UPDATE ${tableName} SET deletedAt = NOW() WHERE ${whereClause}`;
        const result = await this.dataSource.execute(sql, Object.values(id));
        return (result.affectedRows || result.changes || 0) > 0;
    }
}
//# sourceMappingURL=repository.js.map