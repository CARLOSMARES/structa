use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "windows")]
fn get_npm_cmd() -> (&'static str, Vec<&'static str>) {
    ("cmd", vec!["/c", "npm"])
}

#[cfg(not(target_os = "windows"))]
fn get_npm_cmd() -> (&'static str, Vec<&'static str>) {
    ("npm", vec![])
}

pub fn run(project_path: Option<PathBuf>, package: Option<&str>) -> Result<()> {
    let path = project_path.unwrap_or_else(|| PathBuf::from("."));

    if !path.join("package.json").exists() {
        eprintln!("❌ No package.json found in current directory");
        eprintln!("   Are you in a Structa project directory?");
        return Ok(());
    }

    let (npm_bin, npm_args) = get_npm_cmd();

    if let Some(pkg) = package {
        install_specific_package(&path, pkg, npm_bin, &npm_args)?;
    } else {
        install_from_package_json(&path, npm_bin, &npm_args)?;
    }

    Ok(())
}

fn install_specific_package(
    path: &PathBuf,
    package: &str,
    npm_bin: &str,
    npm_base_args: &[&str],
) -> Result<()> {
    println!("\n📦 Installing: {}\n", package);

    let mut install_args: Vec<&str> = npm_base_args.to_vec();
    install_args.push("install");

    if package.starts_with('@') {
        install_args.push(package);
    } else if package.contains('/') {
        install_args.push(package);
    } else {
        install_args.push(package);
    }

    let output = Command::new(npm_bin)
        .args(&install_args)
        .current_dir(path)
        .output()?;

    if output.status.success() {
        println!("✅ {} installed successfully!\n", package);
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            println!("{}", stdout);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ Failed to install {}:", package);
        eprintln!("{}", stderr);
    }

    Ok(())
}

fn install_from_package_json(path: &PathBuf, npm_bin: &str, npm_base_args: &[&str]) -> Result<()> {
    println!("\n📦 Installing dependencies from package.json...\n");

    let mut npm_check_args = npm_base_args.to_vec();
    npm_check_args.push("--version");

    let npm_check = Command::new(npm_bin).args(&npm_check_args).output();

    match npm_check {
        Ok(output) if output.status.success() => {
            let npm_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("   npm version: {}", npm_version);
        }
        _ => {
            eprintln!("❌ npm not found. Please install Node.js from https://nodejs.org");
            return Ok(());
        }
    }

    println!("🔄 Running: npm install\n");

    let mut install_args = npm_base_args.to_vec();
    install_args.push("install");

    let install_output = Command::new(npm_bin)
        .args(&install_args)
        .current_dir(path)
        .output()?;

    if install_output.status.success() {
        println!("\n✅ Dependencies installed successfully!\n");

        let stdout = String::from_utf8_lossy(&install_output.stdout);
        if !stdout.is_empty() {
            println!("{}", stdout);
        }
    } else {
        let stderr = String::from_utf8_lossy(&install_output.stderr);
        eprintln!("\n❌ npm install failed:");
        eprintln!("{}", stderr);
    }

    println!("\n✨ Setup complete!\n");
    println!("📁 Next steps:");
    println!("   structa dev          # Start development server");
    println!("   structa build       # Build for production");
    println!();

    Ok(())
}

pub fn install_package(package_name: &str) -> Result<()> {
    let path = PathBuf::from(".");
    let (npm_bin, npm_args) = get_npm_cmd();
    install_specific_package(&path, package_name, npm_bin, &npm_args)
}
