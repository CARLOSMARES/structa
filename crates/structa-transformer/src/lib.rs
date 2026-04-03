use std::collections::HashMap;
use structa_ast::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("Transformation error: {0}")]
    Message(String),
}

pub type Result<T> = std::result::Result<T, TransformError>;

pub struct Transformer {
    options: TransformOptions,
}

#[derive(Debug, Clone)]
pub struct TransformOptions {
    pub target: String,
    pub module: String,
    pub strict_mode: bool,
    pub decorators_enabled: bool,
}

impl Default for TransformOptions {
    fn default() -> Self {
        Self {
            target: "es2022".to_string(),
            module: "commonjs".to_string(),
            strict_mode: true,
            decorators_enabled: true,
        }
    }
}

impl Transformer {
    pub fn new(options: TransformOptions) -> Self {
        Self { options }
    }

    pub fn transform(&mut self, program: Program) -> Result<TransformedProgram> {
        let mut statements = Vec::new();
        let mut metadata = ProgramMetadata::default();

        for node in program.statements {
            let transformed = self.transform_node(node)?;
            statements.extend(transformed);
        }

        Ok(TransformedProgram {
            statements,
            metadata,
            options: self.options.clone(),
        })
    }

    fn transform_node(&mut self, node: Node) -> Result<Vec<TransformedStmt>> {
        match node {
            Node::Controller(c) => self.transform_controller(c),
            Node::Service(s) => self.transform_service(s),
            Node::Module(m) => self.transform_module(m),
            Node::Dto(d) => self.transform_dto(d),
            Node::Guard(g) => self.transform_guard(g),
            Node::Middleware(m) => self.transform_middleware(m),
            Node::Resolver(r) => self.transform_resolver(r),
            Node::Gateway(g) => self.transform_gateway(g),
            Node::Interface(i) => self.transform_interface(i),
            Node::Enum(e) => self.transform_enum(e),
            Node::TypeAlias(t) => self.transform_type_alias(t),
            Node::Import(imp) => self.transform_import(imp),
            Node::ExprStmt(expr) => self.transform_expression(expr),
            _ => Ok(vec![]),
        }
    }

    fn transform_controller(&mut self, ctrl: ControllerDecl) -> Result<Vec<TransformedStmt>> {
        let mut body = Vec::new();

        for route in &ctrl.routes {
            let method = format!("{:?}", route.method).to_uppercase();
            let route_path = &route.path;
            let handler = &route.handler.name;

            body.push(format!(
                "  @{}(\"{}\")\n  async {}(ctx) {{\n    return Response.json({{}});\n  }}",
                http_method_to_decorator(&route.method),
                route_path,
                handler
            ));
        }

        let path_str = ctrl.path.as_deref().unwrap_or("\"");
        body.push(format!("  static path = {};\n", path_str));

        Ok(vec![TransformedStmt::Class {
            name: ctrl.name,
            extends: Some("Controller".to_string()),
            implements: vec![],
            body,
            decorators: ctrl.decorators,
        }])
    }

    fn transform_service(&mut self, svc: ServiceDecl) -> Result<Vec<TransformedStmt>> {
        let mut body = Vec::new();

        for method in &svc.methods {
            let async_prefix = if method.is_async { "async " } else { "" };
            let params_str = method
                .params
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            body.push(format!(
                "  {}{}({}) {{\n    // TODO: implement\n  }}",
                async_prefix, method.name, params_str
            ));
        }

        Ok(vec![TransformedStmt::Class {
            name: svc.name,
            extends: Some("Service".to_string()),
            implements: vec![],
            body,
            decorators: svc.decorators,
        }])
    }

    fn transform_module(&mut self, module: ModuleDecl) -> Result<Vec<TransformedStmt>> {
        let mut imports = Vec::new();

        for import_name in &module.imports {
            imports.push(TransformedImport {
                path: import_name.clone(),
                names: vec![],
                is_default: false,
            });
        }

        Ok(vec![TransformedStmt::Module {
            name: module.name,
            imports,
            exports: module.exports,
            body: vec![],
        }])
    }

    fn transform_dto(&mut self, dto: DtoDecl) -> Result<Vec<TransformedStmt>> {
        let mut body = Vec::new();

        for field in &dto.fields {
            let optional = if field.optional { "?" } else { "" };
            let field_type = self.type_to_string(&field.field_type);
            body.push(TransformedField {
                name: field.name.clone(),
                field_type_str: format!("{}{}: {}", field.name, optional, field_type),
                optional: field.optional,
                decorators: field.decorators.clone(),
            });
        }

        Ok(vec![TransformedStmt::Interface {
            name: dto.name,
            extends: vec![],
            body,
            decorators: dto.decorators,
        }])
    }

    fn transform_guard(&mut self, guard: GuardDecl) -> Result<Vec<TransformedStmt>> {
        let mut body = Vec::new();

        for method in &guard.methods {
            let params_str = method
                .params
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            body.push(format!(
                "  async {}({}) {{\n    return true;\n  }}",
                method.name, params_str
            ));
        }

        Ok(vec![TransformedStmt::Class {
            name: guard.name,
            extends: Some("Guard".to_string()),
            implements: vec![],
            body,
            decorators: guard.decorators,
        }])
    }

