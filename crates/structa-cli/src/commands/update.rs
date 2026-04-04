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

pub fn run(path: Option<PathBuf>, package: Option<&str>) -> Result<()> {
    let project_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());

    info!("Updating packages in: {:?}", project_path);

    if let Some(pkg) = package {
        if pkg.starts_with("@structa/") || pkg.starts_with("@") {
            println!("📦 Updating package: {}", pkg);
            run_npm(&["update", pkg], &project_path)?;
        } else {
            println!("📦 Updating package: @structa/{}", pkg);
            run_npm(&["update", &format!("@structa/{}", pkg)], &project_path)?;
        }
    } else {
        println!("📦 Updating all @structa packages...");
        run_npm(&["update", "@structa/*"], &project_path)?;
    }

    println!("✅ Packages updated successfully!");
    Ok(())
}

pub fn downgrade(path: Option<PathBuf>, package: &str, version: &str) -> Result<()> {
    let project_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());

    let full_package = if package.starts_with("@structa/") || package.starts_with("@") {
        package.to_string()
    } else {
        format!("@structa/{}", package)
    };

    info!(
        "Downgrading {} to version {} in: {:?}",
        full_package, version, project_path
    );

    println!("📦 Downgrading {} to version {}", full_package, version);
    run_npm(
        &["install", &format!("{}@{}", full_package, version)],
        &project_path,
    )?;

    println!(
        "✅ {} downgraded to {} successfully!",
        full_package, version
    );
    Ok(())
}
