use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character: {ch:?} at line {line}, column {column}")]
    UnexpectedChar {
        ch: char,
        line: usize,
        column: usize,
    },

    #[error("Unterminated string at line {line}, column {column}")]
    UnterminatedString { line: usize, column: usize },

    #[error("Invalid number format: {0}")]
    InvalidNumber(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),

    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LessThan,
    GreaterThan,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Arrow,
    DoubleColon,
    Pipe,
    QuestionMark,
    Equals,
    NotEquals,
    EqualsEquals,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    And,
    Or,
    Not,
    At,
    Hash,
    Newline,
    Comment(String),
    Eof,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Module,
    Controller,
    Service,
    Guard,
    Middleware,
    Resolver,
    Gateway,
    Dto,
    Interface,
    Enum,
    Import,
    Export,
    From,
    As,
    If,
    Else,
    For,
    While,
    Return,
    Break,
    Continue,
    Throw,
    Try,
    Catch,
    Async,
    Await,
    Public,
    Private,
    Protected,
    Static,
    Readonly,
    Abstract,
    True,
    False,
    Null,
    Undefined,
    Type,
    Extends,
    Implements,
    In,
    Of,
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
    Query,
    Mutation,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "module" => Some(Keyword::Module),
            "controller" => Some(Keyword::Controller),
            "service" => Some(Keyword::Service),
            "guard" => Some(Keyword::Guard),
            "middleware" => Some(Keyword::Middleware),
            "resolver" => Some(Keyword::Resolver),
            "gateway" => Some(Keyword::Gateway),
            "dto" => Some(Keyword::Dto),
            "interface" => Some(Keyword::Interface),
            "enum" => Some(Keyword::Enum),
            "import" => Some(Keyword::Import),
            "export" => Some(Keyword::Export),
            "from" => Some(Keyword::From),
            "as" => Some(Keyword::As),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "for" => Some(Keyword::For),
            "while" => Some(Keyword::While),
            "return" => Some(Keyword::Return),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "throw" => Some(Keyword::Throw),
            "try" => Some(Keyword::Try),
            "catch" => Some(Keyword::Catch),
            "async" => Some(Keyword::Async),
            "await" => Some(Keyword::Await),
            "public" => Some(Keyword::Public),
            "private" => Some(Keyword::Private),
            "protected" => Some(Keyword::Protected),
            "static" => Some(Keyword::Static),
            "readonly" => Some(Keyword::Readonly),
            "abstract" => Some(Keyword::Abstract),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "null" => Some(Keyword::Null),
            "undefined" => Some(Keyword::Undefined),
            "type" => Some(Keyword::Type),
            "extends" => Some(Keyword::Extends),
            "implements" => Some(Keyword::Implements),
            "in" => Some(Keyword::In),
            "of" => Some(Keyword::Of),
            "get" => Some(Keyword::Get),
            "post" => Some(Keyword::Post),
            "put" => Some(Keyword::Put),
            "patch" => Some(Keyword::Patch),
            "delete" => Some(Keyword::Delete),
            "head" => Some(Keyword::Head),
            "options" => Some(Keyword::Options),
            "query" => Some(Keyword::Query),
            "mutation" => Some(Keyword::Mutation),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            kind,
            lexeme,
            line,
            column,
        }
    }
}

