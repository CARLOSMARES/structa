use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub async fn run(path: PathBuf, name: String, _template: String) -> Result<()> {
    info!("Initializing Structa project: {} at {:?}", name, path);
    
    let project_dir = path.join(&name);
    
    std::fs::create_dir_all(&project_dir)?;
    
    let src_dir = project_dir.join("src");
    let controllers_dir = src_dir.join("controllers");
    let services_dir = src_dir.join("services");
    let dtos_dir = src_dir.join("dtos");
    let repositories_dir = src_dir.join("repositories");
    let middleware_dir = src_dir.join("middleware");
    
    std::fs::create_dir_all(&controllers_dir)?;
    std::fs::create_dir_all(&services_dir)?;
    std::fs::create_dir_all(&dtos_dir)?;
    std::fs::create_dir_all(&repositories_dir)?;
    std::fs::create_dir_all(&middleware_dir)?;

    // Create user.dto.structa
    let user_dto_file = dtos_dir.join("user.dto.structa");
    let user_dto_content = r#"dto User {
    id: string
    name: string
    email: string
}

dto CreateUserDto {
    name: string
    email: string
    password: string
}

dto UpdateUserDto {
    name: string
    email: string
}
"#;
    std::fs::write(&user_dto_file, user_dto_content)?;

    // Create user.repository.structa
    let user_repo_file = repositories_dir.join("user.repository.structa");
    let user_repo_content = r#"repository UserRepository {
    db: Database

    async findAll() {
        return this.db.query("SELECT * FROM users")
    }

    async findById(id) {
        const result = this.db.query("SELECT * FROM users WHERE id = ?", [id])
        return result[0] || null
    }

    async create(data) {
        return this.db.query(
            "INSERT INTO users (name, email) VALUES (?, ?)",
            [data.name, data.email]
        )
    }

    async update(id, data) {
        return this.db.query(
            "UPDATE users SET name = ?, email = ? WHERE id = ?",
            [data.name, data.email, id]
        )
    }

    async delete(id) {
        return this.db.query("DELETE FROM users WHERE id = ?", [id])
    }
}
"#;
    std::fs::write(&user_repo_file, user_repo_content)?;

    // Create user.service.structa
    let user_service_file = services_dir.join("user.service.structa");
    let user_service_content = r#"service UserService {
    @Inject
    userRepo: UserRepository

    async findAll() {
        return await this.userRepo.findAll()
    }

    async findById(id) {
        return await this.userRepo.findById(id)
    }

    async create(data) {
        return await this.userRepo.create(data)
    }

    async update(id, data) {
        return await this.userRepo.update(id, data)
    }

    async delete(id) {
        return await this.userRepo.delete(id)
    }
}
"#;
    std::fs::write(&user_service_file, user_service_content)?;

    // Create user.controller.structa
    let user_controller_file = controllers_dir.join("user.controller.structa");
    let user_controller_content = r#"controller UserController {
    path "/users"

    @Inject
    userService: UserService

    @Get("/")
    async findAll(req, res) {
        return await this.userService.findAll()
    }

    @Get("/:id")
    async findById(req, res) {
        const user = await this.userService.findById(req.params.id)
        if (!user) {
            res.writeHead(404)
            return { error: "User not found" }
        }
        return user
    }

    @Post("/")
    async create(req, res) {
        return await this.userService.create(req.body)
    }

    @Put("/:id")
    async update(req, res) {
        return await this.userService.update(req.params.id, req.body)
    }

    @Delete("/:id")
    async delete(req, res) {
        return await this.userService.delete(req.params.id)
    }
}
"#;
    std::fs::write(&user_controller_file, user_controller_content)?;

    // Create main.structa
    let main_file = src_dir.join("main.structa");
    let main_content = r#"controller HomeController {
    path "/"

    @Get("/")
    index() {
        return {
            name: "Structa API",
            version: "0.7.0",
            status: "running"
        }
    }

    @Get("/health")
    health() {
        return { status: "ok" }
    }
}

middleware LoggerMiddleware {
    handle(req, res, next) {
        console.log(`${new Date().toISOString()} ${req.method} ${req.url}`)
        next()
    }
}
"#;
    std::fs::write(&main_file, main_content)?;
    
    // Create package.json
    let package_json = project_dir.join("package.json");
    let package_content = format!(r#"{{
    "name": "{}",
    "version": "0.1.0",
    "type": "module",
    "description": "Structa API - TypeScript-like framework in Rust",
    "scripts": {{
        "dev": "structa dev",
        "start": "node dist/src/main.js",
        "build": "structa build"
    }},
    "dependencies": {{
        "@structa/http": "^{}"
    }}
}}"#, name, "0.7.0");
    std::fs::write(&package_json, package_content)?;

    // Create structa.config.json
    let config_file = project_dir.join("structa.config.json");
    let config_content = r#"{
    "name": "structa-project",
    "version": "0.1.0",
    "compiler": {
        "target": "es2020",
        "module": "esnext"
    },
    "server": {
        "port": 3000,
        "host": "0.0.0.0",
        "prefix": "/api"
    },
    "database": {
        "type": "sqlite",
        "url": "sqlite://database.sqlite"
    }
}
"#;
    std::fs::write(&config_file, config_content)?;

    // Create README.md
    let readme_file = project_dir.join("README.md");
    let readme_content = format!(r#"# {}

## Structa API

A TypeScript-like API framework powered by Rust.

## Getting Started

```bash
npm install
npm run dev
```

## Project Structure

```
{}
├── src/
│   ├── controllers/    # Route handlers
│   ├── services/       # Business logic
│   ├── repositories/    # Data access
│   ├── dtos/          # Data transfer objects
│   ├── middleware/    # Custom middleware
│   └── main.structa   # Main entry point
├── package.json
└── structa.config.json
```

## Routes

- GET    /api/users      - List all users
- GET    /api/users/:id  - Get user by ID
- POST   /api/users      - Create user
- PUT    /api/users/:id  - Update user
- DELETE /api/users/:id  - Delete user

## Learn More

Visit https://structa.dev for documentation
"#, name, name);
    std::fs::write(&readme_file, readme_content)?;

    info!("Project {} initialized successfully!", name);
    
    println!();
    println!("\x1b[32m╔══════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[32m║\x1b[0m \x1b[32m  Structa Project Created Successfully!        \x1b[0m\x1b[32m║\x1b[0m");
    println!("\x1b[32m╠══════════════════════════════════════════════════════╣\x1b[0m");
    println!("\x1b[32m║\x1b[0m \x1b[36m📁 Project:\x1b[0m \x1b[33m{}\x1b[0m                              \x1b[32m║\x1b[0m", name);
    println!("\x1b[32m╚══════════════════════════════════════════════════════╝\x1b[0m");
    println!();
    println!("\x1b[36m📦 Next steps:\x1b[0m");
    println!("   \x1b[32mcd {}\x1b[0m", name);
    println!("   \x1b[32mnpm install\x1b[0m");
    println!("   \x1b[32mnpm run dev\x1b[0m");
    println!();
    println!("\x1b[36m🌐 API will be available at:\x1b[0m");
    println!("   \x1b[32mhttp://localhost:3000/api/users\x1b[0m");
    println!();
    
    Ok(())
}
