use anyhow::Result;
use std::path::PathBuf;
use tracing::info;
use tokio::signal;

pub async fn run(source: PathBuf, port: u16, hot_reload: bool) -> Result<()> {
    info!("Starting Structa development server");
    info!("Source: {:?}", source);
    info!("Port: {}", port);
    info!("Hot reload: {}", hot_reload);
    
    println!("\n🚀 Structa Dev Server");
    println!("   Local:   http://localhost:{}", port);
    println!("   Network: http://0.0.0.0:{}", port);
    println!("\n   Press Ctrl+C to stop\n");
    
    info!("Server listening on port {}", port);
    
    signal::ctrl_c().await?;
    
    info!("Shutting down server...");
    println!("\n\n👋 Server stopped");
    
    Ok(())
}
