class Database {
    constructor(config) {
        this.config = config;
        this.connection = null;
        this.knex = null;
    }

    async connect() {
        const { type, ...options } = this.config;
        
        switch (type) {
            case 'mysql':
            case 'mariadb':
                const mysql = await import('mysql2/promise');
                this.connection = await mysql.createPool(options).getConnection();
                break;
            case 'postgresql':
                const { Client } = await import('pg');
                this.connection = new Client(options);
                await this.connection.connect();
                break;
            case 'sqlite':
                this.connection = await import('better-sqlite3');
                this.connection = new this.connection.default(options.filename || ':memory:');
                break;
            default:
                throw new Error(`Unsupported database type: ${type}`);
        }
        
        return this;
    }

    async query(sql, params = []) {
        const { type } = this.config;
        
        switch (type) {
            case 'mysql':
            case 'mariadb':
                const [rows] = await this.connection.execute(sql, params);
                return rows;
            case 'postgresql':
                const result = await this.connection.query(sql, params);
                return result.rows;
            case 'sqlite':
                const stmt = this.connection.prepare(sql);
                if (sql.trim().toUpperCase().startsWith('SELECT')) {
                    return stmt.all(...params);
                } else {
                    return stmt.run(...params);
                }
            default:
                throw new Error(`Unsupported database type: ${type}`);
        }
    }

    async close() {
        if (this.connection) {
            const { type } = this.config;
            if (type === 'postgresql') {
                await this.connection.end();
            } else if (type === 'mysql' || type === 'mariadb') {
                await this.connection.end();
            } else {
                this.connection.close();
            }
        }
    }
}

class Entity {
    static tableName = '';
    static columns = [];
    static primaryKey = 'id';
    static timestamps = true;

    constructor(data = {}) {
        this._data = { ...data };
        for (const key of Object.keys(data)) {
            this[key] = data[key];
        }
    }

    static async findAll(where = {}, options = {}) {
        const db = Database.getInstance();
        let sql = `SELECT * FROM ${this.tableName}`;
        const params = [];
        
        const whereKeys = Object.keys(where);
        if (whereKeys.length > 0) {
            sql += ' WHERE ' + whereKeys.map(k => `${k} = ?`).join(' AND ');
            params.push(...whereKeys.map(k => where[k]));
        }
        
        if (options.orderBy) {
            sql += ` ORDER BY ${options.orderBy}`;
        }
        
        if (options.limit) {
            sql += ` LIMIT ${options.limit}`;
        }
        
        const rows = await db.query(sql, params);
        return rows.map(row => new this(row));
    }

    static async findById(id) {
        const db = Database.getInstance();
        const sql = `SELECT * FROM ${this.tableName} WHERE ${this.primaryKey} = ? LIMIT 1`;
        const rows = await db.query(sql, [id]);
        return rows.length > 0 ? new this(rows[0]) : null;
    }

    static async findOne(where = {}) {
        const results = await this.findAll(where, { limit: 1 });
        return results.length > 0 ? results[0] : null;
    }

    async save() {
        const db = Database.getInstance();
        const tableName = this.constructor.tableName;
        const columns = this.constructor.columns;
        
        const data = {};
        for (const col of columns) {
            if (this[col] !== undefined) {
                data[col] = this[col];
            }
        }

        if (this[this.constructor.primaryKey]) {
            const sets = Object.keys(data).map(k => `${k} = ?`).join(', ');
            const sql = `UPDATE ${tableName} SET ${sets} WHERE ${this.constructor.primaryKey} = ?`;
            await db.query(sql, [...Object.values(data), this[this.constructor.primaryKey]]);
        } else {
            if (this.constructor.timestamps) {
                data.created_at = data.created_at || new Date().toISOString();
                data.updated_at = new Date().toISOString();
            }
            const cols = Object.keys(data).join(', ');
            const placeholders = Object.keys(data).map(() => '?').join(', ');
            const sql = `INSERT INTO ${tableName} (${cols}) VALUES (${placeholders})`;
            const result = await db.query(sql, Object.values(data));
            this[this.constructor.primaryKey] = result.insertId || result.lastInsertRowid;
        }
        
        return this;
    }

    async delete() {
        const db = Database.getInstance();
        const sql = `DELETE FROM ${this.constructor.tableName} WHERE ${this.constructor.primaryKey} = ?`;
        await db.query(sql, [this[this.constructor.primaryKey]]);
    }

    toJSON() {
        return { ...this._data };
    }
}

class DatabaseManager {
    constructor() {
        this.db = null;
        this.entities = new Map();
    }

    static getInstance() {
        if (!DatabaseManager.instance) {
            DatabaseManager.instance = new DatabaseManager();
        }
        return DatabaseManager.instance;
    }

    async init(config) {
        this.db = new Database(config);
        await this.db.connect();
        return this;
    }

    getConnection() {
        return this.db;
    }

    registerEntity(entityClass) {
        this.entities.set(entityClass.tableName, entityClass);
        return this;
    }

    getEntity(tableName) {
        return this.entities.get(tableName);
    }

    async createTables() {
        for (const [tableName, EntityClass] of this.entities) {
            const columns = EntityClass.columns.map(col => {
                let def = `${col.name} ${col.type}`;
                if (col.primaryKey) def += ' PRIMARY KEY';
                if (col.autoIncrement) def += ' AUTO_INCREMENT';
                if (col.nullable === false) def += ' NOT NULL';
                if (col.default !== undefined) def += ` DEFAULT ${col.default}`;
                return def;
            });

            if (EntityClass.timestamps) {
                columns.push('created_at DATETIME DEFAULT CURRENT_TIMESTAMP');
                columns.push('updated_at DATETIME DEFAULT CURRENT_TIMESTAMP');
            }

            const sql = `CREATE TABLE IF NOT EXISTS ${tableName} (${columns.join(', ')})`;
            await this.db.query(sql);
        }
    }

    async dropTables() {
        for (const [tableName] of this.entities) {
            await this.db.query(`DROP TABLE IF EXISTS ${tableName}`);
        }
    }
}

function defineEntity(tableName, columns, options = {}) {
    return class extends Entity {
        static tableName = tableName;
        static columns = columns;
        static primaryKey = options.primaryKey || 'id';
        static timestamps = options.timestamps !== false;
    };
}

async function createDatabase(config) {
    const db = new Database(config);
    await db.connect();
    return db;
}

export { 
    Database, 
    Entity, 
    DatabaseManager, 
    defineEntity, 
    createDatabase 
};
