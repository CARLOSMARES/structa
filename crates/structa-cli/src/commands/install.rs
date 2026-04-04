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

pub fn run(project_path: Option<PathBuf>) -> Result<()> {
    let path = project_path.unwrap_or_else(|| PathBuf::from("."));

    if !path.join("package.json").exists() {
        eprintln!("❌ No package.json found in current directory");
        eprintln!("   Are you in a Structa project directory?");
        return Ok(());
    }

    println!("\n📦 Installing dependencies from package.json...\n");

    let (npm_bin, mut npm_base_args) = get_npm_cmd();

    // Check if npm is available
    let mut npm_check_args = npm_base_args.clone();
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

    // Run npm install
    println!("🔄 Running: npm install\n");

    let mut install_args = npm_base_args.clone();
    install_args.push("install");

    let install_output = Command::new(npm_bin)
        .args(&install_args)
        .current_dir(&path)
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
    println!("\n📦 Installing: {}\n", package_name);

    let (npm_bin, mut npm_args) = get_npm_cmd();
    npm_args.push("install");
    npm_args.push(package_name);

    let output = Command::new(npm_bin).args(&npm_args).output()?;

    if output.status.success() {
        println!("✅ {} installed successfully!", package_name);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ Failed to install {}:", package_name);
        eprintln!("{}", stderr);
    }

    Ok(())
}
