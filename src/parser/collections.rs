use crate::lexer::{Token, TokenType};
use crate::error::{BcclError, BcclResult, Span};
use super::ast::Expr;
use super::Parser;

impl Parser {
    pub fn parse_list(&mut self) -> BcclResult<Expr> {
        let left_bracket = self.expect_token(TokenType::LeftBracket)?;
        let start_span = left_bracket.span;
        
        let mut elements = Vec::new();
        
        // Handle empty list
        if let Some(Token { token_type: TokenType::RightBracket, .. }) = self.current_token() {
            let right_bracket = self.expect_token(TokenType::RightBracket)?;
            let span = start_span.combine(&right_bracket.span);
            return Ok(Expr::List { elements, span });
        }
        
        // Parse list elements
        loop {
            elements.push(self.parse_expression()?);
            
            match self.current_token() {
                Some(Token { token_type: TokenType::Comma, .. }) => {
                    self.advance();
                    continue;
                }
                Some(Token { token_type: TokenType::RightBracket, .. }) => break,
                Some(token) => {
                    let found_str = self.token_type_name(&token.token_type);
                    return Err(BcclError::unexpected_token(&found_str, token.span, 
                        vec![",".to_string(), "]".to_string()]));
                }
                None => {
                    return Err(BcclError::unexpected_eof(start_span, 
                        vec![",".to_string(), "]".to_string()]));
                }
            }
        }
        
        let right_bracket = self.expect_token(TokenType::RightBracket)?;
        let span = start_span.combine(&right_bracket.span);
        
        Ok(Expr::List { elements, span })
    }
    
    pub fn parse_dictionary(&mut self) -> BcclResult<Expr> {
        let left_brace = self.expect_token(TokenType::LeftBrace)?;
        let start_span = left_brace.span;
        
        let mut pairs = Vec::new();
        
        // Handle empty dictionary
        if let Some(Token { token_type: TokenType::RightBrace, .. }) = self.current_token() {
            let right_brace = self.expect_token(TokenType::RightBrace)?;
            let span = start_span.combine(&right_brace.span);
            return Ok(Expr::Dictionary { pairs, span });
        }
        
        // Parse dictionary key-value pairs
        loop {
            // Parse key (must be a string)
            let key = match self.current_token() {
                Some(Token { token_type: TokenType::String(s), .. }) => {
                    let key = s.clone();
                    self.advance();
                    key
                }
                Some(token) => {
                    let found_str = self.token_type_name(&token.token_type);
                    return Err(BcclError::unexpected_token(&found_str, token.span, 
                        vec!["string".to_string()]));
                }
                None => {
                    return Err(BcclError::unexpected_eof(start_span, 
                        vec!["string".to_string()]));
                }
            };
            
            // Expect colon
            self.expect_token(TokenType::Colon)?;
            
            // Parse value
            let value = self.parse_expression()?;
            
            pairs.push((key, value));
            
            match self.current_token() {
                Some(Token { token_type: TokenType::Comma, .. }) => {
                    self.advance();
                    continue;
                }
                Some(Token { token_type: TokenType::RightBrace, .. }) => break,
                Some(token) => {
                    let found_str = self.token_type_name(&token.token_type);
                    return Err(BcclError::unexpected_token(&found_str, token.span, 
                        vec![",".to_string(), "}".to_string()]));
                }
                None => {
                    return Err(BcclError::unexpected_eof(start_span, 
                        vec![",".to_string(), "}".to_string()]));
                }
            }
        }
        
        let right_brace = self.expect_token(TokenType::RightBrace)?;
        let span = start_span.combine(&right_brace.span);
        
        Ok(Expr::Dictionary { pairs, span })
    }
}