    fn transform_middleware(&mut self, middleware: MiddlewareDecl) -> Result<Vec<TransformedStmt>> {
        let mut body = Vec::new();

        for method in &middleware.methods {
            let async_prefix = if method.is_async { "async " } else { "" };
            let params_str = method
                .params
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            body.push(format!(
                "  {}{}({}) {{\n    return ctx;\n  }}",
                async_prefix, method.name, params_str
            ));
        }

        Ok(vec![TransformedStmt::Class {
            name: middleware.name,
            extends: Some("Middleware".to_string()),
            implements: vec![],
            body,
            decorators: middleware.decorators,
        }])
    }

    fn transform_resolver(&mut self, resolver: ResolverDecl) -> Result<Vec<TransformedStmt>> {
        let mut body = Vec::new();

        for field in &resolver.fields {
            let query_type = match field.query_type {
                QueryType::Query => "@Query",
                QueryType::Mutation => "@Mutation",
                QueryType::Subscription => "@Subscription",
            };
            let params_str = field
                .args
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            body.push(format!(
                "  {}(\"{}\")\n  {}({}) {{\n    return null;\n  }}",
                query_type, field.name, field.name, params_str
            ));
        }

        Ok(vec![TransformedStmt::Class {
            name: resolver.name,
            extends: Some("Resolver".to_string()),
            implements: vec![],
            body,
            decorators: resolver.decorators,
        }])
    }

    fn transform_gateway(&mut self, gateway: GatewayDecl) -> Result<Vec<TransformedStmt>> {
        let mut body = Vec::new();
        body.push(format!("  static namespace = \"{}\";", gateway.namespace));

        for event in &gateway.events {
            let params_str = event
                .params
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            body.push(format!(
                "  {}({}) {{\n    // TODO: implement\n  }}",
                event.name, params_str
            ));
        }

        Ok(vec![TransformedStmt::Class {
            name: gateway.name,
            extends: Some("Gateway".to_string()),
            implements: vec![],
            body,
            decorators: gateway.decorators,
        }])
    }

    fn transform_interface(&mut self, interface: InterfaceDecl) -> Result<Vec<TransformedStmt>> {
        let mut body = Vec::new();

        for member in &interface.members {
            let optional = if member.optional { "?" } else { "" };
            let member_type = member
                .param_type
                .as_ref()
                .map(|t| self.type_to_string(t))
                .unwrap_or_else(|| "any".to_string());

            body.push(TransformedField {
                name: member.name.clone(),
                field_type_str: format!("{}{}: {}", member.name, optional, member_type),
                optional: member.optional,
                decorators: vec![],
            });
        }

        Ok(vec![TransformedStmt::Interface {
            name: interface.name,
            extends: interface.extends,
            body,
            decorators: interface.decorators,
        }])
    }

    fn transform_enum(&mut self, enm: EnumDecl) -> Result<Vec<TransformedStmt>> {
        let mut members = Vec::new();

        for member in &enm.members {
            let value = member
                .value
                .as_ref()
                .map(|v| format!(" = {}", self.expr_to_string(v)))
                .unwrap_or_default();
            members.push(format!("{}{}", member.name, value));
        }

        Ok(vec![TransformedStmt::Enum {
            name: enm.name,
            members,
            is_const: false,
            decorators: enm.decorators,
        }])
    }

    fn transform_type_alias(&mut self, alias: TypeAliasDecl) -> Result<Vec<TransformedStmt>> {
        Ok(vec![TransformedStmt::TypeAlias {
            name: alias.name,
            type_params: alias.type_params,
            type_annotation: alias.type_annotation,
            decorators: alias.decorators,
        }])
    }

    fn transform_import(&mut self, import: ImportDecl) -> Result<Vec<TransformedStmt>> {
        Ok(vec![TransformedStmt::Import {
            path: import.path,
            names: import.names.into_iter().map(|n| n.name).collect(),
            is_default: import.is_default,
        }])
    }

    fn transform_expression(&mut self, expr: Expr) -> Result<Vec<TransformedStmt>> {
        Ok(vec![TransformedStmt::Expr {
            expr_str: self.expr_to_string(&expr),
        }])
    }

