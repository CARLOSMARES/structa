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

pub fn install_all() -> Result<()> {
    let project_root = std::env::current_dir()?;

    if !project_root.join("package.json").exists() {
        eprintln!("❌ No package.json found in current directory");
        return Ok(());
    }

    let (npm_bin, npm_args) = get_npm_cmd();

    println!("\n📦 Installing dependencies from package.json...\n");

    let mut install_args = npm_args.clone();
    install_args.push("install");

    let output = Command::new(npm_bin)
        .args(&install_args)
        .current_dir(&project_root)
        .output()?;

    if output.status.success() {
        println!("✅ Dependencies installed successfully!\n");
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            println!("{}", stdout);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ npm install failed:");
        eprintln!("{}", stderr);
    }

    Ok(())
}

pub fn add_and_install(package: &str, dev: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let (npm_bin, npm_args) = get_npm_cmd();

    println!("\n📦 Adding package: {}\n", package);

    let mut install_args = npm_args.clone();
    install_args.push("install");

    if dev {
        install_args.push("--save-dev");
    } else {
        install_args.push("--save");
    }

    install_args.push(package);

    let output = Command::new(npm_bin)
        .args(&install_args)
        .current_dir(&project_root)
        .output()?;

    if output.status.success() {
        println!("✅ {} added successfully!\n", package);
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            println!("{}", stdout);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ Failed to add {}:", package);
        eprintln!("{}", stderr);
    }

    Ok(())
}
