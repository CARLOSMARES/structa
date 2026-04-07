use crate::types::{CompilationError, ModuleType};
use crate::StructaError;
use std::collections::HashMap;
use std::path::Path;

pub struct Linker {
    modules: HashMap<String, CompiledModule>,
    entry_point: Option<String>,
    output_dir: String,
}

#[derive(Debug, Clone)]
pub struct CompiledModule {
    pub name: String,
    pub module_type: ModuleType,
    pub source: String,
    pub path: String,
    pub dependencies: Vec<String>,
}

impl Linker {
    pub fn new(output_dir: &str) -> Self {
        Self {
            modules: HashMap::new(),
            entry_point: None,
            output_dir: output_dir.to_string(),
        }
    }

    pub fn add_module(&mut self, name: &str, module_type: ModuleType, source: String, path: &str) {
        let deps = self.extract_dependencies(&source);

        self.modules.insert(
            name.to_string(),
            CompiledModule {
                name: name.to_string(),
                module_type,
                source,
                path: path.to_string(),
                dependencies: deps,
            },
        );
    }

    pub fn set_entry_point(&mut self, name: &str) {
        self.entry_point = Some(name.to_string());
    }

    pub fn link(&self) -> Result<String, StructaError> {
        let mut output = String::new();

        output.push_str("// ═══════════════════════════════════════════════════════════════\n");
        output.push_str("// Structa Framework - Linked Output\n");
        output.push_str("// ═══════════════════════════════════════════════════════════════\n\n");

        let sorted = self.topological_sort()?;

        for name in &sorted {
            if let Some(module) = self.modules.get(name) {
                output.push_str(&format!("// Module: {} ({:?})\n", name, module.module_type));
                output.push_str(&module.source);
                output.push_str("\n\n");
            }
        }

        if let Some(entry) = &self.entry_point {
            output
                .push_str("\n// ═══════════════════════════════════════════════════════════════\n");
            output.push_str("// Entry Point Bootstrap\n");
            output
                .push_str("// ═══════════════════════════════════════════════════════════════\n\n");
            output.push_str(&format!("const app = new {}();\n", entry));
            output.push_str("app.configure();\n");
            output.push_str("server.listen();\n");
        }

        Ok(output)
    }

    pub fn link_to_file(&self, filename: &str) -> Result<(), StructaError> {
        let output = self.link()?;
        std::fs::write(filename, output).map_err(|e| StructaError::Io(e))?;
        Ok(())
    }

    fn extract_dependencies(&self, source: &str) -> Vec<String> {
        let mut deps = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") {
                if let Some(from_idx) = trimmed.find("from ") {
                    let import_part = &trimmed[from_idx + 5..];
                    if let Some(end_idx) = import_part.find(';') {
                        let path = &import_part[..end_idx].trim();
                        if path.starts_with('\'') || path.starts_with('"') {
                            let dep = path
                                .trim_matches(|c| c == '\'' || c == '"' || c == '/' || c == '.');
                            if !dep.starts_with('@') && !deps.iter().any(|d: &String| d == dep) {
                                deps.push(dep.to_string());
                            }
                        }
                    }
                }
            }
        }

        deps
    }

    fn topological_sort(&self) -> Result<Vec<String>, StructaError> {
        let mut sorted = Vec::new();
        let mut visited = HashMap::new();

        for name in self.modules.keys() {
            if !visited.contains_key(name) {
                self.visit(name, &mut sorted, &mut visited)?;
            }
        }

        Ok(sorted)
    }

    fn visit(
        &self,
        name: &str,
        sorted: &mut Vec<String>,
        visited: &mut HashMap<String, bool>,
    ) -> Result<(), StructaError> {
        if let Some(in_progress) = visited.get(name) {
            if *in_progress {
                return Err(StructaError::Link(format!(
                    "Circular dependency detected involving module: {}",
                    name
                )));
            }
            return Ok(());
        }

        visited.insert(name.to_string(), true);

        if let Some(module) = self.modules.get(name) {
            for dep in &module.dependencies {
                let dep_name = dep.trim_end_matches(".js").trim_end_matches(".structa");
                if !visited.contains_key(dep_name) {
                    self.visit(dep_name, sorted, visited)?;
                }
            }
        }

        visited.insert(name.to_string(), false);
        sorted.push(name.to_string());

        Ok(())
    }

    pub fn generate_runtime(&self) -> String {
        let mut runtime = String::new();

        runtime.push_str(r#"
// ═══════════════════════════════════════════════════════════════
// Structa Runtime - Minimal JavaScript Runtime
// ═══════════════════════════════════════════════════════════════

const _routes = [];
const _middleware = [];
let _server = null;

export function createServer(options = {}) {
    const port = options.port || process.env.PORT || 3000;
    const host = options.host || process.env.HOST || '0.0.0.0';
    
    _server = {
        port,
        host,
        
        route(config) {
            _routes.push(config);
        },
        
        use(fn) {
            _middleware.push(fn);
        },
        
        listen() {
            const http = require('http');
            const server = http.createServer((req, res) => {
                res.setHeader('Access-Control-Allow-Origin', '*');
                res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, PATCH, OPTIONS');
                res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
                
                if (req.method === 'OPTIONS') {
                    res.writeHead(204);
                    res.end();
                    return;
                }
                
                for (const mw of _middleware) {
                    mw(req, res);
                }
                
                let path = req.url.split('?')[0];
                const method = req.method.toUpperCase();
                
                for (const route of _routes) {
                    if (route.method !== method && route.method !== 'ALL') continue;
                    
                    const pattern = route.path.replace(/:(\w+)/g, '([^/]+)');
                    const regex = new RegExp(`^${pattern}$`);
                    const match = path.match(regex);
                    
                    if (match) {
                        const params = match.slice(1);
                        const ctx = { req, res, params };
                        
                        try {
                            const result = route.handler(ctx);
                            if (result && typeof result.then === 'function') {
                                result.then(data => {
                                    res.writeHead(200, { 'Content-Type': 'application/json' });
                                    res.end(JSON.stringify(data));
                                }).catch(err => {
                                    res.writeHead(500, { 'Content-Type': 'application/json' });
                                    res.end(JSON.stringify({ error: err.message }));
                                });
                            } else {
                                res.writeHead(200, { 'Content-Type': 'application/json' });
                                res.end(JSON.stringify(result));
                            }
                        } catch (err) {
                            res.writeHead(500, { 'Content-Type': 'application/json' });
                            res.end(JSON.stringify({ error: err.message }));
                        }
                        return;
                    }
                }
                
                res.writeHead(404, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: 'Not Found', path }));
            });
            
            server.listen(port, host, () => {
                console.log(`\x1b[32m🚀 Structa running at http://${host}:${port}\x1b[0m`);
            });
            
            return server;
        }
    };
    
    return _server;
}

export function server() {
    return _server;
}

export { _routes as routes, _middleware as middleware };
"#);

        runtime
    }
}
