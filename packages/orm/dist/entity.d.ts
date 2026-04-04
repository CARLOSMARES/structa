import 'reflect-metadata';
import { EntityMetadata, ColumnMetadata } from './decorators';
export interface EntityId {
    [key: string]: any;
}
export declare abstract class BaseEntity {
    [key: string]: any;
    static getMetadata(): EntityMetadata;
    static getTableName(): string;
    static getColumns(): ColumnMetadata[];
    static getPrimaryColumns(): ColumnMetadata[];
    getId(): EntityId;
    setId(id: EntityId): void;
    toJSON(): Record<string, any>;
    assign(data: Partial<this>): this;
}
export declare function getEntityMetadata<T extends BaseEntity>(entity: new () => T): EntityMetadata;
export declare function Entity(target: any): void;
export declare function Table(name: string): ClassDecorator;
//# sourceMappingURL=entity.d.ts.map