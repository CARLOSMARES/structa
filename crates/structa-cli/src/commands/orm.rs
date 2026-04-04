use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub fn run(subcommand: OrmSubcommand) -> Result<()> {
    match subcommand {
        OrmSubcommand::Migrate {
            connection,
            rollback,
            create,
        } => {
            run_migrate(connection, rollback, create)?;
        }
        OrmSubcommand::Db { subcommand } => {
            run_db(subcommand)?;
        }
    }
    Ok(())
}

fn run_migrate(connection: Option<String>, rollback: bool, create: Option<String>) -> Result<()> {
    let project_root = std::env::current_dir()?;

    check_orm_installed(&project_root)?;

    if let Some(name) = create {
        println!("📝 Creating migration: {}", name);
        run_npx(
            &[
                "tsx",
                "node_modules/@structa/orm/dist/cli.js",
                "migration:create",
                &name,
            ],
            &project_root,
        )?;
        println!("✅ Migration created: {}", name);
        return Ok(());
    }

    if rollback {
        println!("⏪ Rolling back last migration...");
        let mut args = vec![
            "tsx",
            "node_modules/@structa/orm/dist/cli.js",
            "migrate:revert",
        ];
        if let Some(ref conn) = connection {
            args.push("--connection");
            args.push(conn);
        }
        run_npx(&args, &project_root)?;
        println!("✅ Migration rolled back");
        return Ok(());
    }

    println!("🔄 Running pending migrations...");
    let mut args = vec!["tsx", "node_modules/@structa/orm/dist/cli.js", "migrate"];
    if let Some(ref conn) = connection {
        args.push("--connection");
        args.push(conn);
    }
    run_npx(&args, &project_root)?;
    println!("✅ Migrations completed");
    Ok(())
}

fn run_db(subcommand: DbSubcommand) -> Result<()> {
    let project_root = std::env::current_dir()?;

    check_orm_installed(&project_root)?;

    match subcommand {
        DbSubcommand::Update { connection, force } => {
            println!("🔄 Synchronizing database schema...");
            let mut args = vec![
                "tsx",
                "node_modules/@structa/orm/dist/cli.js",
                "schema:update",
            ];
            if force {
                args.push("--force");
            }
            if let Some(ref conn) = connection {
                args.push("--connection");
                args.push(conn);
            }
            run_npx(&args, &project_root)?;
            println!("✅ Database schema updated");
        }
        DbSubcommand::Drop { connection, force } => {
            if !force {
                println!("⚠️  This will delete ALL data in your database!");
                println!("   Use --force to confirm.");
                return Ok(());
            }
            println!("🗑️  Dropping all tables...");
            let mut args = vec![
                "tsx",
                "node_modules/@structa/orm/dist/cli.js",
                "schema:drop",
            ];
            if let Some(ref conn) = connection {
                args.push("--connection");
                args.push(conn);
            }
            run_npx(&args, &project_root)?;
            println!("✅ All tables dropped");
        }
        DbSubcommand::Sql { output } => {
            println!("📄 Generating SQL schema...");
            let mut args = vec!["tsx", "node_modules/@structa/orm/dist/cli.js", "schema:sql"];
            if let Some(ref out) = output {
                args.push("--output");
                args.push(out);
                run_npx(&args, &project_root)?;
                println!("✅ SQL written to: {}", out);
            } else {
                run_npx(&args, &project_root)?;
            }
        }
    }
    Ok(())
}

fn check_orm_installed(project_root: &PathBuf) -> Result<()> {
    let orm_path = project_root.join("node_modules/@structa/orm");
    if !orm_path.exists() {
        println!("❌ @structa/orm is not installed.");
        println!("   Run: structa add-package orm");
        anyhow::bail!("@structa/orm not found");
    }

    let cli_path = orm_path.join("dist/cli.js");
    if !cli_path.exists() {
        println!("⚠️  ORM CLI not found. Building @structa/orm...");
        run_npm(&["run", "build"], &orm_path)?;
    }

    Ok(())
}

fn run_npm(args: &[&str], path: &PathBuf) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = std::process::Command::new("cmd");
        cmd.args(["/c", "npm"]);
        for arg in args {
            cmd.arg(arg);
        }
        cmd.current_dir(path);
        cmd.stdout(std::process::Stdio::inherit());
        cmd.stderr(std::process::Stdio::inherit());
        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!("npm command failed");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = std::process::Command::new("npm");
        for arg in args {
            cmd.arg(arg);
        }
        cmd.current_dir(path);
        cmd.stdout(std::process::Stdio::inherit());
        cmd.stderr(std::process::Stdio::inherit());
        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!("npm command failed");
        }
    }

    Ok(())
}

fn run_npx(args: &[&str], path: &PathBuf) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = std::process::Command::new("cmd");
        let npx_args: Vec<&str> = ["npx"].iter().chain(args.iter()).copied().collect();
        cmd.args(["/c", "npx"]);
        for arg in args {
            cmd.arg(arg);
        }
        cmd.current_dir(path);
        cmd.stdout(std::process::Stdio::inherit());
        cmd.stderr(std::process::Stdio::inherit());
        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!("npx command failed");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = std::process::Command::new("npx");
        for arg in args {
            cmd.arg(arg);
        }
        cmd.current_dir(path);
        cmd.stdout(std::process::Stdio::inherit());
        cmd.stderr(std::process::Stdio::inherit());
        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!("npx command failed");
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Subcommand)]
pub enum OrmSubcommand {
    #[command(about = "Run pending database migrations")]
    Migrate {
        #[arg(short, long, help = "Connection string or config name")]
        connection: Option<String>,

        #[arg(long, help = "Revert last migration")]
        rollback: bool,

        #[arg(long, help = "Create a new migration file")]
        create: Option<String>,
    },

    #[command(about = "Synchronize database schema with entities")]
    Db {
        #[command(subcommand)]
        subcommand: DbSubcommand,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum DbSubcommand {
    #[command(about = "Update database schema (sync entities)")]
    Update {
        #[arg(short, long, help = "Connection string or config name")]
        connection: Option<String>,

        #[arg(long, help = "Drop tables before creating")]
        force: bool,
    },

    #[command(about = "Drop all tables")]
    Drop {
        #[arg(short, long, help = "Connection string or config name")]
        connection: Option<String>,

        #[arg(long, help = "Skip confirmation prompt")]
        force: bool,
    },

    #[command(about = "Show database schema as SQL")]
    Sql {
        #[arg(short, long, help = "Output SQL to file")]
        output: Option<String>,
    },
}
