use thiserror::Error;

#[derive(Error, Debug)]
pub enum StructaError {
    #[error("Lexical error: {0}")]
    Lexical(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Compilation error: {0}")]
    Compilation(String),

    #[error("Link error: {0}")]
    Link(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Invalid syntax: {0}")]
    Syntax(String),

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

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, StructaError>;
