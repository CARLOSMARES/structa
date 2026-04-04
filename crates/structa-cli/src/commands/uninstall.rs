use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use tracing::info;

#[cfg(target_os = "windows")]
fn run_npm(args: &[&str], path: &PathBuf) -> Result<()> {
    let mut cmd = Command::new("cmd");
    cmd.args(["/c", "npm"]);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.current_dir(path);
    let output = cmd.output()?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        anyhow::bail!("npm command failed");
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn run_npm(args: &[&str], path: &PathBuf) -> Result<()> {
    let mut cmd = Command::new("npm");
    for arg in args {
        cmd.arg(arg);
    }
    cmd.current_dir(path);
    let output = cmd.output()?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        anyhow::bail!("npm command failed");
    }

    Ok(())
}

pub fn run(path: Option<PathBuf>, package: &str) -> Result<()> {
    let project_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());

    let full_package = if package.starts_with("@structa/") || package.starts_with("@") {
        package.to_string()
    } else {
        format!("@structa/{}", package)
    };

    info!("Uninstalling {} from: {:?}", full_package, project_path);

    println!("🗑️  Uninstalling {}...", full_package);
    run_npm(&["uninstall", &full_package], &project_path)?;

    println!("✅ {} uninstalled successfully!", full_package);
    Ok(())
}
