use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructaConfig {
    pub name: String,
    pub template: String,
    pub compiler: CompilerConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    pub target: String,
    pub module: String,
    #[serde(default = "default_true")]
    pub strict: bool,
    #[serde(default = "default_true")]
    pub experimental_decorators: bool,
    #[serde(default = "default_true")]
    pub emit_decorator_metadata: bool,
    pub out_dir: String,
    pub root_dir: String,
    #[serde(default = "default_true")]
    pub declaration: bool,
    #[serde(default = "default_true")]
    pub source_map: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_true")]
    pub cors: bool,
}

fn default_true() -> bool {
    true
}

fn default_port() -> u16 {
    3000
}

fn default_host() -> String {
    "localhost".to_string()
}

impl Default for StructaConfig {
    fn default() -> Self {
        Self {
            name: "my-api".to_string(),
            template: "latest".to_string(),
            compiler: CompilerConfig {
                target: "es2022".to_string(),
                module: "commonjs".to_string(),
                strict: true,
                experimental_decorators: true,
                emit_decorator_metadata: true,
                out_dir: "./dist".to_string(),
                root_dir: "./src".to_string(),
                declaration: true,
                source_map: true,
            },
            server: ServerConfig {
                port: 3000,
                host: "localhost".to_string(),
                cors: true,
            },
        }
    }
}
