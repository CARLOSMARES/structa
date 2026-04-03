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
}

pub fn run(component: ComponentType, name: String, path: Option<PathBuf>) -> Result<()> {
    let target_path = path.unwrap_or_else(|| PathBuf::from("src"));
    let file_name = format!("{}.structa", to_snake_case(&name));

    let content = match component {
        ComponentType::Controller => generate_controller(&name),
        ComponentType::Service => generate_service(&name),
        ComponentType::Module => generate_module(&name),
        ComponentType::Middleware => generate_middleware(&name),
        ComponentType::Guard => generate_guard(&name),
        ComponentType::Resolver => generate_resolver(&name),
        ComponentType::Gateway => generate_gateway(&name),
    };

    let file_path = target_path.join(&file_name);
    std::fs::write(&file_path, content)?;

    info!("Created {} at {:?}", file_name, file_path);
    println!(
        "\n✅ Created {} component: {}",
        format!("{:?}", component),
        name
    );
    println!("   File: {:?}", file_path);

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
