import 'reflect-metadata';

export const ENTITY_METADATA_KEY = 'orm:entity';
export const COLUMN_METADATA_KEY = 'orm:column';
export const RELATION_METADATA_KEY = 'orm:relation';
export const PRIMARY_KEY_METADATA_KEY = 'orm:primaryKey';
export const INDEX_METADATA_KEY = 'orm:index';

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

export enum RelationType {
  ONE_TO_ONE = 'one-to-one',
  ONE_TO_MANY = 'one-to-many',
  MANY_TO_ONE = 'many-to-one',
  MANY_TO_MANY = 'many-to-many',
}

export type ColumnType = 
  | 'int'
  | 'bigint'
  | 'smallint'
  | 'tinyint'
  | 'float'
  | 'double'
  | 'decimal'
  | 'boolean'
  | 'string'
  | 'text'
  | 'varchar'
  | 'char'
  | 'date'
  | 'time'
  | 'datetime'
  | 'timestamp'
  | 'json'
  | 'uuid'
  | 'enum';

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

export function Column(options: ColumnOptions): PropertyDecorator {
  return (target: any, propertyKey: string | symbol) => {
    const columns: ColumnMetadata[] = Reflect.getMetadata(COLUMN_METADATA_KEY, target) || [];
    
    const column: ColumnMetadata = {
      propertyName: String(propertyKey),
      columnName: options.name || String(propertyKey),
      type: options.type,
      length: options.length,
      precision: options.precision,
      scale: options.scale,
      nullable: options.nullable ?? false,
      default: options.default,
      unique: options.unique ?? false,
      primary: options.primary ?? false,
      autoIncrement: options.autoIncrement ?? false,
      enum: options.enum,
      transformer: options.transformer,
    };
    
    columns.push(column);
    Reflect.defineMetadata(COLUMN_METADATA_KEY, columns, target);
  };
}

export function PrimaryGeneratedColumn(): PropertyDecorator {
  return Column({ type: 'int', primary: true, autoIncrement: true });
}

export function PrimaryColumn(type?: ColumnType): PropertyDecorator {
  return Column({ type: type || 'int', primary: true });
}

export function CreateDateColumn(): PropertyDecorator {
  return Column({ 
    type: 'datetime', 
    name: 'createdAt',
    default: () => 'CURRENT_TIMESTAMP',
    nullable: false 
  });
}

export function UpdateDateColumn(): PropertyDecorator {
  return Column({ 
    type: 'datetime', 
    name: 'updatedAt',
    default: () => 'CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP',
    onUpdate: true
  });
}

export function VersionColumn(): PropertyDecorator {
  return Column({ type: 'int', default: 1 });
}

export function OneToOne<T>(type: () => T, options?: Partial<RelationOptions>): PropertyDecorator {
  return Relation(RelationType.ONE_TO_ONE, type, options);
}

export function OneToMany<T>(type: () => T, options?: Partial<RelationOptions>): PropertyDecorator {
  return Relation(RelationType.ONE_TO_MANY, type, options);
}

export function ManyToOne<T>(type: () => T, options?: Partial<RelationOptions>): PropertyDecorator {
  return Relation(RelationType.MANY_TO_ONE, type, options);
}

export function ManyToMany<T>(type: () => T, options?: Partial<RelationOptions>): PropertyDecorator {
  return Relation(RelationType.MANY_TO_MANY, type, options);
}

export function Relation(type: RelationType, target: () => any, options?: Partial<RelationOptions>): PropertyDecorator {
  return (targetClass: any, propertyKey: string | symbol) => {
    const relations: RelationMetadata[] = Reflect.getMetadata(RELATION_METADATA_KEY, targetClass) || [];
    
    relations.push({
      propertyName: String(propertyKey),
      type,
      target,
      foreignKey: options?.foreignKey || `${String(propertyKey)}Id`,
      nullable: options?.nullable ?? true,
      eager: options?.eager ?? false,
      lazy: options?.lazy ?? true,
      cascade: options?.cascade ?? false,
    });
    
    Reflect.defineMetadata(RELATION_METADATA_KEY, relations, targetClass);
  };
}

export function Index(name?: string, columns?: string[]): ClassDecorator {
  return (target: any) => {
    const indices: IndexMetadata[] = Reflect.getMetadata(INDEX_METADATA_KEY, target) || [];
    
    indices.push({
      name: name || `idx_${target.name}`,
      columns: columns || [],
      unique: false,
    });
    
    Reflect.defineMetadata(INDEX_METADATA_KEY, indices, target);
  };
}

export function Unique(columns: string[]): ClassDecorator {
  return (target: any) => {
    const indices: IndexMetadata[] = Reflect.getMetadata(INDEX_METADATA_KEY, target) || [];
    
    indices.push({
      name: `uniq_${target.name}_${columns.join('_')}`,
      columns,
      unique: true,
    });
    
    Reflect.defineMetadata(INDEX_METADATA_KEY, indices, target);
  };
}
