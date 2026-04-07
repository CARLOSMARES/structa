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

    // Create user.dto.ts
    let user_dto_file = dtos_dir.join("user.dto.ts");
    let user_dto_content = r#"export interface User {
    id: string;
    name: string;
    email: string;
}

export interface CreateUserDto {
    name: string;
    email: string;
    password: string;
}

export interface UpdateUserDto {
    name: string;
    email: string;
}
"#;
    std::fs::write(&user_dto_file, user_dto_content)?;

    // Create user.repository.ts
    let user_repo_file = repositories_dir.join("user.repository.ts");
    let user_repo_content = r#"import { User } from '../dtos/user.dto';

const users: User[] = [];

export class UserRepository {
    findAll(): User[] {
        return users;
    }

    findById(id: string): User | undefined {
        return users.find(u => u.id === id);
    }

    create(data: Omit<User, 'id'>): User {
        const user = { id: Date.now().toString(), ...data };
        users.push(user);
        return user;
    }

    update(id: string, data: Partial<User>): User | undefined {
        const index = users.findIndex(u => u.id === id);
        if (index === -1) return undefined;
        users[index] = { ...users[index], ...data };
        return users[index];
    }

    delete(id: string): boolean {
        const index = users.findIndex(u => u.id === id);
        if (index === -1) return false;
        users.splice(index, 1);
        return true;
    }
}
"#;
    std::fs::write(&user_repo_file, user_repo_content)?;

    // Create user.service.ts
    let user_service_file = services_dir.join("user.service.ts");
    let user_service_content = r#"import { User, CreateUserDto, UpdateUserDto } from '../dtos/user.dto';
import { UserRepository } from '../repositories/user.repository';

export class UserService {
    private userRepo = new UserRepository();

    findAll(): User[] {
        return this.userRepo.findAll();
    }

    findById(id: string): User | undefined {
        return this.userRepo.findById(id);
    }

    create(data: CreateUserDto): User {
        return this.userRepo.create(data);
    }

    update(id: string, data: UpdateUserDto): User | undefined {
        return this.userRepo.update(id, data);
    }

    delete(id: string): boolean {
        return this.userRepo.delete(id);
    }
}
"#;
    std::fs::write(&user_service_file, user_service_content)?;

    // Create user.controller.ts
    let user_controller_file = controllers_dir.join("user.controller.ts");
    let user_controller_content = r#"import { UserService } from '../services/user.service';
import { CreateUserDto, UpdateUserDto } from '../dtos/user.dto';

const userService = new UserService();

export const userController = {
    async findAll() {
        return userService.findAll();
    },

    async findById(id: string) {
        const user = userService.findById(id);
        if (!user) {
            throw { status: 404, message: 'User not found' };
        }
        return user;
    },

    async create(data: CreateUserDto) {
        return userService.create(data);
    },

    async update(id: string, data: UpdateUserDto) {
        return userService.update(id, data);
    },

    async delete(id: string) {
        const deleted = userService.delete(id);
        if (!deleted) {
            throw { status: 404, message: 'User not found' };
        }
        return { success: true };
    }
};
"#;
    std::fs::write(&user_controller_file, user_controller_content)?;

    // Create main.ts
    let main_file = src_dir.join("main.ts");
    let main_content = r#"import { createServer } from '@structa/http';

const app = createServer({
    port: 3000,
    prefix: '/api'
});

app.get('/', () => ({
    name: 'Structa API',
    version: '1.0.0',
    status: 'running'
}));

app.get('/health', () => ({ status: 'ok' }));

app.listen();
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
        "start": "tsx src/main.ts",
        "build": "structa build"
    }},
    "dependencies": {{
        "@structa/http": "latest"
    }},
    "devDependencies": {{
        "tsx": "^4.7.0",
        "typescript": "^5.3.0",
        "@types/node": "^20.0.0"
    }}
}}"#, name);
    std::fs::write(&package_json, package_content)?;

    // Create tsconfig.json
    let config_file = project_dir.join("tsconfig.json");
    let config_content = r#"{
    "compilerOptions": {
        "target": "ES2022",
        "module": "ES2022",
        "moduleResolution": "bundler",
        "esModuleInterop": true,
        "strict": false,
        "skipLibCheck": true,
        "resolveJsonModule": true,
        "allowSyntheticDefaultImports": true
    },
    "include": ["src/**/*"],
    "exclude": ["node_modules"]
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
│   └── main.ts        # Main entry point
├── package.json
└── tsconfig.json
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
