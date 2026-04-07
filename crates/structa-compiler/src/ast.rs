use std::collections::HashMap;

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
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Source {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub enum Token {
    Eof,
    Ident(String),
    Str(String),
    Num(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Semi,
    Dot,
    Colon,
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    At,
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Controller,
    Service,
    Dto,
    Model,
    Entity,
    Repository,
    Resolver,
    Middleware,
    Inject,
    Module,
    Return,
    If,
    Else,
    For,
    While,
    Const,
    Let,
    Class,
    True,
    False,
    Null,
    Newline,
    Async,
    Use,
    All,
    New,
    Template(String),
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub kind: String,
    pub name: Option<String>,
    pub props: HashMap<String, String>,
    pub body: Vec<AstNode>,
}

impl AstNode {
    pub fn new(kind: &str) -> Self {
        Self {
            kind: kind.to_string(),
            name: None,
            props: HashMap::new(),
            body: Vec::new(),
        }
    }
    pub fn with_name(mut self, n: &str) -> Self {
        self.name = Some(n.to_string());
        self
    }
    pub fn with_prop(mut self, k: &str, v: &str) -> Self {
        self.props.insert(k.to_string(), v.to_string());
        self
    }
    pub fn with_body(mut self, b: Vec<AstNode>) -> Self {
        self.body = b;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub nodes: Vec<AstNode>,
}
