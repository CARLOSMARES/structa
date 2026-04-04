import 'reflect-metadata';
export const ENTITY_METADATA_KEY = 'orm:entity';
export const COLUMN_METADATA_KEY = 'orm:column';
export const RELATION_METADATA_KEY = 'orm:relation';
export const PRIMARY_KEY_METADATA_KEY = 'orm:primaryKey';
export const INDEX_METADATA_KEY = 'orm:index';
export var RelationType;
(function (RelationType) {
    RelationType["ONE_TO_ONE"] = "one-to-one";
    RelationType["ONE_TO_MANY"] = "one-to-many";
    RelationType["MANY_TO_ONE"] = "many-to-one";
    RelationType["MANY_TO_MANY"] = "many-to-many";
})(RelationType || (RelationType = {}));
export function Column(options) {
    return (target, propertyKey) => {
        const columns = Reflect.getMetadata(COLUMN_METADATA_KEY, target) || [];
        const column = {
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
export function PrimaryGeneratedColumn() {
    return Column({ type: 'int', primary: true, autoIncrement: true });
}
export function PrimaryColumn(type) {
    return Column({ type: type || 'int', primary: true });
}
export function CreateDateColumn() {
    return Column({
        type: 'datetime',
        name: 'createdAt',
        default: () => 'CURRENT_TIMESTAMP',
        nullable: false
    });
}
export function UpdateDateColumn() {
    return Column({
        type: 'datetime',
        name: 'updatedAt',
        default: () => 'CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP',
        onUpdate: true
    });
}
export function VersionColumn() {
    return Column({ type: 'int', default: 1 });
}
export function OneToOne(type, options) {
    return Relation(RelationType.ONE_TO_ONE, type, options);
}
export function OneToMany(type, options) {
    return Relation(RelationType.ONE_TO_MANY, type, options);
}
export function ManyToOne(type, options) {
    return Relation(RelationType.MANY_TO_ONE, type, options);
}
export function ManyToMany(type, options) {
    return Relation(RelationType.MANY_TO_MANY, type, options);
}
export function Relation(type, target, options) {
    return (targetClass, propertyKey) => {
        const relations = Reflect.getMetadata(RELATION_METADATA_KEY, targetClass) || [];
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
export function Index(name, columns) {
    return (target) => {
        const indices = Reflect.getMetadata(INDEX_METADATA_KEY, target) || [];
        indices.push({
            name: name || `idx_${target.name}`,
            columns: columns || [],
            unique: false,
        });
        Reflect.defineMetadata(INDEX_METADATA_KEY, indices, target);
    };
}
export function Unique(columns) {
    return (target) => {
        const indices = Reflect.getMetadata(INDEX_METADATA_KEY, target) || [];
        indices.push({
            name: `uniq_${target.name}_${columns.join('_')}`,
            columns,
            unique: true,
        });
        Reflect.defineMetadata(INDEX_METADATA_KEY, indices, target);
    };
}
//# sourceMappingURL=decorators.js.map