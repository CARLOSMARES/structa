# test-api - Structa API

A TypeScript API built with Structa framework.

## 🚀 Quick Start

```bash
# Install dependencies
structa install

# Start development server
structa dev

# Build for production
structa build
```

## 📦 Available Packages

Install additional packages with `structa add-package`:

```bash
structa add-package @structa/orm      # Database ORM
structa add-package @structa/graphql  # GraphQL support
structa add-package @structa/websocket # WebSocket support
structa add-package @structa/swagger   # API documentation
structa add-package @structa/testing    # Testing utilities
```

## 📁 Project Structure

```
src/
├── controllers/    # Route handlers
├── services/       # Business logic
├── dtos/           # Data transfer objects
├── modules/        # App modules
├── entities/       # Database entities
├── routes/         # Route definitions
├── app.structa     # App configuration
└── main.ts         # Entry point
```

## 🌐 API Endpoints

| Method | Endpoint           | Description           |
|--------|-------------------|----------------------|
| GET    | /api/users        | Get all users        |
| GET    | /api/users/:id    | Get user by ID      |
| POST   | /api/users        | Create new user      |
| PUT    | /api/users/:id    | Update user          |
| DELETE | /api/users/:id    | Delete user          |

## 📚 Documentation

API documentation available at `/docs` when server is running.

## 📜 License

MIT
