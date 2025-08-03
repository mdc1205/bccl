use crate::lexer::{Token, TokenType};
use crate::error::{BcclError, BcclResult, Span};
use super::ast::{Expr, BinaryOp, UnaryOp};
use super::Parser;

impl Parser {
    pub fn parse_expression(&mut self) -> BcclResult<Expr> {
        self.parse_logical_or()
    }
    
    pub fn parse_logical_or(&mut self) -> BcclResult<Expr> {
        let mut left = self.parse_logical_and()?;
        
        while let Some(token) = self.current_token() {
            let op = match token.token_type {
                TokenType::Or => BinaryOp::Or,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_logical_and()?;
            let span = left.span().combine(&right.span());
            
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(left)
    }
    
    pub fn parse_logical_and(&mut self) -> BcclResult<Expr> {
        let mut left = self.parse_equality()?;
        
        while let Some(token) = self.current_token() {
            let op = match token.token_type {
                TokenType::And => BinaryOp::And,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_equality()?;
            let span = left.span().combine(&right.span());
            
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(left)
    }
    
    pub fn parse_equality(&mut self) -> BcclResult<Expr> {
        let mut left = self.parse_comparison()?;
        
        while let Some(token) = self.current_token() {
            let op = match token.token_type {
                TokenType::Equal => BinaryOp::Equal,
                TokenType::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_comparison()?;
            let span = left.span().combine(&right.span());
            
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(left)
    }
    
    pub fn parse_comparison(&mut self) -> BcclResult<Expr> {
        let mut left = self.parse_membership()?;
        
        while let Some(token) = self.current_token() {
            let op = match token.token_type {
                TokenType::Less => BinaryOp::Less,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_membership()?;
            let span = left.span().combine(&right.span());
            
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(left)
    }
    
    pub fn parse_membership(&mut self) -> BcclResult<Expr> {
        let mut left = self.parse_additive()?;
        
        while let Some(token) = self.current_token() {
            let op = match token.token_type {
                TokenType::In => BinaryOp::In,
                TokenType::Not => {
                    // Check for "not in"
                    if let Some(next_token) = self.peek_token() {
                        if matches!(next_token.token_type, TokenType::In) {
                            self.advance(); // consume "not"
                            self.advance(); // consume "in"
                            BinaryOp::NotIn
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            };
            
            if !matches!(op, BinaryOp::NotIn) {
                self.advance();
            }
            let right = self.parse_additive()?;
            let span = left.span().combine(&right.span());
            
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(left)
    }
    
    pub fn parse_additive(&mut self) -> BcclResult<Expr> {
        let mut left = self.parse_multiplicative()?;
        
        while let Some(token) = self.current_token() {
            let op = match token.token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                _ => break,
            };
            
            let op_span = token.span;
            self.advance();
            let right = self.parse_multiplicative()?;
            let span = left.span().combine(&right.span());
            
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(left)
    }
    
    pub fn parse_multiplicative(&mut self) -> BcclResult<Expr> {
        let mut left = self.parse_unary()?;
        
        while let Some(token) = self.current_token() {
            let op = match token.token_type {
                TokenType::Multiply => BinaryOp::Multiply,
                TokenType::Divide => BinaryOp::Divide,
                _ => break,
            };
            
            let op_span = token.span;
            self.advance();
            let right = self.parse_unary()?;
            let span = left.span().combine(&right.span());
            
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        
        Ok(left)
    }
    
    pub fn parse_unary(&mut self) -> BcclResult<Expr> {
        if let Some(token) = self.current_token() {
            match token.token_type {
                TokenType::Minus => {
                    let op_span = token.span;
                    self.advance();
                    let operand = self.parse_postfix()?;
                    let span = op_span.combine(&operand.span());
                    return Ok(Expr::Unary {
                        operator: UnaryOp::Minus,
                        operand: Box::new(operand),
                        span,
                    });
                }
                TokenType::Plus => {
                    let op_span = token.span;
                    self.advance();
                    let operand = self.parse_postfix()?;
                    let span = op_span.combine(&operand.span());
                    return Ok(Expr::Unary {
                        operator: UnaryOp::Plus,
                        operand: Box::new(operand),
                        span,
                    });
                }
                TokenType::Not => {
                    let op_span = token.span;
                    self.advance();
                    let operand = self.parse_postfix()?;
                    let span = op_span.combine(&operand.span());
                    return Ok(Expr::Unary {
                        operator: UnaryOp::Not,
                        operand: Box::new(operand),
                        span,
                    });
                }
                _ => {}
            }
        }
        
        self.parse_postfix()
    }
    
    pub fn parse_postfix(&mut self) -> BcclResult<Expr> {
        let mut expr = self.parse_primary()?;
        
        // Handle postfix operations like indexing
        while let Some(token) = self.current_token() {
            match token.token_type {
                TokenType::LeftBracket => {
                    self.advance(); // consume '['
                    let index = self.parse_expression()?;
                    let right_bracket = self.expect_token(TokenType::RightBracket)?;
                    let span = expr.span().combine(&right_bracket.span);
                    
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                        span,
                    };
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    pub fn parse_primary(&mut self) -> BcclResult<Expr> {
        match self.current_token() {
            Some(Token { token_type: TokenType::Number(n), span }) => {
                let value = *n;
                let span = *span;
                self.advance();
                Ok(Expr::Number { value, span })
            }
            Some(Token { token_type: TokenType::Integer(i), span }) => {
                let value = *i;
                let span = *span;
                self.advance();
                Ok(Expr::Integer { value, span })
            }
            Some(Token { token_type: TokenType::Boolean(b), span }) => {
                let value = *b;
                let span = *span;
                self.advance();
                Ok(Expr::Boolean { value, span })
            }
            Some(Token { token_type: TokenType::String(s), span }) => {
                let value = s.clone();
                let span = *span;
                self.advance();
                Ok(Expr::String { value, span })
            }
            Some(Token { token_type: TokenType::Identifier(name), span }) => {
                let name = name.clone();
                let start_span = *span;
                self.advance();
                
                // Check if this is a function call
                if let Some(Token { token_type: TokenType::LeftParen, .. }) = self.current_token() {
                    self.advance(); // consume '('
                    
                    let mut args = Vec::new();
                    let mut kwargs = Vec::new();
                    let mut found_kwarg = false;
                    
                    // Parse arguments (mix of positional and keyword)
                    if !matches!(self.current_token(), Some(Token { token_type: TokenType::RightParen, .. })) {
                        loop {
                            // Check if this looks like a keyword argument (identifier = expression)
                            if let Some(Token { token_type: TokenType::Identifier(param_name), .. }) = self.current_token() {
                                if let Some(Token { token_type: TokenType::Assign, .. }) = self.peek_token() {
                                    // This is a keyword argument
                                    found_kwarg = true;
                                    let param_name = param_name.clone();
                                    self.advance(); // consume identifier
                                    self.advance(); // consume '='
                                    let value = self.parse_expression()?;
                                    kwargs.push((param_name, value));
                                } else {
                                    // This is a positional argument (identifier expression)
                                    if found_kwarg {
                                        return Err(BcclError::unexpected_token(
                                            "positional argument", 
                                            self.current_token().unwrap().span, 
                                            vec!["keyword argument".to_string()]
                                        ));
                                    }
                                    args.push(self.parse_expression()?);
                                }
                            } else {
                                // This is a positional argument (non-identifier expression)
                                if found_kwarg {
                                    return Err(BcclError::unexpected_token(
                                        "positional argument", 
                                        self.current_token().unwrap().span, 
                                        vec!["keyword argument".to_string()]
                                    ));
                                }
                                args.push(self.parse_expression()?);
                            }
                            
                            match self.current_token() {
                                Some(Token { token_type: TokenType::Comma, .. }) => {
                                    self.advance();
                                    continue;
                                }
                                Some(Token { token_type: TokenType::RightParen, .. }) => break,
                                Some(token) => {
                                    let found_str = self.token_type_name(&token.token_type);
                                    return Err(BcclError::unexpected_token(&found_str, token.span, 
                                        vec![",".to_string(), ")".to_string()]));
                                }
                                None => {
                                    return Err(BcclError::unexpected_eof(start_span, 
                                        vec![",".to_string(), ")".to_string()]));
                                }
                            }
                        }
                    }
                    
                    let right_paren = self.expect_token(TokenType::RightParen)?;
                    let span = start_span.combine(&right_paren.span);
                    
                    Ok(Expr::FunctionCall { name, args, kwargs, span })
                } else {
                    Ok(Expr::Identifier { name, span: start_span })
                }
            }
            Some(Token { token_type: TokenType::LeftParen, span }) => {
                let left_paren_span = *span;
                self.advance();
                let expr = self.parse_expression()?;
                let right_paren = self.expect_token(TokenType::RightParen)?;
                let span = left_paren_span.combine(&right_paren.span);
                Ok(expr)
            }
            Some(Token { token_type: TokenType::LeftBracket, span }) => {
                self.parse_list()
            }
            Some(Token { token_type: TokenType::LeftBrace, span }) => {
                self.parse_dictionary()
            }
            Some(Token { token_type: TokenType::Eof, span }) => {
                // EOF should be treated as unexpected end of file
                // Use a single-character span at the end to show where input is expected
                let expected_span = if span.start > 0 {
                    Span::single(span.start - 1)
                } else {
                    Span::single(0)
                };
                Err(BcclError::unexpected_eof(expected_span, 
                    vec!["number".to_string(), "identifier".to_string(), "(".to_string()]))
            }
            Some(token) => {
                let found_str = self.token_type_name(&token.token_type);
                Err(BcclError::unexpected_token(&found_str, token.span, 
                    vec!["number".to_string(), "identifier".to_string(), "(".to_string()]))
            }
            None => {
                let span = if self.position > 0 {
                    self.tokens[self.position - 1].span
                } else {
                    Span::single(0)
                };
                Err(BcclError::unexpected_eof(span, 
                    vec!["number".to_string(), "identifier".to_string(), "(".to_string()]))
            }
        }
    }
}