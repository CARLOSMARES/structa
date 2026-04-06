# @structa/http

HTTP server module for Structa framework with routing, middleware, and request handling.

## Installation

```bash
structa add @structa/http
```

## Usage

```javascript
import { createApp, Controller, Get, Post } from '@structa/http';

@Controller()
class UserController {
  path = '/users';

  @Get('/')
  async list() {
    return [{ id: 1, name: 'John' }];
  }

  @Get('/:id')
  async getById(params) {
    return { id: params.id };
  }

  @Post('/')
  async create(@Body() data, @Query() query) {
    return { ...data, id: Date.now() };
  }
}

const app = createApp({
  controllers: [UserController],
  port: 3000
});

app.listen();
```

## Decorators

### Request Decorators

```javascript
import { 
  Body, Query, Params, Headers, Cookies,
  Req, Res 
} from '@structa/http';

// Get request body
@Post('/')
async create(@Body() body) { }

// Get query parameters
@Get('/search')
async search(@Query() query) { }

// Get route parameters
@Get('/:id')
async getById(@Params() params) { }

// Get headers
@Get('/')
async getHeaders(@Headers() headers) { }

// Get cookies
@Get('/')
async getCookies(@Cookies() cookies) { }
```

### Response Decorators

```javascript
import { 
  StatusCode, Header, SetCookie 
} from '@structa/http';

// Set status code
@Post('/')
@StatusCode(201)
async create() { }

// Set response header
@Get('/')
@Header('Cache-Control', 'no-cache')
async get() { }
```

## Middleware

### Built-in Middleware

```javascript
import { 
  json, urlencoded, cors, helmet, morgan 
} from '@structa/http';

const app = createApp({
  controllers: [UserController],
  middleware: [
    cors({ origin: '*' }),
    helmet(),
    json({ limit: '10mb' }),
    urlencoded({ extended: true }),
    morgan('combined')
  ]
});
```

### Custom Middleware

```javascript
import { createMiddleware } from '@structa/http';

const authMiddleware = createMiddleware(async (req, res, next) => {
  const token = req.headers.authorization;
  if (!token) {
    return res.status(401).json({ error: 'Unauthorized' });
  }
  req.user = await verifyToken(token);
  next();
});

const app = createApp({
  controllers: [UserController],
  middleware: [authMiddleware]
});
```

## Error Handling

```javascript
import { HttpException, NotFoundException } from '@structa/http';

// Throwing errors
@Get('/:id')
async getById(params) {
  const user = await findUser(params.id);
  if (!user) {
    throw new NotFoundException('User not found');
  }
  return user;
}

// Global error handler
app.use(async (err, req, res, next) => {
  console.error(err);
  res.status(err.status || 500).json({
    statusCode: err.status || 500,
    message: err.message
  });
});
```

## Configuration

```javascript
const app = createApp({
  controllers: [UserController, ProductController],
  middleware: [json(), cors()],
  port: 3000,
  host: '0.0.0.0',
  prefix: '/api/v1',
  onListen: (server) => {
    console.log(`Server running on port ${server.address().port}`);
  }
});
```

## Request/Response Types

```typescript
interface Request {
  method: string;
  url: string;
  path: string;
  query: Record<string, string>;
  params: Record<string, string>;
  headers: Record<string, string>;
  body: any;
  cookies: Record<string, string>;
  user?: any;
}

interface Response {
  status(code: number): this;
  json(data: any): this;
  send(data?: any): this;
  setHeader(name: string, value: string): this;
  setCookie(name: string, value: string, options?: CookieOptions): this;
  redirect(url: string): this;
}
```
