use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

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

pub fn run(package: &str) -> Result<()> {
    let project_root = std::env::current_dir()?;

    println!("\n🗑️  Removing {}...", package);
    run_npm(&["uninstall", package], &project_root)?;
    println!("✅ {} removed successfully!", package);

    Ok(())
}
