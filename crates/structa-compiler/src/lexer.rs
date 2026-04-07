use crate::ast::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if let Token::Eof = tok {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_ws();
        if self.pos >= self.input.len() {
            return Token::Eof;
        }
        let c = self.input[self.pos];
        match c {
            '{' => {
                self.pos += 1;
                self.col += 1;
                Token::LBrace
            }
            '}' => {
                self.pos += 1;
                self.col += 1;
                Token::RBrace
            }
            '(' => {
                self.pos += 1;
                self.col += 1;
                Token::LParen
            }
            ')' => {
                self.pos += 1;
                self.col += 1;
                Token::RParen
            }
            '[' => {
                self.pos += 1;
                self.col += 1;
                Token::LBracket
            }
            ']' => {
                self.pos += 1;
                self.col += 1;
                Token::RBracket
            }
            ',' => {
                self.pos += 1;
                self.col += 1;
                Token::Comma
            }
            ';' => {
                self.pos += 1;
                self.col += 1;
                Token::Semi
            }
            '.' => {
                self.pos += 1;
                self.col += 1;
                Token::Dot
            }
            ':' => {
                self.pos += 1;
                self.col += 1;
                Token::Colon
            }
            '+' => {
                self.pos += 1;
                self.col += 1;
                Token::Plus
            }
            '-' => {
                self.pos += 1;
                self.col += 1;
                Token::Minus
            }
            '*' => {
                self.pos += 1;
                self.col += 1;
                Token::Star
            }
            '/' => {
                self.pos += 1;
                self.col += 1;
                Token::Slash
            }
            '=' => {
                self.pos += 1;
                self.col += 1;
                Token::Eq
            }
            '@' => {
                self.pos += 1;
                self.col += 1;
                Token::At
            }
            '\n' => {
                self.pos += 1;
                self.line += 1;
                self.col = 1;
                Token::Newline
            }
            '"' | '\'' => self.read_string(),
            '`' => self.read_template(),
            c if c.is_alphabetic() || c == '_' => self.read_ident(),
            c if c.is_numeric() => self.read_number(),
            _ => {
                self.pos += 1;
                self.col += 1;
                Token::Eof
            }
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            if c.is_whitespace() && c != '\n' {
                self.col += 1;
                self.pos += 1;
            } else if c == '/' && self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '/'
            {
                while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }
    }

    fn read_ident(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            if c.is_alphanumeric() || c == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        self.col += self.pos - start;
        let s: String = self.input[start..self.pos].iter().collect();
        match s.to_lowercase().as_str() {
            "controller" => Token::Controller,
            "service" => Token::Service,
            "dto" => Token::Dto,
            "model" => Token::Model,
            "entity" => Token::Entity,
            "repository" => Token::Repository,
            "resolver" => Token::Resolver,
            "middleware" => Token::Middleware,
            "inject" => Token::Inject,
            "module" => Token::Module,
            "get" => Token::Get,
            "post" => Token::Post,
            "put" => Token::Put,
            "delete" => Token::Delete,
            "patch" => Token::Patch,
            "use" => Token::Use,
            "all" => Token::All,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "const" => Token::Const,
            "let" => Token::Let,
            "class" => Token::Class,
            "new" => Token::New,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "async" => Token::Async,
            _ => Token::Ident(s),
        }
    }

    fn read_string(&mut self) -> Token {
        let quote = self.input[self.pos];
        self.pos += 1;
        self.col += 1;
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != quote {
            if self.input[self.pos] == '\\' {
                self.pos += 2;
            } else {
                self.pos += 1;
            }
        }
        let s: String = self.input[start..self.pos].iter().collect();
        self.pos += 1;
        self.col += 1;
        Token::Str(s)
    }

    fn read_template(&mut self) -> Token {
        self.pos += 1;
        self.col += 1;
        let start = self.pos;
        let mut depth = 1;
        while self.pos < self.input.len() && depth > 0 {
            let c = self.input[self.pos];
            match c {
                '`' => {
                    depth -= 1;
                    if depth > 0 {
                        self.pos += 1;
                    }
                }
                '$' if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '{' => {
                    depth += 1;
                    self.pos += 2;
                }
                '\\' => {
                    self.pos += 2;
                }
                _ => {
                    self.pos += 1;
                }
            }
        }
        let s: String = self.input[start..self.pos].iter().collect();
        self.pos += 1;
        self.col += 1;
        Token::Template(s)
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.input[self.pos].is_numeric() || self.input[self.pos] == '.')
        {
            self.pos += 1;
        }
        self.col += self.pos - start;
        let s: String = self.input[start..self.pos].iter().collect();
        Token::Num(s)
    }
}
