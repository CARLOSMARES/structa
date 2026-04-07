use anyhow::Result;
use std::path::PathBuf;
use chrono::Local;
use structa_compiler::{compile, Lexer, Parser};

pub async fn run(release: bool, output: PathBuf) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let src_dir = project_root.join("src");
    
    if !src_dir.exists() {
        log_error(&format!("Source directory not found: {:?}", src_dir));
        return Ok(());
    }
    
    log_info(&format!("Mode: {}", if release { "release" } else { "debug" }));
    log_info(&format!("Output: {:?}", output));
    
    std::fs::create_dir_all(&output)?;
    
    let mut files = 0;
    let mut errors = 0;
    
    for entry in walkdir(&src_dir) {
        if let Some(path) = entry {
            let p = std::path::Path::new(&path);
            if p.extension().map_or(false, |ext| ext == "structa") {
                match std::fs::read_to_string(&path) {
                    Ok(source) => {
                        match compile_structa_file(&source) {
                            Ok(js) => {
                                let mut target = output.join(p.strip_prefix(&project_root).unwrap_or(p));
                                target.set_extension("js");
                                if let Some(parent) = target.parent() {
                                    std::fs::create_dir_all(parent)?;
                                }
                                std::fs::write(&target, &js)?;
                                files += 1;
                                log_info(&format!("Compiled: {}", p.file_name().unwrap_or_default().to_string_lossy()));
                            }
                            Err(e) => {
                                errors += 1;
                                log_error(&format!("Error in {}: {}", p.file_name().unwrap_or_default().to_string_lossy(), e));
                            }
                        }
                    }
                    Err(e) => {
                        errors += 1;
                        log_error(&format!("Failed to read {}: {}", path, e));
                    }
                }
            } else if p.extension().map_or(false, |ext| ext == "js") {
                let mut target = output.join(p.strip_prefix(&project_root).unwrap_or(p));
                if let Some(parent) = target.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                if let Ok(content) = std::fs::read_to_string(&path) {
                    std::fs::write(&target, content)?;
                    files += 1;
                }
            }
        }
    }
    
    log_info(&format!("Total files compiled: {}", files));
    
    if errors > 0 {
        log_warn(&format!("Build completed with {} errors", errors));
    } else {
        log_success("Build completed successfully");
    }
    
    Ok(())
}

fn compile_structa_file(source: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let prog = parser.parse();
    let js = compile(&prog);
    Ok(js)
}

fn walkdir(dir: &std::path::Path) -> Vec<Option<String>> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walkdir(&path));
            } else {
                files.push(Some(path.to_string_lossy().to_string()));
            }
        }
    }
    files
}

fn log_info(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("\x1b[32m[{}]\x1b[0m \x1b[36mINFO\x1b[0m     \x1b[32m→\x1b[0m {}", timestamp, msg);
}

fn log_warn(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("\x1b[32m[{}]\x1b[0m \x1b[33mWARN\x1b[0m     \x1b[32m→\x1b[0m \x1b[33m{}\x1b[0m", timestamp, msg);
}

fn log_error(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("\x1b[32m[{}]\x1b[0m \x1b[31mERROR\x1b[0m    \x1b[32m→\x1b[0m \x1b[31m{}\x1b[0m", timestamp, msg);
}

fn log_success(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("\x1b[32m[{}]\x1b[0m \x1b[32mOK\x1b[0m      \x1b[32m→\x1b[0m \x1b[32m{}\x1b[0m", timestamp, msg);
}
