use anyhow::Result;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{Read, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
    info!("Starting Structa development server");
    info!("Source: {:?}", source);
    info!("Port: {}", port);
    info!("Hot reload: {}", hot_reload);
    
    println!("\n🚀 Structa Dev Server");
    println!("   Local:   http://localhost:{}", port);
    println!("   Network: http://0.0.0.0:{}", port);
    println!("\n   Press Ctrl+C to stop\n");

    // Check for structa.json config
    let config_path = source.join("structa.json");
    let server_config = if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        match serde_json::from_str::<StructaConfig>(&content) {
            Ok(config) => {
                println!("📋 Loaded config from structa.json");
                Some(config)
            }
            Err(e) => {
                println!("⚠️  Failed to parse structa.json: {}", e);
                None
            }
        }
    } else {
        // Auto-discover controllers
        println!("🔍 Auto-discovering controllers...");
        let controllers = discover_controllers(&source)?;
        if controllers.is_empty() {
            println!("⚠️  No controllers found. Create .structa files or use @Controller decorator.");
        } else {
            println!("📦 Found {} controller(s)", controllers.len());
        }
        Some(StructaConfig {
            port,
            host: "0.0.0.0".to_string(),
            controllers,
        })
    };

    // Build TypeScript first
    println!("📦 Building TypeScript...");
    let build_result = Command::new("npx")
        .args(&["tsc", "-p", "tsconfig.json"])
        .current_dir(&source)
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
    
    let dist_path = source.join("dist");
    let main_path = if dist_path.join("main.js").exists() {
        dist_path.join("main.js")
    } else {
        source.join("src").join("main.js")
    };

    let mut child = Command::new("node")
        .arg(&main_path)
        .env("PORT", port.to_string())
        .env("STRUCTA_MODE", "development")
        .current_dir(&source)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // Wait for ctrl+c
    tokio::signal::ctrl_c().await?;

    // Kill the child process
    child.kill()?;

    info!("Shutting down server...");
    println!("\n\n👋 Server stopped");
    
    Ok(())
}

fn discover_controllers(source: &PathBuf) -> Result<Vec<Controller>> {
    let mut controllers = Vec::new();
    
    // Look for .structa files
    if let Ok(entries) = std::fs::read_dir(source) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                controllers.extend(discover_controllers(&path)?);
            } else if let Some(ext) = path.extension() {
                if ext == "structa" {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let controller = parse_structa_file(&path, &content)?;
                        if let Some(c) = controller {
                            controllers.push(c);
                        }
                    }
                }
            }
        }
    }

    // Also check for TypeScript files with @Controller decorator
    if let Ok(entries) = std::fs::read_dir(source.join("src")) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "ts") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let controller = parse_typescript_controller(&path, &content)?;
                    if let Some(c) = controller {
                        controllers.push(c);
                    }
                }
            }
        }
    }

    Ok(controllers)
}

fn parse_structa_file(path: &PathBuf, content: &str) -> Result<Option<Controller>> {
    let file_name = path.file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("Controller");

    let mut controller = Controller {
        name: file_name.to_string(),
        path: "/".to_string(),
        routes: Vec::new(),
    };

    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with("path ") {
            if let Some(path) = line.strip_prefix("path ") {
                controller.path = path.trim_matches('"').to_string();
            }
        } else if line.starts_with("GET ") || line.starts_with("POST ") || 
                  line.starts_with("PUT ") || line.starts_with("DELETE ") ||
                  line.starts_with("PATCH ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let method = parts[0].to_string();
                let route_path = parts[1].to_string();
                let handler = parts[2].to_string();
                
                controller.routes.push(Route {
                    method,
                    path: route_path,
                    handler,
                    controller: controller.name.clone(),
                });
            }
        }
    }

    if controller.routes.is_empty() && controller.path == "/" {
        Ok(None)
    } else {
        Ok(Some(controller))
    }
}

fn parse_typescript_controller(path: &PathBuf, content: &str) -> Result<Option<Controller>> {
    let file_name = path.file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("Controller")
        .to_string();

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

    // Find route decorators
    let methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];
    for method in methods {
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find(&format!("@{}(", method)) {
            let actual_pos = search_start + pos;
            if let Some(paren_start) = content[actual_pos..].find('(') {
                if let Some(paren_end) = content[actual_pos..].find(')') {
                    let route_path = &content[actual_pos + paren_start + 1..actual_pos + paren_end];
                    let clean_path = route_path.trim_matches(|c| c == '"' || c == '\'' || c == '`');
                    
                    // Find the method name after this decorator
                    let method_start = actual_pos + paren_end + 1;
                    if let Some(fn_pos) = content[method_start..].find("fn ") {
                        let fn_line_start = method_start + fn_pos + 3;
                        if let Some(fn_line_end) = content[fn_line_start..].find('(') {
                            let fn_name = &content[fn_line_start..fn_line_start + fn_line_end];
                            controller.routes.push(Route {
                                method: method.to_string(),
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
