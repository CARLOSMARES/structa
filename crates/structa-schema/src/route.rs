use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSchema {
    pub name: String,
    pub path: String,
    pub routes: Vec<Route>,
    pub middleware: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: String,
    pub guards: Option<Vec<String>>,
    pub middleware: Option<Vec<String>>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }
}

pub fn parse_route(name: &str, content: &str) -> Result<RouteSchema, String> {
    let mut schema = RouteSchema {
        name: name.to_string(),
        path: "/".to_string(),
        routes: Vec::new(),
        middleware: None,
    };

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") || line.starts_with("#") {
            continue;
        }

        if line.starts_with("path ") {
            if let Some(path) = line.strip_prefix("path ") {
                schema.path = path.trim().to_string();
            }
        } else if line.starts_with("middleware ") {
            if let Some(mw) = line.strip_prefix("middleware ") {
                schema.middleware = Some(mw.split(',').map(|s| s.trim().to_string()).collect());
            }
        } else if let Some(route) = parse_route_line(line) {
            schema.routes.push(route);
        }
    }

    Ok(schema)
}

fn parse_route_line(line: &str) -> Option<Route> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let method = match parts[0].to_uppercase().as_str() {
        "GET" => HttpMethod::Get,
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "PATCH" => HttpMethod::Patch,
        "DELETE" => HttpMethod::Delete,
        "HEAD" => HttpMethod::Head,
        "OPTIONS" => HttpMethod::Options,
        _ => return None,
    };

    let path = parts[1].to_string();
    let handler = parts[2].to_string();

    let mut guards: Option<Vec<String>> = None;
    let mut middleware: Option<Vec<String>> = None;
    let mut summary: Option<String> = None;

    let extra: Vec<&str> = parts[3..].to_vec();
    for chunk in extra.chunks(2) {
        if chunk.len() == 2 {
            match chunk[0] {
                "guards" => {
                    guards = Some(chunk[1].split(',').map(|s| s.to_string()).collect());
                }
                "middleware" | "mw" => {
                    middleware = Some(chunk[1].split(',').map(|s| s.to_string()).collect());
                }
                "summary" | "desc" => {
                    summary = Some(chunk[1].trim_matches('"').to_string());
                }
                _ => {}
            }
        }
    }

    Some(Route {
        method,
        path,
        handler,
        guards,
        middleware,
        summary,
    })
}
