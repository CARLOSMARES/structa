use crate::ast::{AstNode, Program, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Program {
        let mut prog = Program::default();
        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            if let Some(node) = self.parse_stmt() {
                prog.nodes.push(node);
            }
        }
        prog
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len() || matches!(self.peek(), Token::Eof)
    }

    fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            &Token::Eof
        }
    }

    fn peek_next(&self) -> Option<&Token> {
        if self.pos + 1 < self.tokens.len() {
            Some(&self.tokens[self.pos + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) {
            self.advance();
        }
    }

    fn skip_semi(&mut self) {
        while matches!(self.peek(), Token::Semi) {
            self.advance();
        }
    }

    fn parse_stmt(&mut self) -> Option<AstNode> {
        match self.peek() {
            Token::Controller => Some(self.parse_controller()),
            Token::Service => Some(self.parse_service()),
            Token::Dto => Some(self.parse_dto()),
            Token::Model => Some(self.parse_dto()),
            Token::Entity => Some(self.parse_dto()),
            Token::Repository => Some(self.parse_repository()),
            Token::Resolver => Some(self.parse_resolver()),
            Token::Middleware => Some(self.parse_middleware()),
            Token::Module => Some(self.parse_module()),
            Token::At => self.parse_decorator(),
            Token::Newline => {
                self.advance();
                None
            }
            _ => {
                self.advance();
                None
            }
        }
    }

    fn parse_controller(&mut self) -> AstNode {
        self.advance();
        let name = self.parse_name();
        let mut node = AstNode::new("controller").with_name(&name);

        if matches!(self.peek(), Token::LBrace) {
            self.advance();
            self.skip_newlines();

            while !self.is_at_end() && !matches!(self.peek(), Token::RBrace) {
                self.skip_newlines();
                if matches!(self.peek(), Token::RBrace) {
                    break;
                }

                if matches!(self.peek(), Token::At) {
                    if let Some(decorator) = self.parse_decorator() {
                        node.body.push(decorator);
                    }
                } else if matches!(self.peek(), Token::Ident(_)) {
                    let method = self.parse_method_or_property();
                    if !method.name.as_ref().map_or(true, |n| n.is_empty()) {
                        node.body.push(method);
                    }
                } else {
                    self.advance();
                }

                self.skip_newlines();
            }

            if matches!(self.peek(), Token::RBrace) {
                self.advance();
            }
        }

        node
    }

    fn parse_service(&mut self) -> AstNode {
        self.advance();
        let name = self.parse_name();
        let mut node = AstNode::new("service").with_name(&name);

        if matches!(self.peek(), Token::LBrace) {
            self.advance();
            self.skip_newlines();

            while !self.is_at_end() && !matches!(self.peek(), Token::RBrace) {
                self.skip_newlines();
                if matches!(self.peek(), Token::RBrace) {
                    break;
                }

                if matches!(self.peek(), Token::At) {
                    if let Some(decorator) = self.parse_decorator() {
                        node.body.push(decorator);
                    }
                } else if matches!(self.peek(), Token::Ident(_)) {
                    let method = self.parse_method_or_property();
                    if !method.name.as_ref().map_or(true, |n| n.is_empty()) {
                        node.body.push(method);
                    }
                } else {
                    self.advance();
                }

                self.skip_newlines();
            }

            if matches!(self.peek(), Token::RBrace) {
                self.advance();
            }
        }

        node
    }

    fn parse_dto(&mut self) -> AstNode {
        self.advance();
        let name = self.parse_name();
        let mut node = AstNode::new("dto").with_name(&name);

        if matches!(self.peek(), Token::LBrace) {
            self.advance();
            self.skip_newlines();

            while !self.is_at_end() && !matches!(self.peek(), Token::RBrace) {
                self.skip_newlines();
                if matches!(self.peek(), Token::RBrace) {
                    break;
                }

                if matches!(self.peek(), Token::Ident(_)) {
                    let field = self.parse_field();
                    if !field.name.as_ref().map_or(true, |n| n.is_empty()) {
                        node.body.push(field);
                    }
                } else {
                    self.advance();
                }

                self.skip_newlines();
            }

            if matches!(self.peek(), Token::RBrace) {
                self.advance();
            }
        }

        node
    }

    fn parse_repository(&mut self) -> AstNode {
        self.advance();
        let name = self.parse_name();
        let mut node = AstNode::new("repository").with_name(&name);

        if matches!(self.peek(), Token::LBrace) {
            self.advance();
            self.skip_newlines();

            while !self.is_at_end() && !matches!(self.peek(), Token::RBrace) {
                self.skip_newlines();
                if matches!(self.peek(), Token::RBrace) {
                    break;
                }

                if matches!(self.peek(), Token::Ident(_)) {
                    let method = self.parse_method_or_property();
                    if !method.name.as_ref().map_or(true, |n| n.is_empty()) {
                        node.body.push(method);
                    }
                } else {
                    self.advance();
                }

                self.skip_newlines();
            }

            if matches!(self.peek(), Token::RBrace) {
                self.advance();
            }
        }

        node
    }

    fn parse_resolver(&mut self) -> AstNode {
        self.advance();
        let name = self.parse_name();
        let mut node = AstNode::new("resolver").with_name(&name);

        if matches!(self.peek(), Token::LBrace) {
            self.advance();
            self.skip_newlines();

            while !self.is_at_end() && !matches!(self.peek(), Token::RBrace) {
                self.skip_newlines();
                if matches!(self.peek(), Token::RBrace) {
                    break;
                }

                if matches!(self.peek(), Token::At) {
                    if let Some(decorator) = self.parse_decorator() {
                        node.body.push(decorator);
                    }
                } else if matches!(self.peek(), Token::Ident(_)) {
                    let method = self.parse_method_or_property();
                    if !method.name.as_ref().map_or(true, |n| n.is_empty()) {
                        node.body.push(method);
                    }
                } else {
                    self.advance();
                }

                self.skip_newlines();
            }

            if matches!(self.peek(), Token::RBrace) {
                self.advance();
            }
        }

        node
    }

    fn parse_middleware(&mut self) -> AstNode {
        self.advance();
        let name = self.parse_name();
        let mut node = AstNode::new("middleware").with_name(&name);

        if matches!(self.peek(), Token::LBrace) {
            self.advance();
            self.skip_newlines();

            while !self.is_at_end() && !matches!(self.peek(), Token::RBrace) {
                self.skip_newlines();
                if matches!(self.peek(), Token::RBrace) {
                    break;
                }

                if matches!(self.peek(), Token::Ident(_)) {
                    let method = self.parse_method_or_property();
                    if !method.name.as_ref().map_or(true, |n| n.is_empty()) {
                        node.body.push(method);
                    }
                } else {
                    self.advance();
                }

                self.skip_newlines();
            }

            if matches!(self.peek(), Token::RBrace) {
                self.advance();
            }
        }

        node
    }

    fn parse_module(&mut self) -> AstNode {
        self.advance();
        let name = self.parse_name();
        let mut node = AstNode::new("module").with_name(&name);

        if matches!(self.peek(), Token::LBrace) {
            self.advance();
            self.skip_newlines();

            while !self.is_at_end() && !matches!(self.peek(), Token::RBrace) {
                self.skip_newlines();
                if matches!(self.peek(), Token::RBrace) {
                    break;
                }

                if matches!(self.peek(), Token::At) {
                    if let Some(decorator) = self.parse_decorator() {
                        node.body.push(decorator);
                    }
                } else if matches!(self.peek(), Token::Ident(_)) {
                    let method = self.parse_method_or_property();
                    if !method.name.as_ref().map_or(true, |n| n.is_empty()) {
                        node.body.push(method);
                    }
                } else {
                    self.advance();
                }

                self.skip_newlines();
            }

            if matches!(self.peek(), Token::RBrace) {
                self.advance();
            }
        }

        node
    }

    fn parse_name(&mut self) -> String {
        if let Token::Ident(s) = self.peek().clone() {
            self.advance();
            s
        } else {
            "Unknown".to_string()
        }
    }

    fn parse_decorator(&mut self) -> Option<AstNode> {
        if !matches!(self.peek(), Token::At) {
            return None;
        }
        self.advance();

        let method = match self.peek() {
            Token::Get => "GET".to_string(),
            Token::Post => "POST".to_string(),
            Token::Put => "PUT".to_string(),
            Token::Delete => "DELETE".to_string(),
            Token::Patch => "PATCH".to_string(),
            Token::Use => "USE".to_string(),
            Token::All => "ALL".to_string(),
            Token::Ident(s) => s.clone().to_uppercase(),
            _ => return None,
        };
        self.advance();

        if matches!(self.peek(), Token::LParen) {
            self.advance();
        }

        let path = if matches!(self.peek(), Token::Str(_)) {
            if let Token::Str(s) = self.advance() {
                s
            } else {
                "/".to_string()
            }
        } else {
            "/".to_string()
        };

        if matches!(self.peek(), Token::RParen) {
            self.advance();
        }

        if method == "INJECT" {
            let mut node = AstNode::new("_inject");
            node.name = Some("_inject".to_string());
            node.props.insert("path".to_string(), path);
            return Some(node);
        }

        let mut node = AstNode::new("decorator");
        node.name = Some(method);
        node.props.insert("path".to_string(), path);
        Some(node)
    }

    fn parse_method_or_property(&mut self) -> AstNode {
        let name = self.parse_name();

        if matches!(self.peek(), Token::LParen) {
            self.advance();
            let params = self.parse_params();
            let body = if matches!(self.peek(), Token::LBrace) {
                self.advance();
                let body_content = self.parse_block_content();
                if matches!(self.peek(), Token::RBrace) {
                    self.advance();
                }
                body_content
            } else {
                self.skip_to_newline();
                String::new()
            };

            let mut node = AstNode::new("method");
            node.name = Some(name);
            node.props.insert("params".to_string(), params);
            node.props.insert("body".to_string(), body);
            node
        } else if matches!(self.peek(), Token::Str(_)) {
            let value = if let Token::Str(s) = self.peek().clone() {
                self.advance();
                s
            } else {
                String::new()
            };
            let mut node = AstNode::new("property");
            node.name = Some(name);
            node.props.insert("value".to_string(), value);
            node
        } else if matches!(self.peek(), Token::Colon) || matches!(self.peek(), Token::Eq) {
            let is_type_annotation = matches!(self.peek(), Token::Colon);
            self.advance();
            let value = if is_type_annotation {
                self.skip_to_newline();
                String::new()
            } else {
                self.parse_value()
            };
            let mut node = AstNode::new("property");
            node.name = Some(name);
            node.props.insert("value".to_string(), value);
            node
        } else {
            let mut node = AstNode::new("property");
            node.name = Some(name);
            node
        }
    }

    fn parse_params(&mut self) -> String {
        let mut params = Vec::new();
        while !self.is_at_end() && !matches!(self.peek(), Token::RParen) {
            if matches!(self.peek(), Token::Ident(_)) {
                if let Token::Ident(s) = self.peek().clone() {
                    params.push(s);
                }
                self.advance();
            }
            if matches!(self.peek(), Token::Comma) {
                self.advance();
            }
        }
        if matches!(self.peek(), Token::RParen) {
            self.advance();
        }
        params.join(", ")
    }

    fn parse_block_content(&mut self) -> String {
        let mut depth = 1;
        let mut content = String::new();

        while !self.is_at_end() && depth > 0 {
            let t = self.peek().clone();
            match t {
                Token::LBrace => {
                    depth += 1;
                    content.push('{');
                    self.advance();
                }
                Token::RBrace => {
                    depth -= 1;
                    if depth > 0 {
                        content.push('}');
                    }
                    self.advance();
                }
                Token::LParen => {
                    content.push('(');
                    self.advance();
                }
                Token::RParen => {
                    content.push(')');
                    self.advance();
                }
                Token::Newline => {
                    content.push(' ');
                    self.advance();
                }
                Token::Str(s) => {
                    content.push('"');
                    content.push_str(&s);
                    content.push('"');
                    self.advance();
                }
                Token::Num(n) => {
                    content.push_str(&n);
                    self.advance();
                }
                Token::True => {
                    content.push_str("true");
                    self.advance();
                }
                Token::False => {
                    content.push_str("false");
                    self.advance();
                }
                Token::Null => {
                    content.push_str("null");
                    self.advance();
                }
                Token::Dot => {
                    content.push('.');
                    self.advance();
                }
                Token::Comma => {
                    content.push(',');
                    self.advance();
                }
                Token::Colon => {
                    content.push(':');
                    content.push(' ');
                    self.advance();
                }
                Token::Eq => {
                    content.push_str(" = ");
                    self.advance();
                }
                Token::Semi => {
                    content.push(';');
                    self.advance();
                }
                Token::Plus => {
                    content.push('+');
                    self.advance();
                }
                Token::Minus => {
                    content.push('-');
                    self.advance();
                }
                Token::Star => {
                    content.push('*');
                    self.advance();
                }
                Token::Slash => {
                    content.push('/');
                    self.advance();
                }
                Token::Ident(s) => {
                    content.push_str(&s);
                    self.advance();
                }
                Token::Return => {
                    content.push_str("return ");
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }

        content.trim().to_string()
    }

    fn parse_field(&mut self) -> AstNode {
        let name = self.parse_name();

        let field_type = if matches!(self.peek(), Token::Colon) {
            self.advance();
            self.parse_value()
        } else {
            "any".to_string()
        };

        let mut node = AstNode::new("field");
        node.name = Some(name);
        node.props.insert("type".to_string(), field_type);
        node
    }

    fn parse_value(&mut self) -> String {
        let mut value = String::new();
        while !self.is_at_end()
            && !matches!(self.peek(), Token::Newline)
            && !matches!(self.peek(), Token::Semi)
            && !matches!(self.peek(), Token::RBrace)
            && !matches!(self.peek(), Token::Comma)
        {
            match self.peek().clone() {
                Token::Str(s) => {
                    value.push('"');
                    value.push_str(&s);
                    value.push('"');
                    self.advance();
                }
                Token::Ident(s) => {
                    value.push_str(&s);
                    value.push(' ');
                    self.advance();
                }
                Token::Num(n) => {
                    value.push_str(&n);
                    self.advance();
                }
                Token::True => {
                    value.push_str("true");
                    self.advance();
                }
                Token::False => {
                    value.push_str("false");
                    self.advance();
                }
                Token::Null => {
                    value.push_str("null");
                    self.advance();
                }
                Token::Dot => {
                    value.push('.');
                    self.advance();
                }
                Token::Colon => {
                    value.push(':');
                    value.push(' ');
                    self.advance();
                }
                Token::Eq => {
                    value.push_str(" = ");
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }
        value.trim().to_string()
    }

    fn skip_to_newline(&mut self) {
        while !self.is_at_end() && !matches!(self.peek(), Token::Newline) {
            self.advance();
        }
    }
}
