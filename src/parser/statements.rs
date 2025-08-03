use crate::lexer::{Token, TokenType};
use crate::error::{BcclError, BcclResult, Span};
use super::ast::{Stmt, CompoundOp};
use super::Parser;

impl Parser {
    pub fn parse_statement(&mut self) -> BcclResult<Stmt> {
        if let Some(token) = self.current_token() {
            if let TokenType::Identifier(_name) = &token.token_type {
                if let Some(next_token) = self.peek_token() {
                    match next_token.token_type {
                        TokenType::Assign => {
                            return self.parse_assignment();
                        }
                        TokenType::PlusAssign | TokenType::MinusAssign | 
                        TokenType::MultiplyAssign | TokenType::DivideAssign => {
                            return self.parse_compound_assignment();
                        }
                        _ => {}
                    }
                }
            }
        }
        
        let expr = self.parse_expression()?;
        let span = expr.span();
        
        if let Some(token) = self.current_token() {
            if matches!(token.token_type, TokenType::Semicolon) {
                self.advance();
            }
        }
        
        Ok(Stmt::Expression { expr, span })
    }
    
    pub fn parse_assignment(&mut self) -> BcclResult<Stmt> {
        let (name, name_span) = if let Some(Token { token_type: TokenType::Identifier(name), span }) = self.current_token() {
            let name = name.clone();
            let span = *span;
            self.advance();
            (name, span)
        } else {
            return Err(BcclError::unexpected_token("Expected identifier", 
                self.current_token().map_or(Span::single(0), |t| t.span), 
                vec!["identifier".to_string()]));
        };
        
        let assign_token = self.expect_token(TokenType::Assign)?;
        let value = self.parse_expression()?;
        let span = name_span.combine(&value.span());
        
        if let Some(token) = self.current_token() {
            if matches!(token.token_type, TokenType::Semicolon) {
                self.advance();
            }
        }
        
        Ok(Stmt::Assignment { name, value, span })
    }
    
    pub fn parse_compound_assignment(&mut self) -> BcclResult<Stmt> {
        let (name, name_span) = if let Some(Token { token_type: TokenType::Identifier(name), span }) = self.current_token() {
            let name = name.clone();
            let span = *span;
            self.advance();
            (name, span)
        } else {
            return Err(BcclError::unexpected_token("Expected identifier", 
                self.current_token().map_or(Span::single(0), |t| t.span), 
                vec!["identifier".to_string()]));
        };
        
        let operator = match self.current_token() {
            Some(Token { token_type: TokenType::PlusAssign, .. }) => CompoundOp::Add,
            Some(Token { token_type: TokenType::MinusAssign, .. }) => CompoundOp::Subtract,
            Some(Token { token_type: TokenType::MultiplyAssign, .. }) => CompoundOp::Multiply,
            Some(Token { token_type: TokenType::DivideAssign, .. }) => CompoundOp::Divide,
            Some(token) => {
                let found_str = self.token_type_name(&token.token_type);
                return Err(BcclError::unexpected_token(&found_str, token.span, 
                    vec!["+=".to_string(), "-=".to_string(), "*=".to_string(), "/=".to_string()]));
            }
            None => {
                return Err(BcclError::unexpected_eof(name_span, 
                    vec!["+=".to_string(), "-=".to_string(), "*=".to_string(), "/=".to_string()]));
            }
        };
        
        self.advance(); // consume the compound assignment operator
        let value = self.parse_expression()?;
        let span = name_span.combine(&value.span());
        
        if let Some(token) = self.current_token() {
            if matches!(token.token_type, TokenType::Semicolon) {
                self.advance();
            }
        }
        
        Ok(Stmt::CompoundAssignment { name, operator, value, span })
    }
}