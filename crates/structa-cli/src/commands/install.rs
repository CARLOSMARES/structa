use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub fn run(project_path: Option<PathBuf>) -> Result<()> {
    let path = project_path.unwrap_or_else(|| PathBuf::from("."));

    if !path.join("package.json").exists() {
        eprintln!("❌ No package.json found in current directory");
        eprintln!("   Are you in a Structa project directory?");
        return Ok(());
    }

    println!("\n📦 Installing Structa packages...\n");

    // Read structa.json to check configured packages
    let structa_config = path.join("structa.json");
    let mut packages_to_install = Vec::new();

    if structa_config.exists() {
        if let Ok(content) = std::fs::read_to_string(&structa_config) {
            // Parse basic packages from structa.json
            if content.contains("@structa/http") {
                packages_to_install.push("@structa/http");
            }
            if content.contains("@structa/orm") {
                packages_to_install.push("@structa/orm");
            }
            if content.contains("@structa/graphql") {
                packages_to_install.push("@structa/graphql");
            }
            if content.contains("@structa/websocket") {
                packages_to_install.push("@structa/websocket");
            }
            if content.contains("@structa/swagger") {
                packages_to_install.push("@structa/swagger");
            }
            if content.contains("@structa/testing") {
                packages_to_install.push("@structa/testing");
            }
        }
    }

    // Always install @structa/runtime
    packages_to_install.insert(0, "@structa/runtime");

    // Remove duplicates
    packages_to_install.dedup();

    println!("📦 Packages to install:");
    for pkg in &packages_to_install {
        println!("   - {}", pkg);
    }
    println!();

    // Install packages using npm
    if !packages_to_install.is_empty() {
        let packages_str = packages_to_install.join(" ");

        println!("🔄 Running: npm install {}\n", packages_str);

        let output = std::process::Command::new("npm")
            .args(&["install", &packages_str])
            .current_dir(&path)
            .output()?;

        if output.status.success() {
            println!("\n✅ Packages installed successfully!\n");

            // Show installed packages
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.is_empty() {
                println!("{}", stdout);
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("\n❌ Failed to install packages:");
            eprintln!("{}", stderr);
        }
    }

    // Check TypeScript installation
    let typescript_check = std::process::Command::new("npm")
        .args(&["list", "typescript"])
        .current_dir(&path)
        .output()?;

    let typescript_installed =
        String::from_utf8_lossy(&typescript_check.stdout).contains("typescript@");

    if !typescript_installed {
        println!("\n📦 Installing dev dependencies...\n");

        let dev_output = std::process::Command::new("npm")
            .args(&["install", "--save-dev", "typescript", "@types/node"])
            .current_dir(&path)
            .output()?;

        if dev_output.status.success() {
            println!("✅ Dev dependencies installed!");
        }
    }

    println!("\n✨ Setup complete!\n");
    println!("📁 Next steps:");
    println!("   structa dev          # Start development server");
    println!("   structa build         # Build for production");
    println!();

    Ok(())
}

pub fn install_package(package_name: &str) -> Result<()> {
    println!("\n📦 Installing: {}\n", package_name);

    let output = std::process::Command::new("npm")
        .args(&["install", package_name])
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
