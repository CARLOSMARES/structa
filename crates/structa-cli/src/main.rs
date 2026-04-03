use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod commands;
mod config;

use commands::{init, build, dev, generate, add, docs};

#[derive(Parser)]
#[command(name = "structa")]
#[command(version = "0.1.0")]
#[command(about = "Structa Framework CLI - TypeScript API framework compiler", long_about = None)]
struct Cli {
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
    
    #[command(about = "Generate OpenAPI/Swagger documentation")]
    Docs {
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,
        
        #[arg(long)]
        format: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .init();

    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init { path, name, template } => {
            init::run(path, name, template).await?;
        }
        Commands::Build { source, output, watch, config } => {
            build::run(source, output, watch, config).await?;
        }
        Commands::Dev { source, port, hot_reload } => {
            dev::run(source, port, hot_reload).await?;
        }
        Commands::Generate { schema, output, watch } => {
            generate::run(schema, output, watch).await?;
        }
        Commands::Add { component, name, path } => {
            add::run(component, name, path)?;
        }
        Commands::Docs { output, format } => {
            docs::run(output, format)?;
        }
    }
    
    Ok(())
}
