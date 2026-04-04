use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub async fn run(path: PathBuf, name: String, template: String) -> Result<()> {
    info!("Initializing Structa project: {} at {:?}", name, path);
    
    let project_dir = path.join(&name);
    
    std::fs::create_dir_all(&project_dir)?;
    
    // Create directory structure
    let src_dir = project_dir.join("src");
    let controllers_dir = project_dir.join("src/controllers");
    let services_dir = project_dir.join("src/services");
    let dtos_dir = project_dir.join("src/dtos");
    let modules_dir = project_dir.join("src/modules");
    let entities_dir = project_dir.join("src/entities");
    let routes_dir = project_dir.join("src/routes");
    
    std::fs::create_dir_all(&controllers_dir)?;
    std::fs::create_dir_all(&services_dir)?;
    std::fs::create_dir_all(&dtos_dir)?;
    std::fs::create_dir_all(&modules_dir)?;
    std::fs::create_dir_all(&entities_dir)?;
    std::fs::create_dir_all(&routes_dir)?;

    // Create main app.structa
    let app_file = src_dir.join("app.structa");
    let app_content = format!(r#"app {} {{
    port: 3000
    host: "0.0.0.0"
    
    modules: [AppModule]
    
    swagger(
        title: "{} API",
        description: "API documentation",
        version: "1.0.0",
        path: "/docs"
    )
    
    cors(origin: "*", credentials: true)
}}"#, name, name);
    std::fs::write(&app_file, app_content)?;
    
    // Create main.ts entry point
    let main_file = src_dir.join("main.ts");
    let main_content = format!(r#"import 'reflect-metadata';
import {{ StructaApp, createHttpServer }} from '@structa/runtime';

/**
 * {0} Application Entry Point
 */
const app = new StructaApp();
const port = parseInt(process.env.PORT || '3000');
const host = process.env.HOST || '0.0.0.0';

app.listen(port, host).then(() => {{
    console.log("🚀 {0} is running on http://" + host + ":" + port);
    console.log("📚 API Documentation: http://" + host + ":" + port + "/docs");
}});
"#, name);
    std::fs::write(&main_file, main_content)?;
    
    // Create User controller
    let user_controller = controllers_dir.join("user.controller.ts");
    std::fs::write(&user_controller, USER_CONTROLLER)?;
    
    // Create User service
    let user_service = services_dir.join("user.service.ts");
    std::fs::write(&user_service, USER_SERVICE)?;
    
    // Create User DTOs
    let user_dto = dtos_dir.join("user.dto.ts");
    std::fs::write(&user_dto, USER_DTO)?;
    
    // Create User entity
    let user_entity = entities_dir.join("user.entity.ts");
    std::fs::write(&user_entity, USER_ENTITY)?;
    
    // Create App module
    let app_module = modules_dir.join("app.module.ts");
    std::fs::write(&app_module, APP_MODULE)?;
    
    // Create structa.json config
    let structa_config = project_dir.join("structa.json");
    let config_content = format!(r#"{{
    "name": "{}",
    "template": "{}",
    "version": "0.1.0",
    "compiler": {{
        "target": "es2022",
        "module": "commonjs",
        "strict": true,
        "experimentalDecorators": true,
        "emitDecoratorMetadata": true,
        "outDir": "./dist",
        "rootDir": "./src",
        "declaration": true,
        "sourceMap": true,
        "esModuleInterop": true,
        "skipLibCheck": true
    }},
    "server": {{
        "port": 3000,
        "host": "0.0.0.0",
        "cors": {{
            "origin": "*",
            "credentials": true
        }},
        "prefix": "/api"
    }},
    "database": {{
        "type": "sqlite",
        "database": "./data.db"
    }},
    "swagger": {{
        "title": "{} API",
        "description": "API Documentation",
        "version": "1.0.0",
        "path": "/docs"
    }}
}}"#, name, template, name);
    std::fs::write(&structa_config, config_content)?;
    
    // Create package.json
    let package_json = project_dir.join("package.json");
    let package_content = format!(r#"{{
    "name": "{}",
    "version": "0.1.0",
    "description": "Structa API project - TypeScript framework powered by Rust",
    "main": "dist/main.js",
    "scripts": {{
        "dev": "structa dev --no-compile",
        "build": "structa build",
        "start": "node dist/main.js",
        "test": "structa test"
    }},
    "keywords": ["structa", "api", "framework", "typescript"],
    "author": "",
    "license": "MIT",
    "dependencies": {{
        "@structa/runtime": "^0.6.3",
        "@structa/http": "^0.6.3",
        "reflect-metadata": "^0.1.13"
    }},
    "devDependencies": {{
        "typescript": "^5.0.0",
        "@types/node": "^20.0.0",
        "tsx": "^4.21.0"
    }}
}}"#, name);
    std::fs::write(&package_json, package_content)?;
    
    // Create tsconfig.json
    let tsconfig = project_dir.join("tsconfig.json");
    std::fs::write(&tsconfig, TSCONFIG)?;
    
    // Create README.md
    let readme = project_dir.join("README.md");
    let readme_content = README.replace("{name}", &name);
    std::fs::write(&readme, readme_content)?;
    
    // Create .gitignore
    let gitignore = project_dir.join(".gitignore");
    std::fs::write(&gitignore, GITIGNORE)?;
    
    // Create .env.example
    let env_example = project_dir.join(".env.example");
    std::fs::write(&env_example, ENV_EXAMPLE)?;

    info!("Project {} initialized successfully!", name);
    
    println!("\n✅ Project '{}' created at {:?}", name, project_dir);
    println!("\n📁 Project structure:");
    println!("   {}/", name);
    println!("   ├── src/");
    println!("   │   ├── controllers/");
    println!("   │   ├── services/");
    println!("   │   ├── dtos/");
    println!("   │   ├── modules/");
    println!("   │   ├── entities/");
    println!("   │   ├── routes/");
    println!("   │   ├── app.structa");
    println!("   │   └── main.ts");
    println!("   ├── package.json");
    println!("   ├── tsconfig.json");
    println!("   ├── structa.json");
    println!("   ├── README.md");
    println!("   └── .gitignore");
    println!("\n📦 Next steps:");
    println!("   cd {}", name);
    println!("   structa install");
    println!("   structa dev");
    println!("\n🌐 API will be available at http://localhost:3000");
    println!("   GET  /api/users - Get all users");
    println!("   GET  /api/users/:id - Get user by ID");
    println!("   POST /api/users - Create new user");
    
    Ok(())
}

