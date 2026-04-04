use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSchema {
    pub name: String,
    pub controllers: Vec<String>,
    pub services: Vec<String>,
    pub modules: Vec<String>,
    pub exports: Vec<String>,
    pub imports: Vec<String>,
}

pub fn parse_module(name: &str, content: &str) -> Result<ModuleSchema, String> {
    let mut schema = ModuleSchema {
        name: name.to_string(),
        controllers: Vec::new(),
        services: Vec::new(),
        modules: Vec::new(),
        exports: Vec::new(),
        imports: Vec::new(),
    };

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") || line.starts_with("#") {
            continue;
        }

        if line.starts_with("controllers ") || line.starts_with("controller ") {
            if let Some(items) = line.strip_prefix("controllers ") {
                schema.controllers = parse_items(items);
            } else if let Some(items) = line.strip_prefix("controller ") {
                schema.controllers = parse_items(items);
            }
        } else if line.starts_with("services ") || line.starts_with("service ") {
            if let Some(items) = line.strip_prefix("services ") {
                schema.services = parse_items(items);
            } else if let Some(items) = line.strip_prefix("service ") {
                schema.services = parse_items(items);
            }
        } else if line.starts_with("modules ") || line.starts_with("module ") {
            if let Some(items) = line.strip_prefix("modules ") {
                schema.modules = parse_items(items);
            } else if let Some(items) = line.strip_prefix("module ") {
                schema.modules = parse_items(items);
            }
        } else if line.starts_with("exports ") || line.starts_with("export ") {
            if let Some(items) = line.strip_prefix("exports ") {
                schema.exports = parse_items(items);
            } else if let Some(items) = line.strip_prefix("export ") {
                schema.exports = parse_items(items);
            }
        } else if line.starts_with("imports ") || line.starts_with("import ") {
            if let Some(items) = line.strip_prefix("imports ") {
                schema.imports = parse_items(items);
            } else if let Some(items) = line.strip_prefix("import ") {
                schema.imports = parse_items(items);
            }
        }
    }

    Ok(schema)
}

fn parse_items(content: &str) -> Vec<String> {
    content
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
