use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSchema {
    pub name: String,
    pub version: String,
    pub port: u16,
    pub host: String,
    pub modules: Vec<String>,
    pub middlewares: Vec<MiddlewareConfig>,
    pub cors: Option<CorsConfig>,
    pub swagger: Option<SwaggerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareConfig {
    pub name: String,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub origin: Option<String>,
    pub methods: Option<Vec<String>>,
    pub credentials: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwaggerConfig {
    pub title: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub path: Option<String>,
}

pub fn parse_app(content: &str) -> Result<AppSchema, String> {
    let mut schema = AppSchema {
        name: String::new(),
        version: "1.0.0".to_string(),
        port: 3000,
        host: "0.0.0.0".to_string(),
        modules: Vec::new(),
        middlewares: Vec::new(),
        cors: None,
        swagger: None,
    };

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") || line.starts_with("#") {
            continue;
        }

        if line.starts_with("name ") || line.starts_with("app ") {
            if let Some(name) = line.strip_prefix("name ") {
                schema.name = name.trim().to_string();
            } else if let Some(name) = line.strip_prefix("app ") {
                schema.name = name.trim().to_string();
            }
        } else if line.starts_with("version ") {
            if let Some(v) = line.strip_prefix("version ") {
                schema.version = v.trim().to_string();
            }
        } else if line.starts_with("port ") {
            if let Some(p) = line.strip_prefix("port ") {
                schema.port = p.trim().parse().unwrap_or(3000);
            }
        } else if line.starts_with("host ") {
            if let Some(h) = line.strip_prefix("host ") {
                schema.host = h.trim().to_string();
            }
        } else if line.starts_with("modules ") {
            if let Some(mods) = line.strip_prefix("modules ") {
                schema.modules = parse_items(mods);
            }
        } else if line.starts_with("middleware ") || line.starts_with("middlewares ") {
            if let Some(mw) = line.strip_prefix("middleware ") {
                schema.middlewares.push(parse_middleware(mw));
            } else if let Some(mw) = line.strip_prefix("middlewares ") {
                for item in mw.split(',') {
                    schema.middlewares.push(parse_middleware(item));
                }
            }
        } else if line.starts_with("cors") {
            schema.cors = Some(parse_cors(line));
        } else if line.starts_with("swagger") {
            schema.swagger = Some(parse_swagger(line));
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

fn parse_middleware(content: &str) -> MiddlewareConfig {
    let parts: Vec<&str> = content.split('(').collect();
    let name = parts[0].trim().to_string();
    let options = if parts.len() > 1 {
        let opts_str = parts[1].trim_end_matches(')');
        serde_json::from_str(opts_str).ok()
    } else {
        None
    };
    MiddlewareConfig { name, options }
}

fn parse_cors(line: &str) -> CorsConfig {
    let content = line.strip_prefix("cors").unwrap_or(line).trim();
    let content = content.trim_start_matches('(').trim_end_matches(')');

    let mut origin: Option<String> = None;
    let mut methods: Option<Vec<String>> = None;
    let mut credentials: Option<bool> = None;

    for part in content.split(',') {
        let part = part.trim();
        if part.starts_with("origin=") {
            origin = Some(
                part.strip_prefix("origin=")
                    .unwrap()
                    .trim_matches('"')
                    .to_string(),
            );
        } else if part.starts_with("methods=") {
            let m = part.strip_prefix("methods=").unwrap();
            methods = Some(m.split('|').map(|s| s.trim().to_string()).collect());
        } else if part.starts_with("credentials=") {
            credentials = Some(part.strip_prefix("credentials=").unwrap() == "true");
        }
    }

    CorsConfig {
        origin,
        methods,
        credentials,
    }
}

fn parse_swagger(line: &str) -> SwaggerConfig {
    let content = line.strip_prefix("swagger").unwrap_or(line).trim();
    let content = content.trim_start_matches('(').trim_end_matches(')');

    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut version: Option<String> = None;
    let mut path: Option<String> = None;

    for part in content.split(',') {
        let part = part.trim();
        if part.starts_with("title=") {
            title = Some(
                part.strip_prefix("title=")
                    .unwrap()
                    .trim_matches('"')
                    .to_string(),
            );
        } else if part.starts_with("description=") {
            description = Some(
                part.strip_prefix("description=")
                    .unwrap()
                    .trim_matches('"')
                    .to_string(),
            );
        } else if part.starts_with("version=") {
            version = Some(
                part.strip_prefix("version=")
                    .unwrap()
                    .trim_matches('"')
                    .to_string(),
            );
        } else if part.starts_with("path=") {
            path = Some(
                part.strip_prefix("path=")
                    .unwrap()
                    .trim_matches('"')
                    .to_string(),
            );
        }
    }

    SwaggerConfig {
        title,
        description,
        version,
        path,
    }
}
