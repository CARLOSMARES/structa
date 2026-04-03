use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
}

impl Metadata {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            annotations: HashMap::new(),
            properties: HashMap::new(),
        }
    }

    pub fn with_annotation(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.annotations.insert(key.into(), value.into());
        self
    }

    pub fn with_property<T: Serialize>(mut self, key: impl Into<String>, value: T) -> Self {
        self.properties
            .insert(key.into(), serde_json::to_value(value).unwrap_or_default());
        self
    }

    pub fn get_annotation(&self, key: &str) -> Option<&str> {
        self.annotations.get(key).map(|s| s.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    pub name: String,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub declarations: Vec<String>,
    pub providers: Vec<String>,
}

impl ModuleMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            imports: Vec::new(),
            exports: Vec::new(),
            declarations: Vec::new(),
            providers: Vec::new(),
        }
    }

    pub fn import_module(mut self, module: impl Into<String>) -> Self {
        self.imports.push(module.into());
        self
    }

    pub fn export_provider(mut self, provider: impl Into<String>) -> Self {
        self.exports.push(provider.into());
        self
    }

    pub fn declare_provider(mut self, provider: impl Into<String>) -> Self {
        self.declarations.push(provider.into());
        self
    }
}
