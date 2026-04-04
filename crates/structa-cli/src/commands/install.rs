use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn run(project_path: Option<PathBuf>) -> Result<()> {
    let path = project_path.unwrap_or_else(|| PathBuf::from("."));

    if !path.join("package.json").exists() {
        eprintln!("❌ No package.json found in current directory");
        eprintln!("   Are you in a Structa project directory?");
        return Ok(());
    }

    println!("\n📦 Installing dependencies from package.json...\n");

    // Check if npm is available
    let npm_check = Command::new("npm").arg("--version").output();

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

    let install_output = Command::new("npm")
        .args(["install"])
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

    let output = Command::new("npm")
        .args(["install", package_name])
        .output()?;

    if output.status.success() {
        println!("✅ {} installed successfully!", package_name);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ Failed to install {}:", package_name);
        eprintln!("{}", stderr);
    }

    Ok(())
}
