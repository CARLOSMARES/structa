use anyhow::Result;
use clap::ValueEnum;
use std::path::PathBuf;
use tracing::info;

#[derive(ValueEnum, Clone, Debug)]
pub enum ComponentType {
    Controller,
    Service,
    Module,
    Middleware,
    Guard,
    Resolver,
    Gateway,
    Dto,
    Entity,
    Route,
}

pub const STRUCTA_PACKAGES: &[&str] = &[
    "@structa/runtime",
    "@structa/http",
    "@structa/graphql",
    "@structa/websockets",
    "@structa/swagger",
    "@structa/testing",
    "@structa/orm",
    "@structa/validation",
    "@structa/cache",
    "@structa/queue",
    "@structa/mail",
];

pub fn run(component: ComponentType, name: String, path: Option<PathBuf>) -> Result<()> {
    let target_path = path.unwrap_or_else(|| PathBuf::from("src"));

    let content = match component {
        ComponentType::Controller => generate_controller(&name),
        ComponentType::Service => generate_service(&name),
        ComponentType::Module => generate_module(&name),
        ComponentType::Middleware => generate_middleware(&name),
        ComponentType::Guard => generate_guard(&name),
        ComponentType::Resolver => generate_resolver(&name),
        ComponentType::Gateway => generate_gateway(&name),
        ComponentType::Dto => generate_dto(&name),
        ComponentType::Entity => generate_entity(&name),
        ComponentType::Route => generate_route(&name),
    };

    let file_name = match component {
        ComponentType::Controller => format!("{}.structa", to_snake_case(&name)),
        ComponentType::Service => format!("{}.service.structa", to_snake_case(&name)),
        ComponentType::Module => format!("{}.module.structa", to_snake_case(&name)),
        ComponentType::Middleware => format!("{}.middleware.structa", to_snake_case(&name)),
        ComponentType::Guard => format!("{}.guard.structa", to_snake_case(&name)),
        ComponentType::Resolver => format!("{}.resolver.structa", to_snake_case(&name)),
        ComponentType::Gateway => format!("{}.gateway.structa", to_snake_case(&name)),
        ComponentType::Dto => format!("{}.dto.structa", to_snake_case(&name)),
        ComponentType::Entity => format!("{}.entity.structa", to_snake_case(&name)),
        ComponentType::Route => format!("{}.route.structa", to_snake_case(&name)),
    };

    let file_path = target_path.join(&file_name);
    std::fs::write(&file_path, content)?;

    info!("Created {} at {:?}", file_name, file_path);
    println!("\n✅ Created {}: {}", format!("{:?}", component), name);
    println!("   File: {:?}", file_path);

    Ok(())
}

pub fn add_package(package_name: &str) -> Result<()> {
    println!("\n📦 Adding package: {}", package_name);

    if !STRUCTA_PACKAGES.contains(&package_name) {
        eprintln!(
            "⚠️  Warning: {} is not a recognized @structa package",
            package_name
        );
        eprintln!("   Available packages:");
        for pkg in STRUCTA_PACKAGES {
            println!("   - {}", pkg);
        }
    }

    let result = std::process::Command::new("npm")
        .args(&["install", package_name])
        .status();

    match result {
        Ok(status) if status.success() => {
            println!("\n✅ Package {} added successfully!", package_name);
        }
        Ok(_) => {
            eprintln!("\n❌ Failed to add package: npm install returned an error");
        }
        Err(e) => {
            eprintln!("\n❌ Failed to run npm: {}", e);
        }
    }

    Ok(())
}

fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

fn generate_controller(name: &str) -> String {
    format!(
        r#"controller {} {{
    path: "/{}"
    
    @Get("/")
    async getAll(): {}[] {{
        return []
    }}
    
    @Get("/:id")
    async getById(id: string): {} {{
        return {{ id }}
    }}
}}
"#,
        name,
        to_snake_case(name),
        name,
        name
    )
}

fn generate_service(name: &str) -> String {
    format!(
        r#"service {}Service {{
    async findAll(): {}[] {{
        return []
    }}
    
    async findById(id: string): {} | null {{
        return null
    }}
    
    async create(data: Partial<{}>): {} {{
        return {{ id: "1", ...data }}
    }}
}}
"#,
        name, name, name, name, name
    )
}

fn generate_module(name: &str) -> String {
    format!(
        r#"module {}Module {{
    controllers: [{}Controller]
    services: [{}Service]
    exports: [{}Service]
}}
"#,
        name, name, name, name
    )
}

fn generate_middleware(name: &str) -> String {
    format!(
        r#"middleware {}Middleware {{
    async use(req: Request, res: Response, next: NextFunction): Promise<void> {{
        // TODO: Implement middleware logic
        next()
    }}
}}
"#,
        name
    )
}

fn generate_guard(name: &str) -> String {
    format!(
        r#"guard {}Guard {{
    canActivate(context: ExecutionContext): boolean {{
        // TODO: Implement guard logic
        return true
    }}
}}
"#,
        name
    )
}

fn generate_resolver(name: &str) -> String {
    format!(
        r#"resolver {}Resolver {{
    @Query()
    async {}s(): {}[] {{
        return []
    }}
    
    @Query(":id")
    async {}: {} {{
        return null
    }}
    
    @Mutation()
    async create{}(): {} {{
        return null
    }}
}}
"#,
        name, name, name, name, name, name, name
    )
}

fn generate_gateway(name: &str) -> String {
    format!(
        r#"gateway {}Gateway {{
    namespace: "/{}"
    
    @SubscribeMessage("message")
    async handleMessage(client: any, payload: any): Promise<any> {{
        return {{ event: "message", data: payload }}
    }}
    
    afterInit(server: any): void {{
        // TODO: Initialize gateway
    }}
}}
"#,
        name,
        to_snake_case(name)
    )
}

fn generate_dto(name: &str) -> String {
    format!(
        r#"dto {} {{
    id: int pk auto
    name: string
    email?: string
    createdAt: datetime
    updatedAt: datetime
}}
"#,
        name
    )
}

fn generate_entity(name: &str) -> String {
    format!(
        r#"entity {} {{
    table: {}
    
    id: int pk auto
    name: string length(100)
    email?: string
    createdAt: datetime
    updatedAt: datetime
    
    index: [name, email] unique
}}
"#,
        name,
        to_snake_case(name)
    )
}

fn generate_route(name: &str) -> String {
    format!(
        r#"route {} {{
    path: "/{}"
    
    GET "/" list summary "Get all {}s"
    GET "/:id" getById summary "Get {} by ID"
    POST "/" create summary "Create new {}"
    PUT "/:id" update summary "Update {}"
    DELETE "/:id" remove summary "Delete {}"
}}
"#,
        name,
        to_snake_case(name),
        name,
        name,
        name,
        name,
        name
    )
}
