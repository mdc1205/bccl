use crate::lexer::{Token, TokenType};
use crate::error::{BcclError, BcclResult, Span};

mod ast;
mod statements;
mod expressions;
mod collections;

pub use ast::{Expr, Stmt, Program, BinaryOp, UnaryOp, CompoundOp};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }
    
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    
    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn expect_token(&mut self, expected: TokenType) -> BcclResult<Token> {
        match self.current_token() {
            Some(token) if std::mem::discriminant(&token.token_type) == std::mem::discriminant(&expected) => {
                let token = token.clone();
                self.advance();
                Ok(token)
            }
            Some(token) if matches!(token.token_type, TokenType::Eof) => {
                // EOF token should be treated as unexpected end of file
                let expected_str = self.token_type_name(&expected);
                let span = if token.span.start > 0 {
                    Span::single(token.span.start - 1)
                } else {
                    Span::single(0)
                };
                Err(BcclError::unexpected_eof(span, vec![expected_str]))
            }
            Some(token) => {
                let expected_str = self.token_type_name(&expected);
                let found_str = self.token_type_name(&token.token_type);
                Err(BcclError::unexpected_token(&found_str, token.span, vec![expected_str]))
            }
            None => {
                let expected_str = self.token_type_name(&expected);
                let span = if self.position > 0 {
                    self.tokens[self.position - 1].span
                } else {
                    Span::single(0)
                };
                Err(BcclError::unexpected_eof(span, vec![expected_str]))
            }
        }
    }
    
    fn token_type_name(&self, token_type: &TokenType) -> String {
        match token_type {
            // Literals
            TokenType::Number(_) => "number".to_string(),
            TokenType::Integer(_) => "integer".to_string(),
            TokenType::Boolean(_) => "boolean".to_string(),
            TokenType::String(_) => "string".to_string(),
            TokenType::Identifier(_) => "identifier".to_string(),
            
            // Arithmetic operators
            TokenType::Plus => "+".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Multiply => "*".to_string(),
            TokenType::Divide => "/".to_string(),
            
            // Comparison operators
            TokenType::Equal => "==".to_string(),
            TokenType::NotEqual => "!=".to_string(),
            TokenType::Less => "<".to_string(),
            TokenType::Greater => ">".to_string(),
            TokenType::LessEqual => "<=".to_string(),
            TokenType::GreaterEqual => ">=".to_string(),
            
            // Logical
            TokenType::And => "and".to_string(),
            TokenType::Or => "or".to_string(),
            TokenType::Not => "not".to_string(),
            TokenType::In => "in".to_string(),
            
            // Assignment
            TokenType::Assign => "=".to_string(),
            TokenType::PlusAssign => "+=".to_string(),
            TokenType::MinusAssign => "-=".to_string(),
            TokenType::MultiplyAssign => "*=".to_string(),
            TokenType::DivideAssign => "/=".to_string(),
            
            // Delimiters
            TokenType::LeftParen => "(".to_string(),
            TokenType::RightParen => ")".to_string(),
            TokenType::LeftBracket => "[".to_string(),
            TokenType::RightBracket => "]".to_string(),
            TokenType::LeftBrace => "{".to_string(),
            TokenType::RightBrace => "}".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Colon => ":".to_string(),
            TokenType::Semicolon => ";".to_string(),
            
            // Special
            TokenType::Eof => "end of input".to_string(),
        }
    }
    
    pub fn parse(&mut self) -> BcclResult<Program> {
        let mut statements = Vec::new();
        
        while let Some(token) = self.current_token() {
            if matches!(token.token_type, TokenType::Eof) {
                break;
            }
            
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        
        Ok(Program { statements })
    }
}