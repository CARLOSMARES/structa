use anyhow::Result;
use std::path::PathBuf;
use chrono::Local;

pub async fn run(port: u16, hot_reload: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    
    log_info(&format!("Project root: {:?}", project_root));
    log_info(&format!("Port: {}", port));
    log_info(&format!("Hot reload: {}", hot_reload));
    
    let mut server = crate::commands::dev_server::DevServer::new(project_root, port, hot_reload);
    server.run()?;
    
    Ok(())
}

fn log_info(msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    println!("\x1b[32m[{}]\x1b[0m \x1b[36mINFO\x1b[0m     \x1b[32m→\x1b[0m {}", timestamp, msg);
}
