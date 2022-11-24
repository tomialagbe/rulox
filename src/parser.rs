use crate::{
    ast::expr::Expr,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// Equality Grammar:
    /// equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_terminals(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator: operator.unwrap(), // TODO! clean this up
            }
        }

        return expr;
    }

    fn comparison(&self) -> Expr {
        todo!()
    }

    /// Match terminals in grammar
    fn match_terminals(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek()
            .map_or(false, |t: Token| t.token_type == token_type)
    }

    fn previous(&self) -> Option<Token> {
        self.tokens.get(self.current - 1).cloned()
    }

    fn advance(&mut self) -> Option<Token>{
        if !self.is_at_end() {
            self.current += 1
        }
        return self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.token_type == TokenType::Eof,
            None => true,
        }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }
}