pub struct Lexer<'a> {
    #[allow(dead_code)]
    source: &'a str,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            source,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            self.skip_whitespace_and_comments()?;
            if self.is_at_end() {
                break;
            }
            let token = self.scan_token()?;
            if !matches!(token.kind, TokenKind::Comment(_) | TokenKind::Newline) {
                tokens.push(token);
            }
        }
        tokens.push(Token::new(
            TokenKind::Eof,
            String::new(),
            self.line,
            self.column,
        ));
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;
        let c = self.advance();

        let kind = match c {
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '<' => TokenKind::LessThan,
            '>' => TokenKind::GreaterThan,
            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '?' => TokenKind::QuestionMark,
            '@' => TokenKind::At,
            '#' => TokenKind::Hash,
            '+' => TokenKind::Plus,
            '%' => TokenKind::Percent,

            '=' => {
                return Ok(Token::new(
                    if self.peek() == Some('>') {
                        self.advance();
                        TokenKind::Arrow
                    } else if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::EqualsEquals
                    } else {
                        TokenKind::Equals
                    },
                    format!(
                        "={}",
                        if self.peek() == Some('>') || self.peek() == Some('=') {
                            "".to_string()
                        } else {
                            String::new()
                        }
                    ),
                    start_line,
                    start_column,
                ))
            }
            '-' => {
                if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    TokenKind::NotEquals
                } else {
                    TokenKind::Not
                }
            }
            '&' => TokenKind::And,
            '|' => TokenKind::Pipe,
            '\n' => {
                self.line += 1;
                self.column = 1;
                TokenKind::Newline
            }

            '"' | '\'' => return self.scan_string(c, start_line, start_column),
            _ if c.is_alphabetic() || c == '_' => {
                return Ok(self.scan_identifier_or_keyword(c, start_line, start_column))
            }
            _ if c.is_numeric() => return self.scan_number(c, start_line, start_column),
            _ => {
                return Err(LexerError::UnexpectedChar {
                    ch: c,
                    line: start_line,
                    column: start_column,
                })
            }
        };

        Ok(Token::new(kind, c.to_string(), start_line, start_column))
    }

    fn scan_identifier_or_keyword(&mut self, first: char, line: usize, column: usize) -> Token {
        let mut lexeme = first.to_string();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                lexeme.push(self.advance());
            } else {
                break;
            }
        }
        if let Some(keyword) = Keyword::from_str(&lexeme) {
            Token::new(TokenKind::Keyword(keyword), lexeme, line, column)
        } else {
            match lexeme.as_str() {
                "true" => Token::new(TokenKind::Boolean(true), lexeme, line, column),
                "false" => Token::new(TokenKind::Boolean(false), lexeme, line, column),
                _ => Token::new(TokenKind::Identifier(lexeme.clone()), lexeme, line, column),
            }
        }
    }

    fn scan_number(
        &mut self,
        first: char,
        line: usize,
        column: usize,
    ) -> Result<Token, LexerError> {
        let mut lexeme = first.to_string();
        let mut is_float = first == '.';
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                lexeme.push(self.advance());
            } else if c == '.' && !is_float {
                is_float = true;
                lexeme.push(self.advance());
            } else {
                break;
            }
        }
        if is_float {
            let lexeme_clone = lexeme.clone();
            lexeme
                .parse::<f64>()
                .map(move |n| Token::new(TokenKind::Number(n), lexeme_clone, line, column))
                .map_err(|_| LexerError::InvalidNumber(lexeme))
        } else {
            let lexeme_clone = lexeme.clone();
            lexeme
                .parse::<i64>()
                .map(move |n| Token::new(TokenKind::Integer(n), lexeme_clone, line, column))
                .map_err(|_| LexerError::InvalidNumber(lexeme))
        }
    }

    fn scan_string(
        &mut self,
        quote: char,
        line: usize,
        column: usize,
    ) -> Result<Token, LexerError> {
        let mut value = String::new();
        while let Some(c) = self.peek() {
            if c == quote {
                self.advance();
                return Ok(Token::new(
                    TokenKind::String(value.clone()),
                    format!("{}{}", quote, value),
                    line,
                    column,
                ));
            }
            if c == '\\' {
                self.advance();
                if let Some(e) = self.peek() {
                    value.push(match e {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '0' => '\0',
                        _ => e,
                    });
                    self.advance();
                }
            } else if c == '\n' {
                return Err(LexerError::UnterminatedString {
                    line: self.line,
                    column: self.column,
                });
            } else {
                value.push(self.advance());
            }
        }
        Err(LexerError::UnterminatedString { line, column })
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), LexerError> {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '/' if self.peek_next() == Some('/') => {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                '/' if self.peek_next() == Some('*') => {
                    self.advance();
                    self.advance();
                    loop {
                        match self.peek() {
                            None => {
                                return Err(LexerError::UnexpectedChar {
                                    ch: '*',
                                    line: self.line,
                                    column: self.column,
                                });
                            }
                            Some('*') if self.peek_next() == Some('/') => {
                                self.advance();
                                self.advance();
                                break;
                            }
                            Some('\n') => {
                                self.line += 1;
                                self.column = 1;
                                self.advance();
                            }
                            _ => {
                                self.advance();
                            }
                        }
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }
    fn advance(&mut self) -> char {
        let c = self.chars[self.pos];
        self.pos += 1;
        self.column += 1;
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let source = "module controller service guard";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        assert!(matches!(
            tokens[0].kind,
            TokenKind::Keyword(Keyword::Module)
        ));
    }

    #[test]
    fn test_strings() {
        let source = r#""hello world""#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "hello world"));
    }
}