    fn type_to_string(&self, type_annotation: &TypeAnnotation) -> String {
        match &type_annotation.kind {
            TypeKind::String => "string".to_string(),
            TypeKind::Number | TypeKind::Integer | TypeKind::Float => "number".to_string(),
            TypeKind::Boolean => "boolean".to_string(),
            TypeKind::Void => "void".to_string(),
            TypeKind::Null => "null".to_string(),
            TypeKind::Undefined => "undefined".to_string(),
            TypeKind::Any | TypeKind::Unknown => "any".to_string(),
            TypeKind::Never => "never".to_string(),
            TypeKind::Object => "object".to_string(),
            TypeKind::Array(inner) => format!("{}[]", self.type_to_string(inner)),
            TypeKind::Optional(inner) => format!("{} | null", self.type_to_string(inner)),
            TypeKind::Union(types) => types
                .iter()
                .map(|t| self.type_to_string(t))
                .collect::<Vec<_>>()
                .join(" | "),
            TypeKind::Identifier(name) => name.clone(),
            TypeKind::Generic { name, args } => {
                let args_str = args
                    .iter()
                    .map(|a| self.type_to_string(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}<{}>", name, args_str)
            }
            TypeKind::Fn {
                params,
                return_type,
            } => {
                let params_str = params
                    .iter()
                    .map(|p| self.type_to_string(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({}) => {}", params_str, self.type_to_string(return_type))
            }
        }
    }

    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::String(s, _) => format!("\"{}\"", s),
            Expr::Number(n, _) => n.to_string(),
            Expr::Integer(n, _) => n.to_string(),
            Expr::Boolean(b, _) => b.to_string(),
            Expr::Identifier(s, _) => s.clone(),
            Expr::Null(_) => "null".to_string(),
            Expr::Undefined(_) => "undefined".to_string(),
            Expr::Binary {
                left, op, right, ..
            } => {
                let left_str = self.expr_to_string(left);
                let right_str = self.expr_to_string(right);
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Subtract => "-",
                    BinaryOp::Multiply => "*",
                    BinaryOp::Divide => "/",
                    BinaryOp::Equals => "===",
                    BinaryOp::NotEquals => "!==",
                    BinaryOp::LessThan => "<",
                    BinaryOp::GreaterThan => ">",
                    BinaryOp::LessThanOrEqual => "<=",
                    BinaryOp::GreaterThanOrEqual => ">=",
                    BinaryOp::And => "&&",
                    BinaryOp::Or => "||",
                    _ => "/* unknown */",
                };
                format!("({} {} {})", left_str, op_str, right_str)
            }
            Expr::Unary { op, operand, .. } => {
                let operand_str = self.expr_to_string(operand);
                let op_str = match op {
                    UnaryOp::Not => "!",
                    UnaryOp::Negate => "-",
                    UnaryOp::Plus => "+",
                    _ => "",
                };
                format!("({}{})", op_str, operand_str)
            }
            Expr::Call { callee, args, .. } => {
                let callee_str = self.expr_to_string(callee);
                let args_str = args
                    .iter()
                    .map(|a| self.expr_to_string(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", callee_str, args_str)
            }
            Expr::Member {
                object,
                property,
                computed,
                ..
            } => {
                let obj_str = self.expr_to_string(object);
                let prop_str = self.expr_to_string(property);
                if *computed {
                    format!("{}[{}]", obj_str, prop_str)
                } else {
                    format!("{}.{}", obj_str, prop_str)
                }
            }
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                format!(
                    "{} ? {} : {}",
                    self.expr_to_string(condition),
                    self.expr_to_string(then_expr),
                    self.expr_to_string(else_expr)
                )
            }
            _ => "/* unhandled */".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransformedProgram {
    pub statements: Vec<TransformedStmt>,
    pub metadata: ProgramMetadata,
    pub options: TransformOptions,
}

#[derive(Debug, Clone, Default)]
pub struct ProgramMetadata {
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub location: Option<Location>,
}

#[derive(Debug, Clone)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub enum TransformedStmt {
    Class {
        name: String,
        extends: Option<String>,
        implements: Vec<String>,
        body: Vec<String>,
        decorators: Vec<Decorator>,
    },
    Module {
        name: String,
        imports: Vec<TransformedImport>,
        exports: Vec<String>,
        body: Vec<TransformedStmt>,
    },
    Interface {
        name: String,
        extends: Vec<String>,
        body: Vec<TransformedField>,
        decorators: Vec<Decorator>,
    },
    Enum {
        name: String,
        members: Vec<String>,
        is_const: bool,
        decorators: Vec<Decorator>,
    },
    TypeAlias {
        name: String,
        type_params: Vec<String>,
        type_annotation: TypeAnnotation,
        decorators: Vec<Decorator>,
    },
    Import {
        path: String,
        names: Vec<String>,
        is_default: bool,
    },
    Expr {
        expr_str: String,
    },
}

#[derive(Debug, Clone)]
pub struct TransformedImport {
    pub path: String,
    pub names: Vec<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone)]
pub struct TransformedField {
    pub name: String,
    pub field_type_str: String,
    pub optional: bool,
    pub decorators: Vec<Decorator>,
}

fn http_method_to_decorator(method: &HttpMethod) -> &'static str {
    match method {
        HttpMethod::Get => "@Get",
        HttpMethod::Post => "@Post",
        HttpMethod::Put => "@Put",
        HttpMethod::Patch => "@Patch",
        HttpMethod::Delete => "@Delete",
        HttpMethod::Head => "@Head",
        HttpMethod::Options => "@Options",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformer_creation() {
        let options = TransformOptions::default();
        let transformer = Transformer::new(options);
        assert_eq!(transformer.options.target, "es2022");
    }

    #[test]
    fn test_http_method_decorator() {
        assert_eq!(http_method_to_decorator(&HttpMethod::Get), "@Get");
        assert_eq!(http_method_to_decorator(&HttpMethod::Post), "@Post");
    }
}
