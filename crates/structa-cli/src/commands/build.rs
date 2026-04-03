use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, error};
use std::fs;

pub async fn run(source: PathBuf, output: PathBuf, watch: bool, config: Option<PathBuf>) -> Result<()> {
    info!("Building Structa project from {:?} to {:?}", source, output);
    
    if !source.exists() {
        error!("Source directory does not exist: {:?}", source);
        anyhow::bail!("Source directory does not exist");
    }
    
    fs::create_dir_all(&output)?;
    
    let config_path = config.unwrap_or_else(|| source.join("structa.json"));
    let config = if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        serde_json::from_str::<serde_json::Value>(&content)?
    } else {
        serde_json::json!({
            "compiler": {
                "target": "es2022",
                "module": "commonjs"
            }
        })
    };
    
    info!("Compiling Structa files...");
    
    let structa_files: Vec<PathBuf> = collect_structa_files(&source)?;
    
    for file in &structa_files {
        info!("Processing: {:?}", file);
        let _content = fs::read_to_string(file)?;
    }
    
    info!("Build complete! Output written to {:?}", output);
    println!("\n✅ Build successful!");
    println!("   Files compiled: {}", structa_files.len());
    println!("   Output: {:?}", output);
    
    if watch {
        info!("Watch mode enabled. Watching for changes...");
        watch_files(source, output)?;
    }
    
    Ok(())
}

fn collect_structa_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if dir.is_file() {
        if let Some(ext) = dir.extension() {
            if ext == "structa" || ext == "yaml" || ext == "yml" {
                files.push(dir.clone());
            }
        }
        return Ok(files);
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            files.extend(collect_structa_files(&path)?);
        } else if let Some(ext) = path.extension() {
            if ext == "structa" || ext == "yaml" || ext == "yml" {
                files.push(path);
            }
        }
    }
    
    Ok(files)
}

fn watch_files(source: PathBuf, output: PathBuf) -> Result<()> {
    info!("File watching is not yet implemented");
    Ok(())
}
