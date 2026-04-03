use structa_ast::*;
use structa_lexer::{Lexer, Token, TokenKind};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token at line {line}: {found:?}")]
    UnexpectedToken { found: TokenKind, line: usize },

    #[error("Expected {expected}, found {found:?}")]
    ExpectedToken { expected: String, found: TokenKind },

    #[error("Parse error: {0}")]
    Message(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap_or_default();
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            match self.parse_statement()? {
                Node::Import(import) => program.imports.push(import),
                other => program.statements.push(other),
            }
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Node> {
        self.skip_whitespace();

        let _decorators = self.parse_decorators()?;

        match &self.current().kind {
            TokenKind::Keyword(k) if self.is_module_keyword(k) => {
                let node = self.parse_declaration()?;
                Ok(node)
            }
            _ => {
                let expr = self.parse_expression()?;
                if self.check(&TokenKind::Semicolon) {
                    self.advance();
                }
                Ok(Node::ExprStmt(expr))
            }
        }
    }

    fn is_module_keyword(&self, k: &structa_lexer::Keyword) -> bool {
        matches!(
            k,
            structa_lexer::Keyword::Module
                | structa_lexer::Keyword::Controller
                | structa_lexer::Keyword::Service
                | structa_lexer::Keyword::Dto
                | structa_lexer::Keyword::Guard
                | structa_lexer::Keyword::Middleware
                | structa_lexer::Keyword::Resolver
                | structa_lexer::Keyword::Gateway
                | structa_lexer::Keyword::Interface
                | structa_lexer::Keyword::Enum
        )
    }

    fn parse_declaration(&mut self) -> Result<Node> {
        let token = self.current().clone();

        match &token.kind {
            TokenKind::Keyword(k) => match k {
                structa_lexer::Keyword::Module => Ok(Node::Module(self.parse_module()?)),
                structa_lexer::Keyword::Controller => {
                    Ok(Node::Controller(self.parse_controller()?))
                }
                structa_lexer::Keyword::Service => Ok(Node::Service(self.parse_service()?)),
                structa_lexer::Keyword::Dto => Ok(Node::Dto(self.parse_dto()?)),
                structa_lexer::Keyword::Guard => Ok(Node::Guard(self.parse_guard()?)),
                structa_lexer::Keyword::Middleware => {
                    Ok(Node::Middleware(self.parse_middleware()?))
                }
                structa_lexer::Keyword::Resolver => Ok(Node::Resolver(self.parse_resolver()?)),
                structa_lexer::Keyword::Gateway => Ok(Node::Gateway(self.parse_gateway()?)),
                structa_lexer::Keyword::Interface => Ok(Node::Interface(self.parse_interface()?)),
                structa_lexer::Keyword::Enum => Ok(Node::Enum(self.parse_enum()?)),
                _ => Err(ParseError::UnexpectedToken {
                    found: token.kind,
                    line: token.line,
                }),
            },
            _ => Err(ParseError::UnexpectedToken {
                found: token.kind,
                line: token.line,
            }),
        }
    }

    fn parse_module(&mut self) -> Result<ModuleDecl> {
        self.advance();
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;

        let mut imports = Vec::new();
        let exports = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }

            if self.check_keyword(structa_lexer::Keyword::Import) {
                self.advance();
                self.expect(TokenKind::LeftBracket)?;
                while !self.check(&TokenKind::RightBracket) && !self.is_at_end() {
                    imports.push(self.expect_identifier()?);
                    if !self.check(&TokenKind::RightBracket) {
                        self.expect(TokenKind::Comma)?;
                    }
                }
                self.expect(TokenKind::RightBracket)?;
                self.expect(TokenKind::Semicolon)?;
            } else {
                break;
            }
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(ModuleDecl {
            name,
            imports,
            exports,
            location: Location::default(),
        })
    }

    fn parse_controller(&mut self) -> Result<ControllerDecl> {
        self.advance();
        let name = self.expect_identifier()?;
        self.skip_whitespace();

        let mut path = None;
        if self.check(&TokenKind::Colon) {
            self.advance();
            if let TokenKind::String(s) = &self.current().kind {
                path = Some(s.clone());
                self.advance();
            }
        }

        self.expect(TokenKind::LeftBrace)?;
        let mut routes = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            routes.push(self.parse_route()?);
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(ControllerDecl {
            name,
            path,
            routes,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_route(&mut self) -> Result<Route> {
        let decorators = self.parse_decorators()?;

        let method = self.parse_http_method();

        let path = if let TokenKind::String(s) = &self.current().kind {
            let s = s.clone();
            self.advance();
            s
        } else {
            "/".to_string()
        };

        let handler = self.parse_route_handler()?;

        Ok(Route {
            method,
            path,
            handler,
            guards: Vec::new(),
            middleware: Vec::new(),
            decorators,
            location: Location::default(),
        })
    }

    fn parse_http_method(&mut self) -> HttpMethod {
        if self.check_keyword(structa_lexer::Keyword::Get) {
            self.advance();
            HttpMethod::Get
        } else if self.check_keyword(structa_lexer::Keyword::Post) {
            self.advance();
            HttpMethod::Post
        } else if self.check_keyword(structa_lexer::Keyword::Put) {
            self.advance();
            HttpMethod::Put
        } else if self.check_keyword(structa_lexer::Keyword::Delete) {
            self.advance();
            HttpMethod::Delete
        } else {
            HttpMethod::Get
        }
    }

    fn parse_route_handler(&mut self) -> Result<RouteHandler> {
        let name = self.expect_identifier()?;
        let params = self.parse_params()?;
        let body = self.parse_block()?;

        Ok(RouteHandler {
            name,
            params,
            body,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_service(&mut self) -> Result<ServiceDecl> {
        self.advance();
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut methods = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            methods.push(self.parse_service_method()?);
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(ServiceDecl {
            name,
            methods,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_service_method(&mut self) -> Result<ServiceMethod> {
        let is_async = self.check_keyword(structa_lexer::Keyword::Async);
        if is_async {
            self.advance();
        }

        let name = self.expect_identifier()?;
        let params = self.parse_params()?;

        let return_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(ServiceMethod {
            name,
            params,
            return_type,
            body,
            decorators: Vec::new(),
            is_async,
            location: Location::default(),
        })
    }

    fn parse_dto(&mut self) -> Result<DtoDecl> {
        self.advance();
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut fields = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            fields.push(self.parse_dto_field()?);
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(DtoDecl {
            name,
            fields,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_dto_field(&mut self) -> Result<DtoField> {
        self.parse_decorators()?;
        let name = self.expect_identifier()?;
        let field_type = self.parse_type()?;
        let optional = matches!(&field_type.kind, TypeKind::Optional(_));
        self.expect(TokenKind::Semicolon)?;

        Ok(DtoField {
            name,
            field_type,
            optional,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_guard(&mut self) -> Result<GuardDecl> {
        self.advance();
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut methods = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            methods.push(self.parse_guard_method()?);
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(GuardDecl {
            name,
            methods,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_guard_method(&mut self) -> Result<GuardMethod> {
        let name = self.expect_identifier()?;
        let params = self.parse_params()?;
        let return_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = self.parse_block()?;

        Ok(GuardMethod {
            name,
            params,
            return_type,
            body,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_middleware(&mut self) -> Result<MiddlewareDecl> {
        self.advance();
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut methods = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            methods.push(self.parse_middleware_method()?);
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(MiddlewareDecl {
            name,
            methods,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_middleware_method(&mut self) -> Result<MiddlewareMethod> {
        let is_async = self.check_keyword(structa_lexer::Keyword::Async);
        if is_async {
            self.advance();
        }

        let name = self.expect_identifier()?;
        let params = self.parse_params()?;
        let body = self.parse_block()?;

        Ok(MiddlewareMethod {
            name,
            params,
            body,
            decorators: Vec::new(),
            is_async,
            location: Location::default(),
        })
    }

    fn parse_resolver(&mut self) -> Result<ResolverDecl> {
        self.advance();
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut fields = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            fields.push(self.parse_resolver_field()?);
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(ResolverDecl {
            name,
            fields,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_resolver_field(&mut self) -> Result<ResolverField> {
        let decorators = self.parse_decorators()?;

        let query_type = if self.check_keyword(structa_lexer::Keyword::Query) {
            self.advance();
            QueryType::Query
        } else if self.check_keyword(structa_lexer::Keyword::Mutation) {
            self.advance();
            QueryType::Mutation
        } else {
            QueryType::Query
        };

        let name = self.expect_identifier()?;
        let args = self.parse_params()?;
        let return_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = self.parse_block()?;

        Ok(ResolverField {
            query_type,
            name,
            args,
            return_type,
            body,
            decorators,
            location: Location::default(),
        })
    }

    fn parse_gateway(&mut self) -> Result<GatewayDecl> {
        self.advance();
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;

        let mut namespace = "/".to_string();
        let mut events = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }

            if let TokenKind::Identifier(s) = &self.current().kind {
                if s == "namespace" {
                    self.advance();
                    self.expect(TokenKind::Colon)?;
                    if let TokenKind::String(ns) = &self.current().kind {
                        namespace = ns.clone();
                        self.advance();
                    }
                    self.expect(TokenKind::Semicolon)?;
                } else {
                    events.push(self.parse_gateway_event()?);
                }
            }
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(GatewayDecl {
            name,
            namespace,
            events,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_gateway_event(&mut self) -> Result<GatewayEvent> {
        let decorators = self.parse_decorators()?;
        let name = self.expect_identifier()?;
        let params = self.parse_params()?;
        let return_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = self.parse_block()?;

        Ok(GatewayEvent {
            name,
            params,
            return_type,
            body,
            decorators,
            location: Location::default(),
        })
    }

    fn parse_interface(&mut self) -> Result<InterfaceDecl> {
        self.advance();
        let name = self.expect_identifier()?;

        let mut extends = Vec::new();
        if self.check_keyword(structa_lexer::Keyword::Extends) {
            self.advance();
            loop {
                extends.push(self.expect_identifier()?);
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance();
            }
        }

        self.expect(TokenKind::LeftBrace)?;
        let mut members = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }

            let name = self.expect_identifier()?;
            let optional = self.check(&TokenKind::QuestionMark);
            if optional {
                self.advance();
            }

            let param_type = if self.check(&TokenKind::Colon) {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };
            self.expect(TokenKind::Semicolon)?;

            members.push(InterfaceMember {
                name,
                param_type,
                return_type: None,
                optional,
                location: Location::default(),
            });
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(InterfaceDecl {
            name,
            extends,
            members,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_enum(&mut self) -> Result<EnumDecl> {
        self.advance();
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut members = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }

            let name = self.expect_identifier()?;
            let value = if self.check(&TokenKind::Equals) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };

            members.push(EnumMember {
                name,
                value,
                location: Location::default(),
            });

            if !self.check(&TokenKind::RightBrace) {
                self.expect(TokenKind::Comma)?;
            }
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;

        Ok(EnumDecl {
            name,
            members,
            decorators: Vec::new(),
            location: Location::default(),
        })
    }

    fn parse_decorators(&mut self) -> Result<Vec<Decorator>> {
        let mut decorators = Vec::new();

        while self.check(&TokenKind::At) {
            self.advance();
            let name = self.expect_identifier()?;
            let args = if self.check(&TokenKind::LeftParen) {
                self.advance();
                let mut args = Vec::new();
                while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
                    args.push(self.parse_expression()?);
                    if !self.check(&TokenKind::RightParen) {
                        self.expect(TokenKind::Comma)?;
                    }
                }
                self.expect(TokenKind::RightParen)?;
                args
            } else {
                Vec::new()
            };

            decorators.push(Decorator {
                name,
                args,
                location: Location::default(),
            });
        }

        Ok(decorators)
    }

    fn parse_params(&mut self) -> Result<Vec<Param>> {
        let mut params = Vec::new();
        self.expect(TokenKind::LeftParen)?;

        while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
            let decorators = self.parse_decorators()?;
            let name = self.expect_identifier()?;
            let param_type = if self.check(&TokenKind::Colon) {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            params.push(Param {
                name,
                param_type,
                decorators,
                location: Location::default(),
            });

            if !self.check(&TokenKind::RightParen) {
                self.expect(TokenKind::Comma)?;
            }
        }

        self.expect(TokenKind::RightParen)?;
        Ok(params)
    }

    fn parse_type(&mut self) -> Result<TypeAnnotation> {
        let kind = match &self.current().kind {
            TokenKind::Identifier(s) => {
                let s = s.clone();
                self.advance();

                if self.check(&TokenKind::LessThan) {
                    self.advance();
                    let mut args = Vec::new();
                    while !self.check(&TokenKind::GreaterThan) && !self.is_at_end() {
                        args.push(self.parse_type()?);
                        if !self.check(&TokenKind::GreaterThan) {
                            self.expect(TokenKind::Comma)?;
                        }
                    }
                    self.expect(TokenKind::GreaterThan)?;
                    TypeKind::Generic { name: s, args }
                } else {
                    match s.as_str() {
                        "string" => TypeKind::String,
                        "number" => TypeKind::Number,
                        "boolean" => TypeKind::Boolean,
                        "void" => TypeKind::Void,
                        "null" => TypeKind::Null,
                        "any" => TypeKind::Any,
                        "never" => TypeKind::Never,
                        "unknown" => TypeKind::Unknown,
                        "object" => TypeKind::Object,
                        _ => TypeKind::Identifier(s),
                    }
                }
            }
            _ => TypeKind::Any,
        };

        if self.check(&TokenKind::QuestionMark) {
            self.advance();
            return Ok(TypeAnnotation {
                kind: TypeKind::Optional(Box::new(TypeAnnotation {
                    kind,
                    location: Location::default(),
                })),
                location: Location::default(),
            });
        }

        Ok(TypeAnnotation {
            kind,
            location: Location::default(),
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        if !self.check(&TokenKind::LeftBrace) {
            return Ok(vec![Stmt::Return(Some(self.parse_expression()?))]);
        }

        self.advance();
        let mut stmts = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_whitespace();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            stmts.push(Stmt::Expr(self.parse_expression()?));
            self.skip_whitespace();
        }

        self.expect(TokenKind::RightBrace)?;
        Ok(stmts)
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Result<Expr> {
        let mut left = self.parse_multiplication()?;

        loop {
            let op = match &self.current().kind {
                TokenKind::Plus => {
                    self.advance();
                    Some(BinaryOp::Add)
                }
                TokenKind::Minus => {
                    self.advance();
                    Some(BinaryOp::Subtract)
                }
                _ => None,
            };

            if let Some(op) = op {
                let right = self.parse_multiplication()?;
                left = Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    location: Location::default(),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr> {
        let mut left = self.parse_unary()?;

        loop {
            let op = match &self.current().kind {
                TokenKind::Star => {
                    self.advance();
                    Some(BinaryOp::Multiply)
                }
                TokenKind::Slash => {
                    self.advance();
                    Some(BinaryOp::Divide)
                }
                _ => None,
            };

            if let Some(op) = op {
                let right = self.parse_unary()?;
                left = Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    location: Location::default(),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        if self.check(&TokenKind::Minus) {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Negate,
                operand: Box::new(operand),
                location: Location::default(),
            });
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.check(&TokenKind::LeftParen) {
                self.advance();
                let mut args = Vec::new();
                while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
                    args.push(self.parse_expression()?);
                    if !self.check(&TokenKind::RightParen) {
                        self.expect(TokenKind::Comma)?;
                    }
                }
                self.expect(TokenKind::RightParen)?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                    location: Location::default(),
                };
            } else if self.check(&TokenKind::Dot) {
                self.advance();
                let property = self.parse_primary()?;
                expr = Expr::Member {
                    object: Box::new(expr),
                    property: Box::new(property),
                    computed: false,
                    location: Location::default(),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        let token = self.current().clone();

        match &token.kind {
            TokenKind::Integer(n) => {
                self.advance();
                Ok(Expr::Integer(*n, Location::default()))
            }
            TokenKind::Number(n) => {
                self.advance();
                Ok(Expr::Number(*n, Location::default()))
            }
            TokenKind::String(s) => {
                self.advance();
                Ok(Expr::String(s.clone(), Location::default()))
            }
            TokenKind::Boolean(b) => {
                self.advance();
                Ok(Expr::Boolean(*b, Location::default()))
            }
            TokenKind::Identifier(s) => {
                self.advance();
                Ok(Expr::Identifier(s.clone(), Location::default()))
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::UnexpectedToken {
                found: token.kind,
                line: token.line,
            }),
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(
            &self.current().kind,
            TokenKind::Newline | TokenKind::Comment(_)
        ) {
            self.advance();
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or_else(|| {
            self.tokens
                .last()
                .unwrap_or_else(|| panic!("No tokens available"))
        })
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len() || matches!(&self.current().kind, TokenKind::Eof)
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.current().kind) == std::mem::discriminant(kind)
    }

    fn check_keyword(&self, keyword: structa_lexer::Keyword) -> bool {
        matches!(&self.current().kind, TokenKind::Keyword(k) if *k == keyword)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.current()
    }

    fn expect(&mut self, kind: TokenKind) -> Result<()> {
        if self.check(&kind) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::ExpectedToken {
                expected: format!("{:?}", kind),
                found: self.current().kind.clone(),
            })
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        if let TokenKind::Identifier(s) = &self.current().kind {
            let s = s.clone();
            self.advance();
            Ok(s)
        } else {
            Err(ParseError::ExpectedToken {
                expected: "identifier".to_string(),
                found: self.current().kind.clone(),
            })
        }
    }
}
