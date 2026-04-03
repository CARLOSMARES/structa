use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub async fn run(path: PathBuf, name: String, template: String) -> Result<()> {
    info!("Initializing Structa project: {} at {:?}", name, path);
    
    let project_dir = path.join(&name);
    
    std::fs::create_dir_all(&project_dir)?;
    
    let src_dir = project_dir.join("src");
    std::fs::create_dir_all(&src_dir)?;
    
    let app_file = src_dir.join("app.structa");
    std::fs::write(&app_file, generate_app_template(&name))?;
    
    let structa_config = project_dir.join("structa.json");
    std::fs::write(&structa_config, generate_config(&name, &template))?;
    
    let package_json = project_dir.join("package.json");
    std::fs::write(&package_json, generate_package_json(&name))?;
    
    let tsconfig = project_dir.join("tsconfig.json");
    std::fs::write(&tsconfig, generate_tsconfig())?;
    
    info!("Project {} initialized successfully!", name);
    println!("\n✅ Project '{}' created at {:?}", name, project_dir);
    println!("\nNext steps:");
    println!("  cd {}", name);
    println!("  structa dev");
    
    Ok(())
}

fn generate_app_template(name: &str) -> String {
    format!(r#"module {} {{
    imports [
        "@structa/http",
        "@structa/validation"
    ]
    
    controller UserController {{
        path: "/users"
        
        @Get("/")
        async getUsers(): User[] {{
            return this.userService.findAll()
        }}
        
        @Get("/:id")
        async getUser(id: string): User {{
            return this.userService.findById(id)
        }}
        
        @Post("/")
        @Body() body: CreateUserDto
        async createUser(): User {{
            return this.userService.create(body)
        }}
    }}
    
    service UserService {{
        async findAll(): User[] {{
            return []
        }}
        
        async findById(id: string): User {{
            return {{ id, name: "User" }}
        }}
        
        async create(data: CreateUserDto): User {{
            return {{ id: "1", ...data }}
        }}
    }}
}}

dto User {{
    id: string
    name: string
    email?: string
}}

dto CreateUserDto {{
    name: string
    email: string
}}
"#, name)
}

fn generate_config(name: &str, template: &str) -> String {
    format!(r#"{{
    "name": "{}",
    "template": "{}",
    "compiler": {{
        "target": "es2022",
        "module": "commonjs",
        "strict": true,
        "experimentalDecorators": true,
        "emitDecoratorMetadata": true,
        "outDir": "./dist",
        "rootDir": "./src",
        "declaration": true,
        "sourceMap": true
    }},
    "server": {{
        "port": 3000,
        "host": "localhost",
        "cors": true
    }}
}}"#, name, template)
}

fn generate_package_json(name: &str) -> String {
    format!(r#"{{
    "name": "{}",
    "version": "0.1.0",
    "description": "Structa API project",
    "main": "dist/index.js",
    "scripts": {{
        "dev": "structa dev",
        "build": "structa build",
        "start": "node dist/index.js"
    }},
    "dependencies": {{
        "@structa/runtime": "^0.1.0",
        "@structa/http": "^0.1.0"
    }},
    "devDependencies": {{
        "typescript": "^5.0.0",
        "@types/node": "^20.0.0"
    }}
}}"#, name)
}

fn generate_tsconfig() -> String {
    r#"{
    "compilerOptions": {
        "target": "ES2022",
        "module": "commonjs",
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
        "sourceMap": true
    },
    "include": ["src/**/*"],
    "exclude": ["node_modules", "dist"]
}"#.to_string()
}
