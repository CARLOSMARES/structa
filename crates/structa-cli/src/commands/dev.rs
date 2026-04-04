use anyhow::Result;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct Route {
    method: String,
    path: String,
    handler: String,
    controller: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Controller {
    name: String,
    path: String,
    routes: Vec<Route>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StructaConfig {
    port: u16,
    host: String,
    controllers: Vec<Controller>,
}

pub async fn run(source: PathBuf, port: u16, hot_reload: bool) -> Result<()> {
    // Get the project root (parent of src)
    let project_root = if source.file_name().map_or(false, |n| n == "src") {
        source.parent().unwrap_or(&source).to_path_buf()
    } else {
        source.clone()
    };
    
    info!("Starting Structa development server");
    info!("Project root: {:?}", project_root);
    info!("Source: {:?}", source);
    info!("Port: {}", port);
    info!("Hot reload: {}", hot_reload);
    
    println!("\n🚀 Structa Dev Server");
    println!("   Local:   http://localhost:{}", port);
    println!("   Network: http://0.0.0.0:{}", port);
    println!("\n   Press Ctrl+C to stop\n");

    // Check if node_modules exists
    let node_modules = project_root.join("node_modules");
    if !node_modules.exists() {
        println!("⚠️  node_modules not found. Run 'structa install' first.");
        println!("   Installing dependencies...\n");
        
        let install_result = Command::new("npm")
            .args(&["install"])
            .current_dir(&project_root)
            .status();
            
        match install_result {
            Ok(status) if status.success() => {
                println!("✅ Dependencies installed!\n");
            }
            Ok(_) => {
                println!("⚠️  npm install failed. Please install dependencies manually.");
            }
            Err(e) => {
                println!("⚠️  npm not found: {}. Please install Node.js and npm.", e);
            }
        }
    }

    // Check for structa.json config
    let config_path = project_root.join("structa.json");
    if config_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            match serde_json::from_str::<StructaConfig>(&content) {
                Ok(_config) => {
                    println!("📋 Loaded config from structa.json");
                }
                Err(e) => {
                    println!("⚠️  Failed to parse structa.json: {}", e);
                }
            }
        }
    }

    // Auto-discover controllers
    println!("🔍 Auto-discovering controllers...");
    let controllers = discover_controllers(&project_root)?;
    if controllers.is_empty() {
        println!("⚠️  No controllers found.");
    } else {
        println!("📦 Found {} controller(s)", controllers.len());
    }

    // Build TypeScript first
    println!("\n📦 Building TypeScript...");
    let build_result = Command::new("npx")
        .args(&["tsc", "-p", "tsconfig.json"])
        .current_dir(&project_root)
        .output();
    
    match build_result {
        Ok(output) => {
            if !output.status.success() {
                println!("❌ Build failed:");
                println!("{}", String::from_utf8_lossy(&output.stderr));
            } else {
                println!("✅ Build successful!");
            }
        }
        Err(e) => {
            println!("⚠️  Could not run tsc: {}", e);
            println!("   Make sure TypeScript is installed: npm install -D typescript");
        }
    }

    // Start the server using Node.js
    println!("\n🔄 Starting server...\n");
    
    // Find main.js - check dist first, then src
    let main_path = if project_root.join("dist").join("main.js").exists() {
        project_root.join("dist").join("main.js")
    } else if project_root.join("src").join("main.js").exists() {
        project_root.join("src").join("main.js")
    } else {
        println!("❌ main.js not found in dist/ or src/");
        return Ok(());
    };

    println!("   Running: {}\n", main_path.display());

    let mut child = Command::new("node")
        .arg(&main_path)
        .env("PORT", port.to_string())
        .env("STRUCTA_MODE", "development")
        .current_dir(&project_root)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // Wait for ctrl+c
    tokio::signal::ctrl_c().await?;

    // Kill the child process
    let _ = child.kill();

    info!("Shutting down server...");
    println!("\n\n👋 Server stopped");
    
    Ok(())
}

fn discover_controllers(project_root: &PathBuf) -> Result<Vec<Controller>> {
    let mut controllers = Vec::new();
    
    // Look in src directory
    let src_dir = project_root.join("src");
    
    if !src_dir.exists() {
        return Ok(controllers);
    }
    
    fn walk_dir(dir: &PathBuf, controllers: &mut Vec<Controller>) -> Result<()> {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path, controllers)?;
                } else if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if ext_str == "ts" || ext_str == "structa" {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if ext_str == "ts" {
                                if let Some(c) = parse_typescript_file(&path, &content)? {
                                    controllers.push(c);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    walk_dir(&src_dir, &mut controllers)?;
    Ok(controllers)
}

fn parse_typescript_file(path: &PathBuf, content: &str) -> Result<Option<Controller>> {
    let file_name = path.file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("Controller")
        .to_string();

    // Check if this file has a @Controller decorator
    let has_controller = content.contains("@Controller");
    if !has_controller {
        return Ok(None);
    }

    let mut controller = Controller {
        name: file_name,
        path: "/".to_string(),
        routes: Vec::new(),
    };

    // Check for @Controller decorator
    if let Some(controller_match) = content.find("@Controller") {
        if let Some(paren_start) = content[controller_match..].find('(') {
            if let Some(paren_end) = content[controller_match..].find(')') {
                let path_content = &content[controller_match + paren_start + 1..controller_match + paren_end];
                controller.path = path_content.trim_matches(|c| c == '"' || c == '\'' || c == '`').to_string();
            }
        }
    }

    // Find route decorators - look for @Get( @Post( etc.
    let route_patterns = [
        ("GET", "Get"), ("POST", "Post"), ("PUT", "Put"), 
        ("DELETE", "Delete"), ("PATCH", "Patch"),
        ("HEAD", "Head"), ("OPTIONS", "Options")
    ];
    
    for (http_method, decorator) in route_patterns.iter() {
        let search_pattern = format!("@{}(", decorator);
        let mut search_start = 0;
        
        while let Some(pos) = content[search_start..].find(&search_pattern) {
            let actual_pos = search_start + pos;
            if let Some(paren_start) = content[actual_pos..].find('(') {
                if let Some(paren_end) = content[actual_pos..].find(')') {
                    let route_path = &content[actual_pos + paren_start + 1..actual_pos + paren_end];
                    let clean_path = route_path.trim_matches(|c| c == '"' || c == '\'' || c == '`');
                    
                    // Find the method name after this decorator
                    let method_start = actual_pos + paren_end + 1;
                    // Skip whitespace and 'async' keyword
                    let mut skip = method_start;
                    while skip < content.len() && content[skip..].chars().next().map_or(false, |c| c.is_whitespace()) {
                        skip += 1;
                    }
                    if content[skip..].starts_with("async") {
                        skip += 5;
                    }
                    while skip < content.len() && content[skip..].chars().next().map_or(false, |c| c.is_whitespace()) {
                        skip += 1;
                    }
                    
                    if let Some(fn_pos) = content[skip..].find("fn ") {
                        let fn_line_start = skip + fn_pos + 3;
                        if let Some(fn_line_end) = content[fn_line_start..].find('(') {
                            let fn_name = &content[fn_line_start..fn_line_start + fn_line_end];
                            controller.routes.push(Route {
                                method: http_method.to_string(),
                                path: clean_path.to_string(),
                                handler: fn_name.to_string(),
                                controller: controller.name.clone(),
                            });
                        }
                    }
                }
            }
            search_start = actual_pos + 1;
        }
    }

    if controller.routes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(controller))
    }
}
