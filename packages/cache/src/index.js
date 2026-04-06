export class CacheService {
  constructor(private driver: CacheDriver) {}

  async get<T>(key: string): Promise<T | null> {
    return this.driver.get(key);
  }

  async set<T>(key: string, value: T, ttl?: number): Promise<void> {
    return this.driver.set(key, value, ttl);
  }

  async delete(key: string): Promise<void> {
    return this.driver.delete(key);
  }

  async has(key: string): Promise<boolean> {
    return this.driver.has(key);
  }

  async clear(): Promise<void> {
    return this.driver.clear();
  }

  async getOrSet<T>(key: string, factory: () => T | Promise<T>, ttl?: number): Promise<T> {
    const cached = await this.get<T>(key);
    if (cached !== null) {
      return cached;
    }

    const value = await factory();
    await this.set(key, value, ttl);
    return value;
  }

  async deletePattern(pattern: string): Promise<void> {
    return this.driver.deletePattern(pattern);
  }

  async keys(): Promise<string[]> {
    return this.driver.keys();
  }
}

export interface CacheDriver {
  get<T>(key: string): Promise<T | null>;
  set<T>(key: string, value: T, ttl?: number): Promise<void>;
  delete(key: string): Promise<void>;
  has(key: string): Promise<boolean>;
  clear(): Promise<void>;
  deletePattern(pattern: string): Promise<void>;
  keys(): Promise<string[]>;
}

export class MemoryCacheDriver implements CacheDriver {
  private store = new Map<string, { value: any; expiresAt?: number }>();

  async get<T>(key: string): Promise<T | null> {
    const item = this.store.get(key);
    if (!item) return null;

    if (item.expiresAt && Date.now() > item.expiresAt) {
      this.store.delete(key);
      return null;
    }

    return item.value;
  }

  async set<T>(key: string, value: T, ttl?: number): Promise<void> {
    const expiresAt = ttl ? Date.now() + ttl * 1000 : undefined;
    this.store.set(key, { value, expiresAt });
  }

  async delete(key: string): Promise<void> {
    this.store.delete(key);
  }

  async has(key: string): Promise<boolean> {
    const item = this.store.get(key);
    if (!item) return false;

    if (item.expiresAt && Date.now() > item.expiresAt) {
      this.store.delete(key);
      return false;
    }

    return true;
  }

  async clear(): Promise<void> {
    this.store.clear();
  }

  async deletePattern(pattern: string): Promise<void> {
    const regex = new RegExp('^' + pattern.replace(/\*/g, '.*').replace(/\?/g, '.') + '$');
    for (const key of this.store.keys()) {
      if (regex.test(key)) {
        this.store.delete(key);
      }
    }
  }

  async keys(): Promise<string[]> {
    this.cleanup();
    return Array.from(this.store.keys());
  }

  private cleanup() {
    const now = Date.now();
    for (const [key, item] of this.store.entries()) {
      if (item.expiresAt && now > item.expiresAt) {
        this.store.delete(key);
      }
    }
  }
}

export class RedisCacheDriver implements CacheDriver {
  private client: any = null;

  constructor(private connectionString: string) {}

  private async getClient() {
    if (!this.client) {
      const { default: Redis } = await import('ioredis');
      this.client = new Redis(this.connectionString);
    }
    return this.client;
  }

  async get<T>(key: string): Promise<T | null> {
    const client = await this.getClient();
    const value = await client.get(key);
    return value ? JSON.parse(value) : null;
  }

  async set<T>(key: string, value: T, ttl?: number): Promise<void> {
    const client = await this.getClient();
    const serialized = JSON.stringify(value);
    if (ttl) {
      await client.setex(key, ttl, serialized);
    } else {
      await client.set(key, serialized);
    }
  }

  async delete(key: string): Promise<void> {
    const client = await this.getClient();
    await client.del(key);
  }

  async has(key: string): Promise<boolean> {
    const client = await this.getClient();
    return (await client.exists(key)) === 1;
  }

  async clear(): Promise<void> {
    const client = await this.getClient();
    await client.flushdb();
  }

  async deletePattern(pattern: string): Promise<void> {
    const client = await this.getClient();
    const regex = new RegExp('^' + pattern.replace(/\*/g, '.*').replace(/\?/g, '.') + '$');
    const keys = await client.keys(pattern);
    for (const key of keys) {
      if (regex.test(key)) {
        await client.del(key);
      }
    }
  }

  async keys(): Promise<string[]> {
    const client = await this.getClient();
    return client.keys('*');
  }
}

export class FileCacheDriver implements CacheDriver {
  private cacheDir: string;
  private store = new Map<string, { value: any; expiresAt?: number }>();

  constructor(cacheDir: string = './cache') {
    this.cacheDir = cacheDir;
  }

  private getFilePath(key: string): string {
    const safeName = key.replace(/[^a-zA-Z0-9_-]/g, '_');
    return `${this.cacheDir}/${safeName}.json`;
  }

  async get<T>(key: string): Promise<T | null> {
    const item = this.store.get(key);
    if (item) {
      if (item.expiresAt && Date.now() > item.expiresAt) {
        this.store.delete(key);
        return null;
      }
      return item.value;
    }
    return null;
  }

  async set<T>(key: string, value: T, ttl?: number): Promise<void> {
    const expiresAt = ttl ? Date.now() + ttl * 1000 : undefined;
    this.store.set(key, { value, expiresAt });
  }

  async delete(key: string): Promise<void> {
    this.store.delete(key);
  }

  async has(key: string): Promise<boolean> {
    return this.store.has(key);
  }

  async clear(): Promise<void> {
    this.store.clear();
  }

  async deletePattern(pattern: string): Promise<void> {
    const regex = new RegExp('^' + pattern.replace(/\*/g, '.*').replace(/\?/g, '.') + '$');
    for (const key of this.store.keys()) {
      if (regex.test(key)) {
        this.store.delete(key);
      }
    }
  }

  async keys(): Promise<string[]> {
    return Array.from(this.store.keys());
  }
}

export interface CacheConfig {
  driver: 'memory' | 'redis' | 'file';
  connectionString?: string;
  cacheDir?: string;
  defaultTtl?: number;
}

export function createCacheService(config: CacheConfig): CacheService {
  let driver: CacheDriver;

  switch (config.driver) {
    case 'redis':
      if (!config.connectionString) {
        throw new Error('Redis connection string is required');
      }
      driver = new RedisCacheDriver(config.connectionString);
      break;
    case 'file':
      driver = new FileCacheDriver(config.cacheDir || './cache');
      break;
    case 'memory':
    default:
      driver = new MemoryCacheDriver();
      break;
  }

  return new CacheService(driver);
}

export const memoryCache = new CacheService(new MemoryCacheDriver());
