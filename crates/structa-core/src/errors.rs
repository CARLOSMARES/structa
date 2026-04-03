use thiserror::Error;

#[derive(Error, Debug)]
pub enum StructaError {
    #[error("Provider not found for token: {0}")]
    ProviderNotFound(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Scope error: {0}")]
    ScopeError(String),

    #[error("Module error: {0}")]
    ModuleError(String),

    #[error("Plugin error: {0}")]
    PluginError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, StructaError>;
