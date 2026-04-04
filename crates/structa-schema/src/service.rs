use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSchema {
    pub name: String,
    pub methods: Vec<ServiceMethod>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMethod {
    pub name: String,
    pub return_type: String,
    pub params: Vec<MethodParam>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodParam {
    pub name: String,
    pub param_type: String,
    pub optional: bool,
}

pub fn parse_service(name: &str, content: &str) -> Result<ServiceSchema, String> {
    let mut schema = ServiceSchema {
        name: name.to_string(),
        methods: Vec::new(),
        dependencies: Vec::new(),
    };

    let mut current_method: Option<ServiceMethod> = None;
    let mut method_body: Vec<String> = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") || line.starts_with("#") {
            continue;
        }

        if line.starts_with("inject ") || line.starts_with("dependency ") {
            if let Some(deps) = line.strip_prefix("inject ") {
                schema.dependencies = parse_items(deps);
            } else if let Some(deps) = line.strip_prefix("dependency ") {
                schema.dependencies = parse_items(deps);
            }
        } else if let Some(method) = parse_method_signature(line) {
            if let Some(mut m) = current_method.take() {
                m.body = if method_body.is_empty() {
                    None
                } else {
                    Some(method_body.join("\n"))
                };
                schema.methods.push(m);
            }
            current_method = Some(method);
            method_body.clear();
        } else if current_method.is_some() && !line.starts_with('{') && !line.starts_with('}') {
            method_body.push(line.to_string());
        }
    }

    if let Some(mut m) = current_method {
        m.body = if method_body.is_empty() {
            None
        } else {
            Some(method_body.join("\n"))
        };
        schema.methods.push(m);
    }

    Ok(schema)
}

fn parse_method_signature(line: &str) -> Option<ServiceMethod> {
    let parts: Vec<&str> = line.split("->").collect();
    if parts.len() < 2 {
        return None;
    }

    let signature = parts[0].trim();
    let return_type = parts[1].trim().to_string();

    let paren_start = signature.find('(')?;
    let name = signature[..paren_start].trim().to_string();
    let params_str = &signature[paren_start + 1..signature.find(')')?];

    let params: Vec<MethodParam> = params_str
        .split(',')
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() {
                return None;
            }
            let parts: Vec<&str> = p.split(':').collect();
            if parts.len() < 2 {
                return Some(MethodParam {
                    name: p.to_string(),
                    param_type: "any".to_string(),
                    optional: false,
                });
            }
            let name = parts[0].trim().to_string();
            let param_type = parts[1].trim().to_string();
            let optional = param_type.ends_with('?');
            Some(MethodParam {
                name,
                param_type,
                optional,
            })
        })
        .collect();

    Some(ServiceMethod {
        name,
        return_type,
        params,
        body: None,
    })
}

fn parse_items(content: &str) -> Vec<String> {
    content
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