const USER_CONTROLLER: &str = r#"import { Controller, Get, Post, Put, Delete } from '@structa/runtime';
import { Body, Param } from '@structa/http';
import { UserService } from '../services/user.service';
import { CreateUserDto, UpdateUserDto } from '../dtos/user.dto';

@Controller('/users')
export class UserController {
    constructor(private readonly userService: UserService) {}

    @Get('/')
    async findAll(): Promise<User[]> {
        return this.userService.findAll();
    }

    @Get('/:id')
    async findById(@Param('id') id: string): Promise<User | null> {
        return this.userService.findById(id);
    }

    @Post('/')
    async create(@Body() createUserDto: CreateUserDto): Promise<User> {
        return this.userService.create(createUserDto);
    }

    @Put('/:id')
    async update(
        @Param('id') id: string, 
        @Body() updateUserDto: UpdateUserDto
    ): Promise<User | null> {
        return this.userService.update(id, updateUserDto);
    }

    @Delete('/:id')
    async delete(@Param('id') id: string): Promise<boolean> {
        return this.userService.delete(id);
    }
}
"#;

const USER_SERVICE: &str = r#"import { Injectable } from '@structa/runtime';
import { User, CreateUserDto, UpdateUserDto } from '../dtos/user.dto';

@Injectable()
export class UserService {
    private users: User[] = [
        { id: '1', name: 'John Doe', email: 'john@example.com', createdAt: new Date() },
        { id: '2', name: 'Jane Doe', email: 'jane@example.com', createdAt: new Date() }
    ];

