# DSL Syntax

Structa uses a simple DSL syntax for defining application components. Files use the `.structa` extension and compile to JavaScript.

## Basic Structure

```structa
// This is a comment

controller UserController {
    path: "/users"
    
    @Get("/")
    async getAll() {
        return []
    }
    
    @Get("/:id")
    async getById(id) {
        return null
    }
}

dto UserDto {
    name: string
    email: string
}
```

## Keywords

| Keyword | Description |
|---------|-------------|
| `controller` | HTTP request handler with routes |
| `service` | Business logic layer |
| `repository` | Data access layer |
| `dto` | Data transfer object |
| `middleware` | Request/response middleware |
| `module` | Application module |
| `resolver` | GraphQL resolver |
| `entity` | Database entity |

## Controllers

Controllers define HTTP routes using decorators:

```structa
controller UserController {
    path: "/users"
    
    @Inject("UserService")
    userService
    
    @Get("/")
    async getAll() {
        return await this.userService.findAll()
    }
    
    @Get("/:id")
    async getById(id) {
        return await this.userService.findById(id)
    }
    
    @Post("/")
    async create(data) {
        return await this.userService.create(data)
    }
    
    @Put("/:id")
    async update(id, data) {
        return await this.userService.update(id, data)
    }
    
    @Delete("/:id")
    async delete(id) {
        return await this.userService.delete(id)
    }
    
    @Patch("/:id/status")
    async updateStatus(id, status) {
        return await this.userService.updateStatus(id, status)
    }
}
```

## Services

Services contain business logic and can be injected:

```structa
service UserService {
    @Inject("UserRepository")
    userRepo
    
    @Inject("CacheService")
    cache
    
    async findAll() {
        const cached = await this.cache.get("users:all")
        if (cached) return cached
        
        const users = await this.userRepo.findAll()
        await this.cache.set("users:all", users, 300)
        return users
    }
    
    async findById(id) {
        const cached = await this.cache.get(`users:${id}`)
        if (cached) return cached
        
        const user = await this.userRepo.findById(id)
        if (user) await this.cache.set(`users:${id}`, user, 300)
        return user
    }
    
    async create(data) {
        const user = await this.userRepo.save(data)
        await this.cache.delete("users:all")
        return user
    }
    
    async update(id, data) {
        const user = await this.userRepo.update(id, data)
        await this.cache.delete(`users:${id}`)
        await this.cache.delete("users:all")
        return user
    }
    
    async delete(id) {
        await this.userRepo.delete(id)
        await this.cache.delete(`users:${id}`)
        await this.cache.delete("users:all")
        return { success: true }
    }
}
```

## Repositories

Repositories handle data access:

```structa
repository UserRepository {
    async findAll() {
        return [
            { id: 1, name: "John", email: "john@example.com" },
            { id: 2, name: "Jane", email: "jane@example.com" }
        ]
    }
    
    async findById(id) {
        return { id, name: "John", email: "john@example.com" }
    }
    
    async save(data) {
        return { id: Date.now(), ...data }
    }
    
    async update(id, data) {
        return { id, ...data, updated: true }
    }
    
    async delete(id) {
        return { success: true }
    }
}
```

## DTOs

DTOs define data structures:

```structa
dto CreateUserDto {
    name: string
    email: string
    password: string
}

dto UpdateUserDto {
    name?: string
    email?: string
}

dto UserDto {
    id: int
    name: string
    email: string
    createdAt: datetime
}
```

## Middleware

Middleware processes requests before handlers:

```structa
middleware LoggerMiddleware {
    async handle(req, res, next) {
        const start = Date.now()
        console.log(req.method, req.url)
        await next()
        console.log(req.method, req.url, Date.now() - start + "ms")
    }
}

middleware AuthMiddleware {
    async handle(req, res, next) {
        const token = req.headers.authorization
        if (!token) {
            res.status(401).json({ error: "Unauthorized" })
            return
        }
        req.user = await verifyToken(token)
        await next()
    }
}

middleware CorsMiddleware {
    async handle(req, res, next) {
        res.setHeader("Access-Control-Allow-Origin", "*")
        res.setHeader("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH")
        res.setHeader("Access-Control-Allow-Headers", "Content-Type, Authorization")
        await next()
    }
}
```

## Modules

Modules group related components:

```structa
module UserModule {
    controllers: [UserController]
    services: [UserService]
    repositories: [UserRepository]
}

module AppModule {
    controllers: [UserController, ProductController]
    services: [UserService, ProductService]
    repositories: [UserRepository, ProductRepository]
    middleware: [LoggerMiddleware, AuthMiddleware]
}
```

## Decorator Reference

### HTTP Decorators

| Decorator | HTTP Method | Path |
|-----------|-------------|------|
| `@Get(path)` | GET | path |
| `@Post(path)` | POST | path |
| `@Put(path)` | PUT | path |
| `@Patch(path)` | PATCH | path |
| `@Delete(path)` | DELETE | path |
| `@Options(path)` | OPTIONS | path |

### DI Decorator

| Decorator | Description |
|-----------|-------------|
| `@Inject(name)` | Inject a dependency |

## Data Types

```structa
dto ExampleDto {
    id: int           // Integer
    name: string      // String
    price: float      // Float/Decimal
    active: boolean   // Boolean
    createdAt: date  // Date
    updatedAt: datetime // DateTime
    metadata: json    // JSON object
}
```

## Full Example

```structa
// Main application file

controller UserController {
    path: "/users"
    
    @Inject("UserService")
    userService
    
    @Get("/")
    async getAll() {
        return await this.userService.findAll()
    }
    
    @Get("/:id")
    async getById(id) {
        return await this.userService.findById(id)
    }
    
    @Post("/")
    async create(data) {
        return await this.userService.create(data)
    }
    
    @Put("/:id")
    async update(id, data) {
        return await this.userService.update(id, data)
    }
    
    @Delete("/:id")
    async delete(id) {
        return await this.userService.delete(id)
    }
}

service UserService {
    @Inject("UserRepository")
    userRepo
    
    async findAll() {
        return await this.userRepo.findAll()
    }
    
    async findById(id) {
        return await this.userRepo.findById(id)
    }
    
    async create(data) {
        return await this.userRepo.save(data)
    }
    
    async update(id, data) {
        return await this.userRepo.update(id, data)
    }
    
    async delete(id) {
        return await this.userRepo.delete(id)
    }
}

repository UserRepository {
    async findAll() {
        return [
            { id: 1, name: "John", email: "john@example.com" },
            { id: 2, name: "Jane", email: "jane@example.com" }
        ]
    }
    
    async findById(id) {
        return { id, name: "John", email: "john@example.com" }
    }
    
    async save(data) {
        return { id: Date.now(), ...data }
    }
    
    async update(id, data) {
        return { id, ...data, updated: true }
    }
    
    async delete(id) {
        return { success: true }
    }
}

dto CreateUserDto {
    name: string
    email: string
    password: string
}

dto UserDto {
    id: int
    name: string
    email: string
}
```
