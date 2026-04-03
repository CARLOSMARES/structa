use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub fn run(output: PathBuf, format: Option<String>) -> Result<()> {
    let format = format.unwrap_or_else(|| "openapi".to_string());

    info!("Generating documentation in {} format", format);

    std::fs::create_dir_all(&output)?;

    match format.as_str() {
        "openapi" | "swagger" => {
            let spec = generate_openapi_spec();
            let spec_path = output.join("openapi.json");
            std::fs::write(&spec_path, serde_json::to_string_pretty(&spec)?)?;
            info!("Generated: {:?}", spec_path);
        }
        "html" | "redoc" => {
            let html = generate_redoc_html();
            let html_path = output.join("api.html");
            std::fs::write(&html_path, html)?;
            info!("Generated: {:?}", html_path);
        }
        _ => {
            anyhow::bail!("Unknown format: {}. Use 'openapi' or 'html'", format);
        }
    }

    println!("\n✅ Documentation generated successfully!");

    Ok(())
}

fn generate_openapi_spec() -> serde_json::Value {
    serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Structa API",
            "version": "1.0.0"
        },
        "paths": {}
    })
}

fn generate_redoc_html() -> String {
    r#"<!DOCTYPE html>
<html>
<head>
    <title>API Documentation</title>
    <redoc spec-url='openapi.json'></redoc>
    <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"></script>
</head>
<body>
    <h1>API Documentation</h1>
    <p>Loading...</p>
</body>
</html>"#
        .to_string()
}
