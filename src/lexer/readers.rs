use crate::error::{BcclError, BcclResult, Span};
use super::token::TokenType;
use super::Lexer;

impl Lexer {
    pub fn read_number_or_check_malformed(&mut self) -> BcclResult<(TokenType, Span)> {
        let start_pos = self.position;
        let mut number_str = String::new();
        let mut dot_count = 0;
        let mut is_float = false;
        
        // Read the entire potential number including malformed parts
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else if ch == '.' {
                dot_count += 1;
                is_float = true;
                number_str.push(ch);
                self.advance();
                
                // If we have more than one dot, this is malformed
                if dot_count > 1 {
                    // Continue reading to capture the full malformed number
                    while let Some(ch) = self.current_char {
                        if ch.is_ascii_digit() || ch == '.' {
                            number_str.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let end_pos = self.position;
                    let span = Span::new(start_pos, end_pos);
                    return Err(BcclError::malformed_number(&number_str, span));
                }
            } else {
                break;
            }
        }
        
        let end_pos = self.position;
        let span = Span::new(start_pos, end_pos);
        
        // Validate the number format
        if number_str.is_empty() || number_str == "." {
            return Err(BcclError::malformed_number(&number_str, span));
        }
        
        // Check for invalid patterns like ending with a dot
        if number_str.ends_with('.') && number_str.len() > 1 {
            return Err(BcclError::malformed_number(&number_str, span));
        }
        
        if is_float {
            match number_str.parse::<f64>() {
                Ok(num) => Ok((TokenType::Number(num), span)),
                Err(_) => Err(BcclError::malformed_number(&number_str, span)),
            }
        } else {
            // Try to parse as integer first, then as float if needed
            match number_str.parse::<i64>() {
                Ok(num) => Ok((TokenType::Integer(num), span)),
                Err(_) => match number_str.parse::<f64>() {
                    Ok(num) => Ok((TokenType::Number(num), span)),
                    Err(_) => Err(BcclError::malformed_number(&number_str, span)),
                }
            }
        }
    }
    
    pub fn read_string(&mut self) -> BcclResult<(String, Span)> {
        let start_pos = self.position;
        self.advance(); // Skip opening quote
        
        let mut string_val = String::new();
        
        while let Some(ch) = self.current_char {
            match ch {
                '"' => {
                    self.advance(); // Skip closing quote
                    let end_pos = self.position;
                    let span = Span::new(start_pos, end_pos);
                    return Ok((string_val, span));
                }
                '\\' => {
                    self.advance();
                    match self.current_char {
                        Some('n') => string_val.push('\n'),
                        Some('t') => string_val.push('\t'),
                        Some('r') => string_val.push('\r'),
                        Some('\\') => string_val.push('\\'),
                        Some('"') => string_val.push('"'),
                        Some(ch) => {
                            string_val.push('\\');
                            string_val.push(ch);
                        }
                        None => {
                            let end_pos = self.position;
                            let span = Span::new(start_pos, end_pos);
                            return Err(BcclError::malformed_number("Unterminated string literal", span));
                        }
                    }
                    self.advance();
                }
                ch => {
                    string_val.push(ch);
                    self.advance();
                }
            }
        }
        
        // If we get here, the string was not terminated
        let end_pos = self.position;
        let span = Span::new(start_pos, end_pos);
        Err(BcclError::malformed_number("Unterminated string literal", span))
    }
    
    pub fn read_identifier(&mut self) -> (TokenType, Span) {
        let start_pos = self.position;
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        let end_pos = self.position;
        let span = Span::new(start_pos, end_pos);
        
        // Check for keywords
        let token_type = match identifier.as_str() {
            "true" => TokenType::Boolean(true),
            "false" => TokenType::Boolean(false),
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            "in" => TokenType::In,
            _ => TokenType::Identifier(identifier),
        };
        
        (token_type, span)
    }
}