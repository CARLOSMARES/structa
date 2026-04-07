# DSL Syntax

Structa uses a simple DSL syntax for defining application components. Files use the `.structa` extension.

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
| `guard` | Route protection (planned) |
| `gateway` | WebSocket gateway (planned) |
| `entity` | Database entity (planned) |

## Controllers

Controllers define HTTP routes using decorators:

```structa
controller UserController {
    path: "/users"
    
    @Get("/")
    async getAll() {
        return [{ id: 1, name: "John" }]
    }
    
    @Get("/:id")
    async getById(id) {
        return { id }
    }
    
    @Post("/")
    async create(data) {
        return { id: Date.now(), ...data }
    }
    
    @Put("/:id")
    async update(id, data) {
        return { id, ...data }
    }
    
    @Delete("/:id")
    async delete(id) {
        return { deleted: true }
    }
}
```

## Services

Services contain business logic and can be injected:

```structa
service UserService {
    @Inject("UserRepository")
    userRepo
    
    async findAll() {
        return this.userRepo.findAll()
    }
    
    async findById(id) {
        return this.userRepo.findById(id)
    }
    
    async create(data) {
        return this.userRepo.save(data)
    }
}
```

## DTOs

DTOs define data structures:

```structa
dto CreateUserDto {
    name: string
    email: string
    age: int
}
```

## Middleware

Middleware processes requests before handlers:

```structa
middleware LoggerMiddleware {
    async handle(req, res, next) {
        console.log(req.method, req.url)
        next()
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
```

## Decorator Reference

### HTTP Decorators

| Decorator | HTTP Method |
|-----------|-------------|
| `@Get(path)` | GET |
| `@Post(path)` | POST |
| `@Put(path)` | PUT |
| `@Patch(path)` | PATCH |
| `@Delete(path)` | DELETE |
| `@Options(path)` | OPTIONS |

## Parameter Types

```structa
controller ExampleController {
    @Get("/:id")           // Route params via :name
    async get(id) { }      // Access as id
    
    @Post("/")
    async create(data) {   // Request body
    }
}
```

## Dependency Injection

Inject services using `@Inject`:

```structa
service MyService {
    @Inject("OtherService")
    other
    
    @Inject("Repository")
    repo
    
    async doSomething() {
        return this.other.action()
    }
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
    
    async delete(id) {
        return { success: true }
    }
}

dto CreateUserDto {
    name: string
    email: string
}
```
