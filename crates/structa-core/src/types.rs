use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleType {
    Controller,
    Service,
    Dto,
    Model,
    Module,
    Route,
    Middleware,
    App,
    Entity,
    Repository,
    Resolver,
    Config,
    Interceptor,
    Guard,
    Pipe,
    Filter,
}

impl ModuleType {
    pub fn from_filename(filename: &str) -> Option<Self> {
        let lower = filename.to_lowercase();

        if lower.contains("controller") {
            Some(ModuleType::Controller)
        } else if lower.contains("service") {
            Some(ModuleType::Service)
        } else if lower.contains("dto") {
            Some(ModuleType::Dto)
        } else if lower.contains("model") {
            Some(ModuleType::Model)
        } else if lower.contains("module") {
            Some(ModuleType::Module)
        } else if lower.contains("route") {
            Some(ModuleType::Route)
        } else if lower.contains("middleware") {
            Some(ModuleType::Middleware)
        } else if lower.contains("entity") {
            Some(ModuleType::Entity)
        } else if lower.contains("repository") {
            Some(ModuleType::Repository)
        } else if lower.contains("resolver") {
            Some(ModuleType::Resolver)
        } else if lower.contains("config") {
            Some(ModuleType::Config)
        } else if lower.contains("interceptor") {
            Some(ModuleType::Interceptor)
        } else if lower.contains("guard") {
            Some(ModuleType::Guard)
        } else if lower.contains("pipe") {
            Some(ModuleType::Pipe)
        } else if lower.contains("filter") {
            Some(ModuleType::Filter)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleType::Controller => "Controller",
            ModuleType::Service => "Service",
            ModuleType::Dto => "Dto",
            ModuleType::Model => "Model",
            ModuleType::Module => "Module",
            ModuleType::Route => "Route",
            ModuleType::Middleware => "Middleware",
            ModuleType::App => "App",
            ModuleType::Entity => "Entity",
            ModuleType::Repository => "Repository",
            ModuleType::Resolver => "Resolver",
            ModuleType::Config => "Config",
            ModuleType::Interceptor => "Interceptor",
            ModuleType::Guard => "Guard",
            ModuleType::Pipe => "Pipe",
            ModuleType::Filter => "Filter",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: String,
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub struct CompilationError {
    pub message: String,
    pub location: Option<SourceLocation>,
}

impl CompilationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            location: None,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = &self.location {
            write!(f, "{}: {}", loc, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for CompilationError {}
