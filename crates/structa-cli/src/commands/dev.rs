use anyhow::Result;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tracing::info;

pub struct DevOptions {
    pub source: PathBuf,
    pub port: u16,
    pub hot_reload: bool,
    pub no_compile: bool,
}

pub async fn run(source: PathBuf, port: u16, hot_reload: bool, no_compile: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    
    info!("Starting Structa development server");
    info!("Project root: {:?}", project_root);
    info!("Port: {}", port);
    info!("No-compile mode: {}", no_compile);
    
    println!("\n🚀 Structa Dev Server");
    println!("   Local:   http://localhost:{}", port);
    println!("   Network: http://0.0.0.0:{}", port);
    println!("\n   Press Ctrl+C to stop\n");

    let node_modules = project_root.join("node_modules");
    let package_json = project_root.join("package.json");
    
    if !node_modules.exists() || !package_json.exists() {
        println!("❌ No project found. Run 'structa init' first to create a project.");
        return Ok(());
    }

    println!("🔍 Discovering routes...");
    let controller_count = discover_controllers(&project_root);
    if controller_count == 0 {
        println!("⚠️  No controllers found. Create controllers with @Controller decorator.");
    } else {
        println!("📦 Found {} controller(s)", controller_count);
    }

    let main_path = if no_compile {
        project_root.join("src").join("main.ts")
    } else {
        println!("\n📦 Building TypeScript...");
        let build_success = run_typescript_build(&project_root);
        
        if !build_success {
            println!("⚠️  Build failed. Trying to start anyway...");
        }
        
        if project_root.join("dist").join("main.js").exists() {
            project_root.join("dist").join("main.js")
        } else {
            project_root.join("src").join("main.ts")
        }
    };

    println!("\n🔄 Starting server...\n");

    let entry_display = main_path.display().to_string();
    println!("   Entry: {}", entry_display);
    println!();

    let mut cmd = if no_compile {
        run_ts_direct(&project_root, &main_path)?
    } else {
        let mut c = Command::new("node");
        c.arg(&main_path);
        c
    };

    cmd.env("PORT", port.to_string())
       .env("STRUCTA_MODE", "development")
       .current_dir(&project_root)
       .stdout(Stdio::inherit())
       .stderr(Stdio::inherit());

    let mut child = cmd.spawn()?;

    tokio::signal::ctrl_c().await?;
    let _ = child.kill();

    println!("\n👋 Server stopped");
    
    Ok(())
}

fn run_ts_direct(project_root: &PathBuf, main_path: &PathBuf) -> Result<Command> {
    let tsx_path = project_root.join("node_modules").join(".bin").join(if cfg!(target_os = "windows") { "tsx.cmd" } else { "tsx" });
    let ts_node_path = project_root.join("node_modules").join(".bin").join(if cfg!(target_os = "windows") { "ts-node.cmd" } else { "ts-node" });

    let mut cmd = if tsx_path.exists() {
        if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.args(["/c", &tsx_path.to_string_lossy(), &main_path.to_string_lossy()]);
            c
        } else {
            let mut c = Command::new(&tsx_path);
            c.arg(&main_path);
            c
        }
    } else if ts_node_path.exists() {
        if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.args(["/c", &ts_node_path.to_string_lossy(), &main_path.to_string_lossy()]);
            c
        } else {
            let mut c = Command::new(&ts_node_path);
            c.arg(&main_path);
            c
        }
    } else {
        println!("⚠️  No TypeScript runner found (tsx or ts-node).");
        println!("   Installing tsx for fast TypeScript execution...");
        
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/c", "npm install -D tsx"])
                .current_dir(project_root)
                .output()?;
        } else {
            Command::new("npm")
                .args(["install", "-D", "tsx"])
                .current_dir(project_root)
                .output()?;
        }
        
        println!("✅ tsx installed. Run 'structa dev' again.");
        return Err(anyhow::anyhow!("tsx installed, please run again"));
    };

    Ok(cmd)
}

fn discover_controllers(project_root: &PathBuf) -> usize {
    let src_dir = project_root.join("src");
    if !src_dir.exists() {
        return 0;
    }
    
    let mut count = 0;
    
    fn walk_dir(dir: &PathBuf, count: &mut usize) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path, count);
                } else if path.extension().map_or(false, |e| e == "ts") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if content.contains("@Controller") {
                            *count += 1;
                        }
                    }
                }
            }
        }
    }
    
    walk_dir(&src_dir, &mut count);
    count
}

fn run_typescript_build(project_root: &PathBuf) -> bool {
    let npx_result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "npm exec -- tsc -p tsconfig.json"])
            .current_dir(project_root)
            .output()
    } else {
        Command::new("npx")
            .args(["tsc", "-p", "tsconfig.json"])
            .current_dir(project_root)
            .output()
    };

    match npx_result {
        Ok(output) => {
            if output.status.success() {
                println!("✅ TypeScript compiled successfully!");
                return true;
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.is_empty() {
                    println!("⚠️  TypeScript warnings/errors:");
                    for line in stderr.lines().take(5) {
                        println!("   {}", line);
                    }
                }
                if stderr.contains("error TS") {
                    return false;
                }
                return true;
            }
        }
        Err(e) => {
            println!("⚠️  TypeScript not found: {}", e);
            println!("   Run 'npm install' to install dependencies");
            return false;
        }
    }
}
