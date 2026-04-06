# DSL Syntax

Structa uses a TypeScript-like syntax with decorators for defining application components.

## Basic Structure

```structa
// Comments start with //

controller MyController {
    path: "/resource"
    
    @Get("/")
    async getAll(): MyDto[] {
        return []
    }
}

dto MyDto {
    id: int
    name: string
    email?: string
}
```

## Keywords

| Keyword | Description |
|---------|-------------|
| `controller` | HTTP request handler |
| `service` | Business logic layer |
| `module` | Application module |
| `middleware` | Request/response middleware |
| `guard` | Route protection |
| `resolver` | GraphQL resolver |
| `gateway` | WebSocket gateway |
| `dto` | Data transfer object |
| `entity` | Database entity |

## Decorators

### HTTP Decorators

```structa
controller UserController {
    path: "/users"
    
    @Get("/")
    async list(): User[] { }
    
    @Get("/:id")
    async getById(id: string): User | null { }
    
    @Post("/")
    async create(data: CreateUserDto): User { }
    
    @Put("/:id")
    async update(id: string, data: UpdateUserDto): User { }
    
    @Delete("/:id")
    async remove(id: string): void { }
    
    @Patch("/:id/status")
    async updateStatus(id: string, status: string): User { }
}
```

### GraphQL Decorators

```structa
resolver UserResolver {
    @Query()
    async users(): User[] { }
    
    @Query("user")
    async userById(args: { id: string }): User | null { }
    
    @Mutation()
    async createUser(data: CreateUserInput): User { }
    
    @Subscription()
    async userCreated(): AsyncIterator<User> { }
}
```

### WebSocket Decorators

```structa
gateway ChatGateway {
    namespace: "/chat"
    
    @SubscribeMessage("message")
    async handleMessage(client: any, payload: MessagePayload): MessageResponse { }
    
    @SubscribeMessage("join")
    async handleJoin(client: any, room: string): void { }
    
    afterInit(server: any): void { }
    
    handleDisconnect(client: any): void { }
}
```

## Data Types

```structa
dto UserDto {
    // Primitives
    id: int
    name: string
    age: float
    active: boolean
    createdAt: datetime
    birthDate: date
    
    // Optional
    email?: string
    phone?: string
    
    // Arrays
    tags: string[]
    scores: int[]
    
    // Nested
    address: AddressDto
    
    // Map
    metadata: object
}
```

## Type Modifiers

```structa
dto ProductDto {
    id: int pk auto          // Primary key with auto-increment
    name: string length(100) // String with max length
    price: decimal(10,2)     // Decimal with precision
    stock: int default(0)    // Default value
    email: string unique     // Unique constraint
    category?: string        // Optional (nullable)
}
```

## Entity Definitions

```structa
entity User {
    table: users
    
    id: int pk auto
    name: string length(100) notNull
    email: string unique notNull
    password: string notNull
    createdAt: datetime
    updatedAt: datetime
    
    // Relations
    posts: Post[] relation(hasMany: Post)
    
    // Indexes
    index: [email] unique
    index: [createdAt]
}
```

## Middleware

```structa
middleware AuthMiddleware {
    async use(req: Request, res: Response, next: NextFunction): Promise<void> {
        const token = req.headers.authorization
        if (!token) {
            res.status(401).json({ error: "Unauthorized" })
            return
        }
        next()
    }
}
```

## Guards

```structa
guard RolesGuard {
    roles: string[] = ["admin"]
    
    canActivate(context: ExecutionContext): boolean {
        const user = context.request.user
        return this.roles.includes(user.role)
    }
}
```

## Modules

```structa
module UserModule {
    controllers: [UserController]
    services: [UserService]
    exports: [UserService]
    
    providers: [Database]
}
```

## Full Example

```structa
// User Controller
controller UserController {
    path: "/users"
    middleware: [AuthMiddleware]
    
    @Get("/")
    async list(): User[] {
        return this.userService.findAll()
    }
    
    @Get("/:id")
    async getById(id: string): User | null {
        return this.userService.findById(id)
    }
    
    @Post("/")
    async create(data: CreateUserDto): User {
        return this.userService.create(data)
    }
}

// User Service
service UserService {
    async findAll(): User[] {
        return this.userRepository.findAll()
    }
    
    async findById(id: string): User | null {
        return this.userRepository.findById(id)
    }
    
    async create(data: CreateUserDto): User {
        const user = new User(data)
        return this.userRepository.save(user)
    }
}

// DTOs
dto CreateUserDto {
    name: string
    email: string
    password: string
}

dto User {
    id: int
    name: string
    email: string
    createdAt: datetime
}

// Middleware
middleware AuthMiddleware {
    async use(req: Request, res: Response, next: NextFunction): Promise<void> {
        // Check authentication
        next()
    }
}
```

## Routes Summary

| Decorator | Method | Path |
|-----------|--------|------|
| `@Get(path)` | GET | path |
| `@Post(path)` | POST | path |
| `@Put(path)` | PUT | path |
| `@Patch(path)` | PATCH | path |
| `@Delete(path)` | DELETE | path |
| `@Options(path)` | OPTIONS | path |
| `@Head(path)` | HEAD | path |
