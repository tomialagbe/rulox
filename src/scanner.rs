use std::collections::HashMap;

use crate::token::{Token, TokenLiteral, TokenType};

#[derive(Debug)]
pub enum ScanError {
    UnexpectedCharacter { line: usize, ch: char },
    UnterminatedString { line: usize },
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut sc = Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::new(),
        };
        sc.init_keywords();
        return sc;
    }

    fn init_keywords(&mut self) {
        self.keywords.insert(String::from("and"), TokenType::And);
        self.keywords
            .insert(String::from("class"), TokenType::Class);
        self.keywords.insert(String::from("else"), TokenType::Else);
        self.keywords
            .insert(String::from("false"), TokenType::False);
        self.keywords.insert(String::from("for"), TokenType::For);
        self.keywords.insert(String::from("fun"), TokenType::Fun);
        self.keywords.insert(String::from("if"), TokenType::If);
        self.keywords.insert(String::from("nil"), TokenType::Nil);
        self.keywords.insert(String::from("or"), TokenType::Or);
        self.keywords
            .insert(String::from("print"), TokenType::Print);
        self.keywords
            .insert(String::from("return"), TokenType::Return);
        self.keywords
            .insert(String::from("super"), TokenType::Super);
        self.keywords.insert(String::from("this"), TokenType::This);
        self.keywords.insert(String::from("true"), TokenType::True);
        self.keywords.insert(String::from("var"), TokenType::Var);
        self.keywords
            .insert(String::from("while"), TokenType::While);
    }

    fn is_at_end(&self) -> bool {
        return self.current > self.source.len();
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.source.chars().nth(self.current);
        self.current += 1;
        return char;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap_or('\0');
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        return self.source.chars().nth(self.current + 1).unwrap_or('\0');
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let Some(c) = self.source.chars().nth(self.current) else {
            return false;
        };

        if c != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn string(&mut self) -> Result<String, ScanError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ScanError::UnterminatedString { line: self.line });
        }

        // the closing ".
        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        println!("Parsed string {}", value);
        return Ok(value.to_string());
    }

    fn number(&mut self) -> f64 {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // look for the fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value_str = &self.source[self.start..self.current];
        let value_num = value_str.parse::<f64>().unwrap_or(0.0);
        return value_num;
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        match self.keywords.get(text) {
            Some(keyword) => self.add_token(keyword.clone()),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn is_alpha(&self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn scan_token(&mut self) -> Result<Option<Token>, ScanError> {
        let c = self.advance();
        use TokenType::*;
        match c {
            Some('(') => Ok(Some(self.add_token(LeftParen))),
            Some(')') => Ok(Some(self.add_token(RightParen))),
            Some('{') => Ok(Some(self.add_token(LeftBrace))),
            Some('}') => Ok(Some(self.add_token(RightBrace))),
            Some(',') => Ok(Some(self.add_token(Comma))),
            Some('.') => Ok(Some(self.add_token(Dot))),
            Some('-') => Ok(Some(self.add_token(Minus))),
            Some('+') => Ok(Some(self.add_token(Plus))),
            Some(';') => Ok(Some(self.add_token(Semicolon))),
            Some('*') => Ok(Some(self.add_token(Star))),
            Some('!') => {
                let matches_next = self.match_next('=');
                Ok(Some(self.add_token(if matches_next {
                    BangEqual
                } else {
                    Bang
                })))
            }
            Some('=') => {
                let matches_next = self.match_next('=');
                Ok(Some(self.add_token(if matches_next {
                    EqualEqual
                } else {
                    Equal
                })))
            }
            Some('<') => {
                let matches_next = self.match_next('=');
                Ok(Some(self.add_token(if matches_next {
                    LessEqual
                } else {
                    Less
                })))
            }
            Some('>') => {
                let matches_next = self.match_next('=');
                Ok(Some(self.add_token(if matches_next {
                    GreaterEqual
                } else {
                    Greater
                })))
            }
            Some('/') => {
                if self.match_next('/') {
                    // single line comment //
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                } else if self.match_next('*') {
                    // multi line comment /* */
                    while self.peek() != '*' && !self.is_at_end() {
                        self.advance();
                    }

                    // capture the closing "*"
                    self.advance();
                    while self.peek() != '/' {
                        self.advance();
                    }

                    // capture closing "/"
                    self.advance();
                    Ok(None)
                } else {
                    Ok(Some(self.add_token(Slash)))
                }
            }
            Some('"') => {
                let possible_str = self.string();
                match possible_str {
                    Ok(str_token) => {
                        let tok =
                            self.add_token_(String, Some(TokenLiteral::from_string(&str_token)));
                        Ok(Some(tok))
                    }
                    Err(err) => Err(err),
                }
            }
            Some(' ' | '\r' | '\t') => Ok(None),
            Some('\n') => {
                self.line += 1;
                Ok(None)
            }
            Some('\0') => Ok(None),
            None => Ok(None),
            Some(ch) => {
                if ch.is_ascii_digit() {
                    let num = self.number();
                    let tok = self.add_token_(Number, Some(TokenLiteral::from_number(num)));
                    Ok(Some(tok))
                } else if self.is_alpha(ch) {
                    let identifier = self.identifier();
                    Ok(Some(identifier))
                } else {
                    Err(ScanError::UnexpectedCharacter {
                        line: self.line,
                        ch,
                    })
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) -> Token {
        self.add_token_(token_type, None)
    }

    fn add_token_(&mut self, token_type: TokenType, literal: Option<TokenLiteral>) -> Token {
        let text = &self.source[self.start..self.current];
        let new_token = Token::new(token_type, text.to_string(), literal, self.line);
        // self.tokens.append(new_token.);
        new_token
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = std::result::Result<Token, ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            let new_token = Token::new(TokenType::Eof, "".to_string(), None, self.line);
            return Some(Ok(new_token));
        }

        self.start = self.current;

        match self.scan_token() {
            Ok(new_token) => {
                if new_token.is_some() {
                    let inner = new_token.unwrap();
                    self.tokens.push(inner.clone());
                    Some(Ok(inner))
                } else {
                    self.next()
                }
            }
            Err(err) => Some(Err(err)),
        }
    }
}
