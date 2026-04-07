use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub async fn run(path: PathBuf, name: String, _template: String) -> Result<()> {
    info!("Initializing Structa project: {} at {:?}", name, path);
    
    let project_dir = path.join(&name);
    
    std::fs::create_dir_all(&project_dir)?;
    
    let src_dir = project_dir.join("src");

    std::fs::create_dir_all(&src_dir)?;

    let main_file = src_dir.join("main.structa");
    let main_content = r#"import { createServer } from '@structa/http';

const app = createServer({
    port: 3000
});

app.get('/', () => ({
    name: 'Structa API',
    version: '1.0.0',
    status: 'running'
}));

app.get('/health', () => ({ status: 'ok' }));

app.listen();
"#;
    std::fs::write(&main_file, main_content)?;
    
    // Create package.json
    let package_json = project_dir.join("package.json");
    let package_content = format!(r#"{{
    "name": "{}",
    "version": "0.1.0",
    "type": "module",
    "description": "Structa API - TypeScript-like framework in Rust",
    "scripts": {{
        "dev": "structa dev",
        "build": "structa build"
    }},
    "dependencies": {{
        "@structa/http": ">=0.8.5"
    }}
}}"#, name);
    std::fs::write(&package_json, package_content)?;

    // Create README.md
    let readme_file = project_dir.join("README.md");
    let readme_content = format!(r#"# {}

## Structa API

A TypeScript-like API framework powered by Rust.

## Getting Started

```bash
npm install
npm run dev
```

## Learn More

Visit https://structa.dev for documentation
"#, name);
    std::fs::write(&readme_file, readme_content)?;

    info!("Project {} initialized successfully!", name);
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  npm install");
    println!("  npm run dev");
    println!();
    println!("API available at: http://localhost:3000/");
    println!();
    
    Ok(())
}
