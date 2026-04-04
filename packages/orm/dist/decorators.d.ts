import 'reflect-metadata';
export declare const ENTITY_METADATA_KEY = "orm:entity";
export declare const COLUMN_METADATA_KEY = "orm:column";
export declare const RELATION_METADATA_KEY = "orm:relation";
export declare const PRIMARY_KEY_METADATA_KEY = "orm:primaryKey";
export declare const INDEX_METADATA_KEY = "orm:index";
export interface ColumnOptions {
    name?: string;
    type: ColumnType;
    length?: number;
    precision?: number;
    scale?: number;
    nullable?: boolean;
    default?: any;
    unique?: boolean;
    primary?: boolean;
    autoIncrement?: boolean;
    enum?: any[];
    transformer?: ValueTransformer;
    onUpdate?: boolean;
}
export interface RelationOptions {
    type: RelationType;
    target: () => any;
    foreignKey?: string;
    nullable?: boolean;
    eager?: boolean;
    lazy?: boolean;
    cascade?: boolean;
    onDelete?: 'CASCADE' | 'SET NULL' | 'RESTRICT' | 'NO ACTION';
    onUpdate?: 'CASCADE' | 'SET NULL' | 'RESTRICT' | 'NO ACTION';
}
export declare enum RelationType {
    ONE_TO_ONE = "one-to-one",
    ONE_TO_MANY = "one-to-many",
    MANY_TO_ONE = "many-to-one",
    MANY_TO_MANY = "many-to-many"
}
export type ColumnType = 'int' | 'bigint' | 'smallint' | 'tinyint' | 'float' | 'double' | 'decimal' | 'boolean' | 'string' | 'text' | 'varchar' | 'char' | 'date' | 'time' | 'datetime' | 'timestamp' | 'json' | 'uuid' | 'enum';
export interface ValueTransformer {
    from(value: any): any;
    to(value: any): any;
}
export interface EntityMetadata {
    name: string;
    tableName: string;
    columns: ColumnMetadata[];
    relations: RelationMetadata[];
    indices: IndexMetadata[];
    primaryColumns: ColumnMetadata[];
}
export interface ColumnMetadata {
    propertyName: string;
    columnName: string;
    type: ColumnType;
    length?: number;
    precision?: number;
    scale?: number;
    nullable: boolean;
    default?: any;
    unique: boolean;
    primary: boolean;
    autoIncrement: boolean;
    enum?: any[];
    transformer?: ValueTransformer;
}
export interface RelationMetadata {
    propertyName: string;
    type: RelationType;
    target: () => any;
    foreignKey: string;
    nullable: boolean;
    eager: boolean;
    lazy: boolean;
    cascade: boolean;
}
export interface IndexMetadata {
    name: string;
    columns: string[];
    unique: boolean;
}
export declare function Column(options: ColumnOptions): PropertyDecorator;
export declare function PrimaryGeneratedColumn(): PropertyDecorator;
export declare function PrimaryColumn(type?: ColumnType): PropertyDecorator;
export declare function CreateDateColumn(): PropertyDecorator;
export declare function UpdateDateColumn(): PropertyDecorator;
export declare function VersionColumn(): PropertyDecorator;
export declare function OneToOne<T>(type: () => T, options?: Partial<RelationOptions>): PropertyDecorator;
export declare function OneToMany<T>(type: () => T, options?: Partial<RelationOptions>): PropertyDecorator;
export declare function ManyToOne<T>(type: () => T, options?: Partial<RelationOptions>): PropertyDecorator;
export declare function ManyToMany<T>(type: () => T, options?: Partial<RelationOptions>): PropertyDecorator;
export declare function Relation(type: RelationType, target: () => any, options?: Partial<RelationOptions>): PropertyDecorator;
export declare function Index(name?: string, columns?: string[]): ClassDecorator;
export declare function Unique(columns: string[]): ClassDecorator;
//# sourceMappingURL=decorators.d.ts.map