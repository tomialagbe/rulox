use std::fmt::{Display};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // end of file
    Eof,
}

#[derive(Debug, Clone)]
pub struct TokenLiteral {
    string_val: Option<String>,
    numeric_val: Option<f64>,
}

impl TokenLiteral {
    pub fn from_string(val: &str) -> Self {
        TokenLiteral {
            string_val: Some(val.to_owned()),
            numeric_val: None,
        }
    }

    pub fn from_number(val: f64) -> Self {
        TokenLiteral {
            string_val: None,
            numeric_val: Some(val),
        }
    }

    pub fn string_val(&self) -> Option<&String> {
        self.string_val.as_ref()
    }

    pub fn num_val(&self) -> Option<f64> {
        self.numeric_val
    }
}

impl Default for TokenLiteral {
    fn default() -> Self {
        Self {
            string_val: Default::default(),
            numeric_val: Default::default(),
        }
    }
}

impl Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.string_val.is_some() {
            write!(f, "{}", self.string_val.as_ref().unwrap_or(&"".to_string()))
        } else {
            write!(f, "{}", self.numeric_val.unwrap_or_default())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<TokenLiteral>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<TokenLiteral>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.clone().unwrap_or_default(),
            // self.literal.as_ref().unwrap_or(&String::new()),
        )
    }
}
