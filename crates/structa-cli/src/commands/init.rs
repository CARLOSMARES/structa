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

    // Create user.dto.structa
    let user_dto_file = dtos_dir.join("user.dto.structa");
    let user_dto_content = r#"dto User {
    id: string;
    name: string;
    email: string;
}

dto CreateUserDto {
    name: string;
    email: string;
}
"#;
    std::fs::write(&user_dto_file, user_dto_content)?;

    // Create user.service.structa
    let user_service_file = services_dir.join("user.service.structa");
    let user_service_content = r#"service UserService {
    findAll() { return [{id:"1",name:"John Doe",email:"john@example.com"},{id:"2",name:"Jane Doe",email:"jane@example.com"}]; }
    findById(id) { return [{id:"1",name:"John Doe"},{id:"2",name:"Jane Doe"}].find(u=>u.id===id)||null; }
    create(data) { return {id:"3",...data}; }
}
"#;
    std::fs::write(&user_service_file, user_service_content)?;

    // Create user.controller.structa
    let user_controller_file = controllers_dir.join("user.controller.structa");
    let user_controller_content = r#"controller UserController "/users" {
    Get "/"
    findAll() { return this.userService.findAll(); }

    Get "/:id"
    findById(id: string) { return this.userService.findById(id); }

    Post "/"
    create(data) { return this.userService.create(data); }
}
"#;
    std::fs::write(&user_controller_file, user_controller_content)?;

    // Create main.structa entry point
    let main_file = src_dir.join("main.structa");
    let main_content = r#"import { createHttpServer } from '@structa/runtime';
import './controllers/user.controller';
import './services/user.service';

const port = parseInt(process.env.PORT || '3000');
const host = process.env.HOST || '0.0.0.0';

const server = createHttpServer();
server.listen(port, host).then(() => {
    console.log(`🚀 Structa is running on http://${host}:${port}`);
});
"#;
    std::fs::write(&main_file, main_content)?;
    
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
    "type": "module",
    "description": "Structa API project - TypeScript framework powered by Rust",
    "main": "dist/src/main.js",
    "scripts": {{
        "dev": "structa dev",
        "build": "structa build",
        "start": "node dist/src/main.js",
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
    println!("   │   └── main.structa");
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
├── controllers/    # Route handlers (user.controller.structa)
├── services/       # Business logic (user.service.structa)
├── dtos/           # Data transfer objects (user.dto.structa)
├── modules/        # App modules
├── entities/       # Database entities
├── routes/         # Route definitions
└── main.structa    # Entry point
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