    async findAll(): Promise<User[]> {
        return this.users;
    }

    async findById(id: string): Promise<User | null> {
        return this.users.find(user => user.id === id) || null;
    }

    async create(data: CreateUserDto): Promise<User> {
        const newUser: User = {
            id: String(this.users.length + 1),
            name: data.name,
            email: data.email,
            createdAt: new Date()
        };
        this.users.push(newUser);
        return newUser;
    }

    async update(id: string, data: UpdateUserDto): Promise<User | null> {
        const index = this.users.findIndex(user => user.id === id);
        if (index === -1) return null;
        
        this.users[index] = { ...this.users[index], ...data };
        return this.users[index];
    }

    async delete(id: string): Promise<boolean> {
        const index = this.users.findIndex(user => user.id === id);
        if (index === -1) return false;
        
        this.users.splice(index, 1);
        return true;
    }
}
"#;

const USER_DTO: &str = r#"export interface User {
    id: string;
    name: string;
    email: string;
    createdAt: Date;
    updatedAt?: Date;
}

export interface CreateUserDto {
    name: string;
    email: string;
}

export interface UpdateUserDto {
    name?: string;
    email?: string;
}
"#;

const USER_ENTITY: &str = r#"import 'reflect-metadata';
import { Entity, PrimaryGeneratedColumn, Column, CreateDateColumn, UpdateDateColumn, Index } from '@structa/orm';

@Entity('users')
export class UserEntity {
    @PrimaryGeneratedColumn()
    id!: number;

    @Column({ type: 'varchar', length: 100 })
    @Index()
    name!: string;

    @Column({ type: 'varchar', length: 255, unique: true })
    email!: string;

    @CreateDateColumn()
    createdAt!: Date;

    @UpdateDateColumn()
    updatedAt!: Date;
}
"#;

const APP_MODULE: &str = r#"import 'reflect-metadata';
import { Module } from '@structa/runtime';
import { UserController } from '../controllers/user.controller';
import { UserService } from '../services/user.service';

@Module({
    controllers: [UserController],
    providers: [UserService],
    exports: [UserService]
})
export class AppModule {}
"#;

const TSCONFIG: &str = r#"{
    "compilerOptions": {
        "target": "ES2022",
        "module": "NodeNext",
        "moduleResolution": "NodeNext",
        "lib": ["ES2022"],
        "outDir": "./dist",
        "rootDir": "./src",
        "strict": true,
        "esModuleInterop": true,
        "skipLibCheck": true,
        "forceConsistentCasingInFileNames": true,
        "experimentalDecorators": true,
        "emitDecoratorMetadata": true,
        "declaration": true,
        "declarationMap": true,
        "sourceMap": true,
        "resolveJsonModule": true
    },
    "include": ["src/**/*"],
    "exclude": ["node_modules", "dist", "**/*.spec.ts"]
}"#;

const README: &str = r#"# {name} - Structa API

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
"#;

const GITIGNORE: &str = r#"# Dependencies
node_modules/
.pnp
.pnp.js

# Build
dist/
build/

# Environment
.env
.env.local
.env.*.local

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Logs
logs/
*.log
npm-debug.log*

# Test
coverage/

# Database
*.db
*.sqlite
data/
"#;

const ENV_EXAMPLE: &str = r#"# Server Configuration
PORT=3000
HOST=0.0.0.0

# Database Configuration
DATABASE_TYPE=sqlite
DATABASE_HOST=localhost
DATABASE_PORT=5432
DATABASE_NAME=structa_db
DATABASE_USER=
DATABASE_PASSWORD=

# JWT Configuration
JWT_SECRET=your-secret-key
JWT_EXPIRES_IN=7d

# CORS
CORS_ORIGIN=*
"#;
