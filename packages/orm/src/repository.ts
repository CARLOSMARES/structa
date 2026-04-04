import { DataSource } from './data-source';
import { BaseEntity, getEntityMetadata } from './entity';
import { QueryBuilder } from './query-builder';

export class Repository<T extends BaseEntity> {
  private dataSource: DataSource;
  private entityClass: new () => T;
  metadata: any;

  constructor(dataSource: DataSource, entityClass: new () => T) {
    this.dataSource = dataSource;
    this.entityClass = entityClass;
    this.metadata = getEntityMetadata(entityClass);
  }

  create(plainObject: Partial<T>): T {
    const entity = new this.entityClass();
    Object.assign(entity, plainObject);
    return entity;
  }

  createMany(plainObjects: Partial<T>[]): T[] {
    return plainObjects.map(obj => this.create(obj));
  }

  async save(entity: T): Promise<T> {
    const id = entity.getId();
    const isUpdate = Object.values(id).some(v => v !== undefined);
    
    if (isUpdate) {
      return this.update(id, entity);
    } else {
      return this.insert(entity);
    }
  }

  async insert(entity: T): Promise<T> {
    const tableName = this.metadata.tableName;
    const columns = this.metadata.columns.filter((c: any) => !c.autoIncrement);
    const columnNames = columns.map((c: any) => c.columnName);
    const placeholders = columns.map(() => '?');
    const values = columns.map((c: any) => {
      let value = (entity as any)[c.propertyName];
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
        (entity as any)[primaryCol.propertyName] = result.insertId || result.id || 1;
      }
    }

    return entity;
  }

  async update(id: Record<string, any>, entity: T): Promise<T> {
    const tableName = this.metadata.tableName;
    const updates: string[] = [];
    const values: any[] = [];

    for (const col of this.metadata.columns) {
      if (!col.primary) {
        updates.push(`${col.columnName} = ?`);
        let value = (entity as any)[col.propertyName];
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

  async delete(id: Record<string, any>): Promise<boolean> {
    const tableName = this.metadata.tableName;
    const whereClause = Object.entries(id)
      .map(([key, _]) => `${key} = ?`)
      .join(' AND ');

    const sql = `DELETE FROM ${tableName} WHERE ${whereClause}`;
    const result = await this.dataSource.execute(sql, Object.values(id));

    return (result.affectedRows || result.changes || 0) > 0;
  }

  async findOne(id: any): Promise<T | null> {
    return this.createQueryBuilder()
      .where(`${this.getAlias()}.id = ?`, id)
      .getOne();
  }

  async findById(id: any): Promise<T | null> {
    return this.findOne(id);
  }

  async find(options?: FindOptions<T>): Promise<T[]> {
    const qb = this.createQueryBuilder();
    
    if (options?.where) {
      for (const [key, value] of Object.entries(options.where)) {
        if (value === null) {
          qb.where(`${this.getAlias()}.${key} IS NULL`);
        } else if (Array.isArray(value)) {
          qb.whereIn(key, value);
        } else {
          qb.andWhere(`${this.getAlias()}.${key} = ?`, value);
        }
      }
    }
    
    if (options?.orderBy) {
      for (const [key, order] of Object.entries(options.orderBy)) {
        qb.orderBy(key, order as 'ASC' | 'DESC');
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

  async findAll(): Promise<T[]> {
    return this.find();
  }

  async findAndCount(options?: FindOptions<T>): Promise<[T[], number]> {
    const qb = this.createQueryBuilder();
    
    if (options?.where) {
      for (const [key, value] of Object.entries(options.where)) {
        if (value === null) {
          qb.where(`${this.getAlias()}.${key} IS NULL`);
        } else if (Array.isArray(value)) {
          qb.whereIn(key, value);
        } else {
          qb.andWhere(`${this.getAlias()}.${key} = ?`, value);
        }
      }
    }
    
    if (options?.orderBy) {
      for (const [key, order] of Object.entries(options.orderBy)) {
        qb.orderBy(key, order as 'ASC' | 'DESC');
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

  async count(where?: Record<string, any>): Promise<number> {
    const qb = this.createQueryBuilder();
    
    if (where) {
      for (const [key, value] of Object.entries(where)) {
        qb.andWhere(`${this.getAlias()}.${key} = ?`, value);
      }
    }
    
    return qb.getCount();
  }

  async exists(where: Record<string, any>): Promise<boolean> {
    const qb = this.createQueryBuilder();
    
    for (const [key, value] of Object.entries(where)) {
      qb.andWhere(`${this.getAlias()}.${key} = ?`, value);
    }
    
    qb.limit(1);
    const result = await qb.getOne();
    return result !== null;
  }

  createQueryBuilder(): QueryBuilder<T> {
    return new QueryBuilder<T>(this.dataSource, this.entityClass);
  }

  private getAlias(): string {
    return 't';
  }

  clear(): Promise<void> {
    return Promise.resolve();
  }

  async remove(entity: T): Promise<T> {
    const id = entity.getId();
    await this.delete(id);
    return entity;
  }

  async softDelete(id: Record<string, any>): Promise<boolean> {
    const tableName = this.metadata.tableName;
    const whereClause = Object.entries(id)
      .map(([key, _]) => `${key} = ?`)
      .join(' AND ');

    const sql = `UPDATE ${tableName} SET deletedAt = NOW() WHERE ${whereClause}`;
    const result = await this.dataSource.execute(sql, Object.values(id));

    return (result.affectedRows || result.changes || 0) > 0;
  }
}

export interface FindOptions<T> {
  where?: Partial<Record<keyof T, any>>;
  orderBy?: Partial<Record<keyof T, 'ASC' | 'DESC'>>;
  skip?: number;
  take?: number;
  limit?: number;
  offset?: number;
}
