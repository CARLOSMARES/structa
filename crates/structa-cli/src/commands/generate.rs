use anyhow::Result;
use clap::ValueEnum;
use std::path::PathBuf;
use tracing::info;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum GenType {
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

pub fn run(type_: GenType, name: &str, path: PathBuf) -> Result<()> {
    let content = match type_ {
        GenType::Controller => generate_controller(name),
        GenType::Service => generate_service(name),
        GenType::Module => generate_module(name),
        GenType::Middleware => generate_middleware(name),
        GenType::Guard => generate_guard(name),
        GenType::Resolver => generate_resolver(name),
        GenType::Gateway => generate_gateway(name),
        GenType::Dto => generate_dto(name),
        GenType::Entity => generate_entity(name),
        GenType::Route => generate_route(name),
    };

    let file_name = match type_ {
        GenType::Controller => format!("{}.structa", to_snake_case(name)),
        GenType::Service => format!("{}.service.structa", to_snake_case(name)),
        GenType::Module => format!("{}.module.structa", to_snake_case(name)),
        GenType::Middleware => format!("{}.middleware.structa", to_snake_case(name)),
        GenType::Guard => format!("{}.guard.structa", to_snake_case(name)),
        GenType::Resolver => format!("{}.resolver.structa", to_snake_case(name)),
        GenType::Gateway => format!("{}.gateway.structa", to_snake_case(name)),
        GenType::Dto => format!("{}.dto.structa", to_snake_case(name)),
        GenType::Entity => format!("{}.entity.structa", to_snake_case(name)),
        GenType::Route => format!("{}.route.structa", to_snake_case(name)),
    };

    let target_path = path.join(&file_name);
    if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&target_path, content)?;

    info!("Created {} at {:?}", file_name, target_path);
    println!("\n✅ Created {}: {}", format!("{:?}", type_), name);
    println!("   File: {:?}", target_path);

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
    
    @Post("/")
    async create(data: any): {} {{
        return {{ id: "1", ...data }}
    }}
    
    @Put("/:id")
    async update(id: string, data: any): {} {{
        return {{ id, ...data }}
    }}
    
    @Delete("/:id")
    async delete(id: string): void {{
    }}
}}
"#,
        name,
        to_snake_case(name),
        name,
        name,
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
    
    async update(id: string, data: Partial<{}>): {} {{
        return {{ id, ...data }}
    }}
    
    async delete(id: string): void {{
    }}
}}
"#,
        name, name, name, name, name, name, name
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
