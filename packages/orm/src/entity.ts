import 'reflect-metadata';
import { EntityMetadata, ColumnMetadata } from './decorators';

export interface EntityId {
  [key: string]: any;
}

export abstract class BaseEntity {
  [key: string]: any;

  static getMetadata(): EntityMetadata {
    throw new Error('Not implemented');
  }

  static getTableName(): string {
    const meta = this.getMetadata();
    return meta.tableName;
  }

  static getColumns(): ColumnMetadata[] {
    return this.getMetadata().columns;
  }

  static getPrimaryColumns(): ColumnMetadata[] {
    return this.getMetadata().primaryColumns;
  }

  getId(): EntityId {
    const meta = (this.constructor as any).getMetadata();
    const id: EntityId = {};
    
    for (const col of meta.primaryColumns) {
      id[col.columnName] = this[col.propertyName];
    }
    
    return id;
  }

  setId(id: EntityId): void {
    const meta = (this.constructor as any).getMetadata();
    
    for (const col of meta.primaryColumns) {
      if (id[col.columnName] !== undefined) {
        this[col.propertyName] = id[col.columnName];
      }
    }
  }

  toJSON(): Record<string, any> {
    const obj: Record<string, any> = {};
    const meta = (this.constructor as any).getMetadata();
    
    for (const col of meta.columns) {
      if (this[col.propertyName] !== undefined) {
        obj[col.columnName] = this[col.propertyName];
      }
    }
    
    return obj;
  }

  assign(data: Partial<this>): this {
    for (const key of Object.keys(data)) {
      if (data[key as keyof this] !== undefined) {
        (this as any)[key] = data[key as keyof this];
      }
    }
    return this;
  }
}

export function getEntityMetadata<T extends BaseEntity>(entity: new () => T): EntityMetadata {
  const columns: ColumnMetadata[] = Reflect.getMetadata('orm:column', entity.prototype) || [];
  const relations: any[] = Reflect.getMetadata('orm:relation', entity.prototype) || [];
  const indices: any[] = Reflect.getMetadata('orm:index', entity.prototype) || [];
  
  const tableName = Reflect.getMetadata('orm:tableName', entity) || entity.name.toLowerCase();
  
  return {
    name: entity.name,
    tableName,
    columns,
    relations,
    indices,
    primaryColumns: columns.filter(c => c.primary),
  };
}

export function Entity(target: any): void {
  Reflect.defineMetadata('orm:entity', true, target);
}

export function Table(name: string): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('orm:tableName', name, target);
  };
}
