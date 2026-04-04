import 'reflect-metadata';
export class BaseEntity {
    static getMetadata() {
        throw new Error('Not implemented');
    }
    static getTableName() {
        const meta = this.getMetadata();
        return meta.tableName;
    }
    static getColumns() {
        return this.getMetadata().columns;
    }
    static getPrimaryColumns() {
        return this.getMetadata().primaryColumns;
    }
    getId() {
        const meta = this.constructor.getMetadata();
        const id = {};
        for (const col of meta.primaryColumns) {
            id[col.columnName] = this[col.propertyName];
        }
        return id;
    }
    setId(id) {
        const meta = this.constructor.getMetadata();
        for (const col of meta.primaryColumns) {
            if (id[col.columnName] !== undefined) {
                this[col.propertyName] = id[col.columnName];
            }
        }
    }
    toJSON() {
        const obj = {};
        const meta = this.constructor.getMetadata();
        for (const col of meta.columns) {
            if (this[col.propertyName] !== undefined) {
                obj[col.columnName] = this[col.propertyName];
            }
        }
        return obj;
    }
    assign(data) {
        for (const key of Object.keys(data)) {
            if (data[key] !== undefined) {
                this[key] = data[key];
            }
        }
        return this;
    }
}
export function getEntityMetadata(entity) {
    const columns = Reflect.getMetadata('orm:column', entity.prototype) || [];
    const relations = Reflect.getMetadata('orm:relation', entity.prototype) || [];
    const indices = Reflect.getMetadata('orm:index', entity.prototype) || [];
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
export function Entity(target) {
    Reflect.defineMetadata('orm:entity', true, target);
}
export function Table(name) {
    return (target) => {
        Reflect.defineMetadata('orm:tableName', name, target);
    };
}
//# sourceMappingURL=entity.js.map