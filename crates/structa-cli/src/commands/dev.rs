use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub struct DevOptions {
    pub source: PathBuf,
    pub port: u16,
    pub hot_reload: bool,
    pub no_compile: bool,
}

pub async fn run(source: PathBuf, port: u16, hot_reload: bool, _no_compile: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    
    info!("Starting Structa development server");
    info!("Project root: {:?}", project_root);
    info!("Port: {}", port);
    info!("Hot reload: {}", hot_reload);
    
    let mut server = crate::commands::dev_server::DevServer::new(project_root, port, hot_reload);
    server.run().await?;
    
    Ok(())
}
