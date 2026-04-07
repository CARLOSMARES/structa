use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use chrono::Local;

mod commands;
mod config;

use commands::{init, build, dev, generate, install, uninstall, orm};

fn matrix_banner() {
    println!();
    println!("\x1b[32m+====================================================================+\x1b[0m");
    println!("\x1b[32m|  ____  _ _       ____                           _            |\x1b[0m");
    println!("\x1b[32m| | __ )| (_) ___ |  _ \\ ___ _ __   ___  _ __ | |_ ___ _ __ |\x1b[0m");
    println!("\x1b[32m| |  _ \\| | |/ _ \\| |_)/ _ \\ '_ \\ / _ \\| '_ \\| __/ _ \\ '__||\x1b[0m");
    println!("\x1b[32m| | |_) | | | (_) |  _/  __/ |_) | (_) | |_) | ||  __/ |   |\x1b[0m");
    println!("\x1b[32m| |____/|_|_|\\___/|_|  \\___| .__/ \\___/| .__/ \\__\\___|_|   |\x1b[0m");
    println!("\x1b[32m|                            |_|       |_|                     |\x1b[0m");
    println!("\x1b[32m+====================================================================+\x1b[0m");
    println!("\x1b[32m|  Framework v{} - TypeScript-like API Framework in Rust       |\x1b[0m", env!("CARGO_PKG_VERSION"));
    println!("\x1b[32m+====================================================================+\x1b[0m");
    println!();
}

fn log_info(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("\x1b[32m[{}]\x1b[0m \x1b[36mINFO\x1b[0m     \x1b[32m→\x1b[0m {}", timestamp, msg);
}

fn log_warn(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("\x1b[32m[{}]\x1b[0m \x1b[33mWARN\x1b[0m     \x1b[32m→\x1b[0m \x1b[33m{}\x1b[0m", timestamp, msg);
}

fn log_error(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("\x1b[32m[{}]\x1b[0m \x1b[31mERROR\x1b[0m    \x1b[32m→\x1b[0m \x1b[31m{}\x1b[0m", timestamp, msg);
}

fn log_success(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("\x1b[32m[{}]\x1b[0m \x1b[32mOK\x1b[0m      \x1b[32m→\x1b[0m \x1b[32m{}\x1b[0m", timestamp, msg);
}

#[derive(Parser)]
#[command(name = "structa")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Structa Framework - TypeScript-like API framework powered by Rust", long_about = None)]
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
        
        #[arg(short, long, default_value = "api")]
        template: String,
    },
    
    #[command(about = "Run development server with hot reload")]
    Dev {
        #[arg(short, long, default_value = "3000")]
        port: u16,
        
        #[arg(long, help = "Enable hot reload")]
        hot_reload: bool,
    },
    
    #[command(about = "Build the project to JavaScript")]
    Build {
        #[arg(long, help = "Build in release mode")]
        release: bool,
        
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,
    },
    
    #[command(about = "Install dependencies from package.json")]
    Install {
        #[arg(short, long, help = "Install a specific package and add to dependencies")]
        package: Option<String>,
    },
    
    #[command(about = "Add and install a new package from npm")]
    Add {
        #[arg(required = true, help = "Package name to add (e.g., lodash, @structa/orm)")]
        package: String,
        
        #[arg(long, help = "Install as dev dependency")]
        dev: bool,
        
        #[arg(long, help = "Install globally")]
        global: bool,
    },
    
    #[command(about = "Remove a package from the project")]
    Remove {
        #[arg(required = true, help = "Package name to remove")]
        package: String,
    },
    
    #[command(about = "Generate code (controller, service, dto, etc.)")]
    Generate {
        #[arg(required = true, value_enum)]
        type_: commands::generate::GenType,
        
        #[arg(required = true, help = "Name of the component to generate")]
        name: String,
        
        #[arg(short, long, default_value = "src")]
        path: PathBuf,
    },
    
    #[command(about = "ORM database commands")]
    Orm {
        #[command(subcommand)]
        subcommand: commands::orm::OrmSubcommand,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let is_help = std::env::args().any(|arg| arg == "--help" || arg == "-h");
    if !is_help {
        matrix_banner();
    }
    
    match cli.command {
        Commands::Init { path, name, template } => {
            log_info(&format!("Initializing new project: {}", name));
            init::run(path, name, template).await?;
            log_success("Project initialized successfully");
        }
        
        Commands::Dev { port, hot_reload } => {
            log_info(&format!("Starting development server on port {}", port));
            dev::run(port, hot_reload).await?;
        }
        
        Commands::Build { release, output } => {
            log_info(&format!("Building project ({} mode)", if release { "release" } else { "debug" }));
            build::run(release, output).await?;
            log_success("Build completed");
        }
        
        Commands::Install { package } => {
            if let Some(pkg) = package {
                log_info(&format!("Installing: {}", pkg));
                install::add_and_install(&pkg, false)?;
            } else {
                log_info("Installing dependencies from package.json");
                install::install_all()?;
            }
            log_success("Dependencies installed");
        }
        
        Commands::Add { package, dev, global } => {
            log_info(&format!("Adding package: {}", package));
            install::add_and_install(&package, dev || global)?;
            log_success(&format!("Package {} added successfully", package));
        }
        
        Commands::Remove { package } => {
            log_info(&format!("Removing package: {}", package));
            uninstall::run(&package)?;
            log_success(&format!("Package {} removed successfully", package));
        }
        
        Commands::Generate { type_, name, path } => {
            let type_name = format!("{:?}", type_);
            log_info(&format!("Generating {}: {}", type_name, name));
            generate::run(type_, &name, path)?;
            log_success(&format!("{} {} created successfully", type_name, name));
        }
        
        Commands::Orm { subcommand } => {
            log_info("Running ORM commands...");
            orm::run(subcommand)?;
        }
    }
    
    Ok(())
}
