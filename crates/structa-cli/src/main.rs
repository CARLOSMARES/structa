use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod commands;
mod config;

use commands::{init, build, dev, generate, add, docs, install, update, uninstall, orm};

#[derive(Parser)]
#[command(name = "structa")]
#[command(version = "0.6.1")]
#[command(about = "Structa Framework CLI - TypeScript API framework powered by Rust compiler", long_about = None)]
struct Cli {
    #[arg(short, long, global = true, help = "Enable verbose logging")]
    verbose: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize a new Structa project")]
    Init {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        
        #[arg(short, long, default_value = "my-api")]
        name: String,
        
        #[arg(short, long, default_value = "latest")]
        template: String,
    },
    
    #[command(about = "Build the Structa project")]
    Build {
        #[arg(short, long, default_value = "src")]
        source: PathBuf,
        
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,
        
        #[arg(long)]
        watch: bool,
        
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    #[command(about = "Run development server")]
    Dev {
        #[arg(short, long, default_value = "src")]
        source: PathBuf,
        
        #[arg(short, long, default_value = "3000")]
        port: u16,
        
        #[arg(long)]
        hot_reload: bool,
        
        #[arg(long, help = "Run TypeScript directly without compiling (requires tsx or ts-node)")]
        no_compile: bool,
    },
    
    #[command(about = "Generate code from schema")]
    Generate {
        #[arg(short, long)]
        schema: Option<PathBuf>,
        
        #[arg(short, long, default_value = "src/generated")]
        output: PathBuf,
        
        #[arg(long)]
        watch: bool,
    },
    
    #[command(about = "Add a new component to the project")]
    Add {
        #[arg(value_enum)]
        component: commands::add::ComponentType,
        
        #[arg(short, long)]
        name: String,
        
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    
    #[command(about = "Add a @structa package (e.g., structa add @structa/testing)")]
    AddPackage {
        #[arg(value_name = "PACKAGE")]
        package_name: String,
    },
    
    #[command(about = "Install project dependencies")]
    Install {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        
        #[arg(value_name = "PACKAGE")]
        package: Option<String>,
    },
    
    #[command(about = "Update @structa packages to the latest version")]
    Update {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        
        #[arg(value_name = "PACKAGE")]
        package: Option<String>,
    },
    
    #[command(about = "Uninstall a package")]
    Uninstall {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        
        #[arg(value_name = "PACKAGE")]
        package: String,
    },
    
    #[command(about = "Downgrade a package to a specific version")]
    Downgrade {
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        
        #[arg(value_name = "PACKAGE")]
        package: String,
        
        #[arg(value_name = "VERSION")]
        version: String,
    },
    
    #[command(about = "Generate OpenAPI/Swagger documentation")]
    Docs {
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,
        
        #[arg(long)]
        format: Option<String>,
    },
    
    #[command(about = "ORM database commands (requires @structa/orm)")]
    Orm {
        #[command(subcommand)]
        subcommand: commands::orm::OrmSubcommand,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let log_level = if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive(log_level.into()))
        .init();
    
    match cli.command {
        Commands::Init { path, name, template } => {
            init::run(path, name, template).await?;
        }
        Commands::Build { source, output, watch, config } => {
            build::run(source, output, watch, config).await?;
        }
        Commands::Dev { source, port, hot_reload, no_compile } => {
            dev::run(source, port, hot_reload, no_compile).await?;
        }
        Commands::Generate { schema, output, watch } => {
            generate::run(schema, output, watch).await?;
        }
        Commands::Add { component, name, path } => {
            add::run(component, name, path)?;
        }
        Commands::AddPackage { package_name } => {
            add::add_package(&package_name)?;
        }
        Commands::Install { path, package } => {
            install::run(Some(path), package.as_deref())?;
        }
        Commands::Update { path, package } => {
            update::run(Some(path), package.as_deref())?;
        }
        Commands::Uninstall { path, package } => {
            uninstall::run(Some(path), &package)?;
        }
        Commands::Downgrade { path, package, version } => {
            update::downgrade(Some(path), &package, &version)?;
        }
        Commands::Docs { output, format } => {
            docs::run(output, format)?;
        }
        Commands::Orm { subcommand } => {
            orm::run(subcommand)?;
        }
    }
    
    Ok(())
}
