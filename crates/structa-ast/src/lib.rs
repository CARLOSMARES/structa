use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Program(Program),
    Module(ModuleDecl),
    Controller(ControllerDecl),
    Service(ServiceDecl),
    Guard(GuardDecl),
    Middleware(MiddlewareDecl),
    Resolver(ResolverDecl),
    Gateway(GatewayDecl),
    Dto(DtoDecl),
    Interface(InterfaceDecl),
    Enum(EnumDecl),
    Import(ImportDecl),
    TypeAlias(TypeAliasDecl),
    ExprStmt(Expr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Node>,
    pub imports: Vec<ImportDecl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDecl {
    pub name: String,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerDecl {
    pub name: String,
    pub path: Option<String>,
    pub routes: Vec<Route>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDecl {
    pub name: String,
    pub methods: Vec<ServiceMethod>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Vec<Stmt>,
    pub decorators: Vec<Decorator>,
    pub is_async: bool,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: RouteHandler,
    pub guards: Vec<String>,
    pub middleware: Vec<String>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteHandler {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardDecl {
    pub name: String,
    pub methods: Vec<GuardMethod>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Vec<Stmt>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareDecl {
    pub name: String,
    pub methods: Vec<MiddlewareMethod>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub decorators: Vec<Decorator>,
    pub is_async: bool,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverDecl {
    pub name: String,
    pub fields: Vec<ResolverField>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverField {
    pub query_type: QueryType,
    pub name: String,
    pub args: Vec<Param>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Vec<Stmt>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    Query,
    Mutation,
    Subscription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayDecl {
    pub name: String,
    pub namespace: String,
    pub events: Vec<GatewayEvent>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayEvent {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Vec<Stmt>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtoDecl {
    pub name: String,
    pub fields: Vec<DtoField>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtoField {
    pub name: String,
    pub field_type: TypeAnnotation,
    pub optional: bool,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceDecl {
    pub name: String,
    pub extends: Vec<String>,
    pub members: Vec<InterfaceMember>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceMember {
    pub name: String,
    pub param_type: Option<TypeAnnotation>,
    pub return_type: Option<TypeAnnotation>,
    pub optional: bool,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDecl {
    pub name: String,
    pub members: Vec<EnumMember>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumMember {
    pub name: String,
    pub value: Option<Expr>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDecl {
    pub path: String,
    pub names: Vec<ImportName>,
    pub is_default: bool,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportName {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAliasDecl {
    pub name: String,
    pub type_params: Vec<String>,
    pub type_annotation: TypeAnnotation,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decorator {
    pub name: String,
    pub args: Vec<Expr>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub param_type: Option<TypeAnnotation>,
    pub decorators: Vec<Decorator>,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAnnotation {
    pub kind: TypeKind,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeKind {
    String,
    Number,
    Boolean,
    Void,
    Null,
    Undefined,
    Any,
    Never,
    Unknown,
    Integer,
    Float,
    Object,
    Array(Box<TypeAnnotation>),
    Optional(Box<TypeAnnotation>),
    Union(Vec<TypeAnnotation>),
    Identifier(String),
    Generic {
        name: String,
        args: Vec<TypeAnnotation>,
    },
    Fn {
        params: Vec<TypeAnnotation>,
        return_type: Box<TypeAnnotation>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Expr(Expr),
    Return(Option<Expr>),
    VarDecl {
        name: String,
        var_type: Option<TypeAnnotation>,
        value: Option<Expr>,
        location: Location,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        location: Location,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        location: Location,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Box<Expr>>,
        body: Vec<Stmt>,
        location: Location,
    },
    Break(Location),
    Continue(Location),
    Throw(Expr, Location),
    Block(Vec<Stmt>, Location),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Null(Location),
    Undefined(Location),
    Boolean(bool, Location),
    Integer(i64, Location),
    Number(f64, Location),
    String(String, Location),
    Identifier(String, Location),
    Array(Vec<Expr>, Location),
    Object(Vec<ObjectProperty>, Location),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        location: Location,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
        location: Location,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        location: Location,
    },
    Member {
        object: Box<Expr>,
        property: Box<Expr>,
        computed: bool,
        location: Location,
    },
    Conditional {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
        location: Location,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectProperty {
    pub key: Expr,
    pub value: Expr,
    pub shorthand: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    StrictEquals,
    StrictNotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    In,
    InstanceOf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Negate,
    Plus,
    TypeOf,
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            start: Position::default(),
            end: Position::default(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            imports: Vec::new(),
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
