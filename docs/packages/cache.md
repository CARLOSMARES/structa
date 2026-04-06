# @structa/cache

Caching module with support for Memory, Redis, and File drivers.

## Installation

```bash
structa add @structa/cache
```

## Usage

### Basic Usage

```javascript
import { memoryCache } from '@structa/cache';

// Set cache
await memoryCache.set('user:1', { id: 1, name: 'John' }, 3600); // 1 hour TTL

// Get cache
const user = await memoryCache.get('user:1');

// Check exists
const exists = await memoryCache.has('user:1');

// Delete
await memoryCache.delete('user:1');

// Clear all
await memoryCache.clear();
```

### Get or Set

```javascript
// Get from cache or execute factory and cache result
const user = await memoryCache.getOrSet(
  'user:1',
  () => fetchUserFromDatabase(1),
  3600 // TTL in seconds
);
```

## Drivers

### Memory Driver (Default)

```javascript
import { createCacheService } from '@structa/cache';

const cache = createCacheService({
  driver: 'memory'
});
```

### Redis Driver

```javascript
import { createCacheService } from '@structa/cache';

const cache = createCacheService({
  driver: 'redis',
  connectionString: 'redis://localhost:6379'
});
```

### File Driver

```javascript
import { createCacheService } from '@structa/cache';

const cache = createCacheService({
  driver: 'file',
  cacheDir: './cache'
});
```

## Cache Service API

```javascript
const cache = createCacheService({ driver: 'memory' });

// Get value
await cache.get(key);

// Set value (TTL in seconds)
await cache.set(key, value, ttl);

// Check exists
await cache.has(key);

// Delete
await cache.delete(key);

// Delete by pattern
await cache.deletePattern('user:*');

// Clear all
await cache.clear();

// Get all keys
await cache.keys();

// Get or set
await cache.getOrSet(key, factory, ttl);
```

## Example: Caching Database Queries

```javascript
import { memoryCache } from '@structa/cache';

async function getUserWithCache(userId) {
  const cacheKey = `user:${userId}`;
  
  // Try cache first
  const cached = await memoryCache.get(cacheKey);
  if (cached) {
    return cached;
  }
  
  // Fetch from database
  const user = await db.users.findById(userId);
  
  // Cache for 1 hour
  await memoryCache.set(cacheKey, user, 3600);
  
  return user;
}
```

## Example: Cache with Redis

```javascript
import { createCacheService } from '@structa/cache';

const redisCache = createCacheService({
  driver: 'redis',
  connectionString: process.env.REDIS_URL
});

// Use in service
class UserService {
  constructor() {
    this.cache = redisCache;
  }

  async findById(id) {
    const key = `user:${id}`;
    const cached = await this.cache.get(key);
    
    if (cached) return cached;
    
    const user = await this.db.users.findById(id);
    await this.cache.set(key, user, 1800);
    
    return user;
  }

  async update(id, data) {
    const user = await this.db.users.update(id, data);
    await this.cache.delete(`user:${id}`);
    return user;
  }
}
```

## TTL (Time To Live)

TTL is specified in seconds:

```javascript
// Cache for 1 minute
await cache.set('key', value, 60);

// Cache for 1 hour
await cache.set('key', value, 3600);

// Cache for 1 day
await cache.set('key', value, 86400);

// Cache forever (until manually deleted)
await cache.set('key', value); // No TTL
```

## Configuration

```javascript
const cache = createCacheService({
  driver: 'memory',      // 'memory' | 'redis' | 'file'
  connectionString: 'redis://localhost:6379',  // Redis only
  cacheDir: './cache',     // File driver only
  defaultTtl: 3600         // Default TTL in seconds
});
```

## Pattern Deletion

```javascript
// Delete all user caches
await cache.deletePattern('user:*');

// Delete all caches
await cache.deletePattern('*');

// Delete specific patterns
await cache.deletePattern('product:category:*');
```
