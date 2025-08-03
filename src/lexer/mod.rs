use crate::error::{BcclError, BcclResult, Span};

mod token;
mod readers;

#[cfg(test)]
mod tests;

pub use token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Self {
            input: chars,
            position: 0,
            current_char,
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }
    
    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    pub fn next_token(&mut self) -> BcclResult<Token> {
        self.skip_whitespace();
        
        let token = match self.current_char {
            // Compound assignment and comparison operators
            Some('+') => {
                let start_pos = self.position;
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::new(TokenType::PlusAssign, Span::new(start_pos, self.position))
                } else {
                    Token::new(TokenType::Plus, Span::single(start_pos))
                }
            }
            Some('-') => {
                let start_pos = self.position;
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::new(TokenType::MinusAssign, Span::new(start_pos, self.position))
                } else {
                    Token::new(TokenType::Minus, Span::single(start_pos))
                }
            }
            Some('*') => {
                let start_pos = self.position;
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::new(TokenType::MultiplyAssign, Span::new(start_pos, self.position))
                } else {
                    Token::new(TokenType::Multiply, Span::single(start_pos))
                }
            }
            Some('/') => {
                let start_pos = self.position;
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::new(TokenType::DivideAssign, Span::new(start_pos, self.position))
                } else {
                    Token::new(TokenType::Divide, Span::single(start_pos))
                }
            }
            Some('=') => {
                let start_pos = self.position;
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::new(TokenType::Equal, Span::new(start_pos, self.position))
                } else {
                    Token::new(TokenType::Assign, Span::single(start_pos))
                }
            }
            Some('!') => {
                let start_pos = self.position;
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::new(TokenType::NotEqual, Span::new(start_pos, self.position))
                } else {
                    return Err(BcclError::invalid_character('!', Span::single(start_pos)));
                }
            }
            Some('<') => {
                let start_pos = self.position;
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::new(TokenType::LessEqual, Span::new(start_pos, self.position))
                } else {
                    Token::new(TokenType::Less, Span::single(start_pos))
                }
            }
            Some('>') => {
                let start_pos = self.position;
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::new(TokenType::GreaterEqual, Span::new(start_pos, self.position))
                } else {
                    Token::new(TokenType::Greater, Span::single(start_pos))
                }
            }
            
            // Delimiters
            Some('(') => {
                let span = Span::single(self.position);
                self.advance();
                Token::new(TokenType::LeftParen, span)
            }
            Some(')') => {
                let span = Span::single(self.position);
                self.advance();
                Token::new(TokenType::RightParen, span)
            }
            Some('[') => {
                let span = Span::single(self.position);
                self.advance();
                Token::new(TokenType::LeftBracket, span)
            }
            Some(']') => {
                let span = Span::single(self.position);
                self.advance();
                Token::new(TokenType::RightBracket, span)
            }
            Some('{') => {
                let span = Span::single(self.position);
                self.advance();
                Token::new(TokenType::LeftBrace, span)
            }
            Some('}') => {
                let span = Span::single(self.position);
                self.advance();
                Token::new(TokenType::RightBrace, span)
            }
            Some(',') => {
                let span = Span::single(self.position);
                self.advance();
                Token::new(TokenType::Comma, span)
            }
            Some(':') => {
                let span = Span::single(self.position);
                self.advance();
                Token::new(TokenType::Colon, span)
            }
            Some(';') => {
                let span = Span::single(self.position);
                self.advance();
                Token::new(TokenType::Semicolon, span)
            }
            
            // String literals
            Some('"') => {
                let (string_val, span) = self.read_string()?;
                Token::new(TokenType::String(string_val), span)
            }
            
            // Numbers
            Some(ch) if ch.is_ascii_digit() => {
                let (token_type, span) = self.read_number_or_check_malformed()?;
                Token::new(token_type, span)
            }
            
            // Identifiers and keywords
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let (token_type, span) = self.read_identifier();
                Token::new(token_type, span)
            }
            
            // Invalid characters
            Some(ch) => {
                let span = Span::single(self.position);
                self.advance();
                return Err(BcclError::invalid_character(ch, span));
            }
            
            // End of file
            None => Token::new(TokenType::Eof, Span::single(self.position))
        };
        
        Ok(token)
    }
    
    pub fn tokenize(&mut self) -> BcclResult<Vec<Token>> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.token_type, TokenType::Eof);
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}