use super::*;
use crate::error::BcclError;

#[test]
fn test_tokenize_numbers() {
    let mut lexer = Lexer::new("123 45.67");
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 3); // 2 numbers + EOF
    assert_eq!(tokens[0].token_type, TokenType::Integer(123));
    assert_eq!(tokens[1].token_type, TokenType::Number(45.67));
    assert!(matches!(tokens[2].token_type, TokenType::Eof));
}

#[test]
fn test_tokenize_operators() {
    let mut lexer = Lexer::new("+ - * / =");
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 6); // 5 operators + EOF
    assert_eq!(tokens[0].token_type, TokenType::Plus);
    assert_eq!(tokens[1].token_type, TokenType::Minus);
    assert_eq!(tokens[2].token_type, TokenType::Multiply);
    assert_eq!(tokens[3].token_type, TokenType::Divide);
    assert_eq!(tokens[4].token_type, TokenType::Assign);
    assert!(matches!(tokens[5].token_type, TokenType::Eof));
}

#[test]
fn test_tokenize_identifiers() {
    let mut lexer = Lexer::new("x var_name _underscore");
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 4); // 3 identifiers + EOF
    assert_eq!(tokens[0].token_type, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Identifier("var_name".to_string()));
    assert_eq!(tokens[2].token_type, TokenType::Identifier("_underscore".to_string()));
    assert!(matches!(tokens[3].token_type, TokenType::Eof));
}

#[test]
fn test_tokenize_delimiters() {
    let mut lexer = Lexer::new("( ) ;");
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 4); // 3 delimiters + EOF
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::RightParen);
    assert_eq!(tokens[2].token_type, TokenType::Semicolon);
    assert!(matches!(tokens[3].token_type, TokenType::Eof));
}

#[test]
fn test_tokenize_expression() {
    let mut lexer = Lexer::new("x = 10 + 5 * 2;");
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 9); // 8 tokens + EOF
    assert_eq!(tokens[0].token_type, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Assign);
    assert_eq!(tokens[2].token_type, TokenType::Integer(10));
    assert_eq!(tokens[3].token_type, TokenType::Plus);
    assert_eq!(tokens[4].token_type, TokenType::Integer(5));
    assert_eq!(tokens[5].token_type, TokenType::Multiply);
    assert_eq!(tokens[6].token_type, TokenType::Integer(2));
    assert_eq!(tokens[7].token_type, TokenType::Semicolon);
    assert!(matches!(tokens[8].token_type, TokenType::Eof));
}

#[test]
fn test_tokenize_whitespace_handling() {
    let mut lexer = Lexer::new("  x   =   42  ");
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 4); // 3 tokens + EOF
    assert_eq!(tokens[0].token_type, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Assign);
    assert_eq!(tokens[2].token_type, TokenType::Integer(42));
    assert!(matches!(tokens[3].token_type, TokenType::Eof));
}

#[test]
fn test_tokenize_illegal_character() {
    let mut lexer = Lexer::new("x @ 5");
    let result = lexer.tokenize();
    
    assert!(result.is_err());
    if let Err(BcclError::LexError { message, .. }) = result {
        assert!(message.contains("Invalid character '@'"));
    } else {
        panic!("Expected LexError for invalid character");
    }
}

#[test]
fn test_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 1); // Only EOF
    assert!(matches!(tokens[0].token_type, TokenType::Eof));
}

#[test]
fn test_tokenize_strings() {
    let mut lexer = Lexer::new(r#""hello" "world with spaces""#);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 3); // 2 strings + EOF
    assert_eq!(tokens[0].token_type, TokenType::String("hello".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::String("world with spaces".to_string()));
    assert!(matches!(tokens[2].token_type, TokenType::Eof));
}

#[test]
fn test_tokenize_compound_operators() {
    let mut lexer = Lexer::new("+= -= *= /= == != <= >= and or not in");
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 13); // 12 operators + EOF
    assert_eq!(tokens[0].token_type, TokenType::PlusAssign);
    assert_eq!(tokens[1].token_type, TokenType::MinusAssign);
    assert_eq!(tokens[2].token_type, TokenType::MultiplyAssign);
    assert_eq!(tokens[3].token_type, TokenType::DivideAssign);
    assert_eq!(tokens[4].token_type, TokenType::Equal);
    assert_eq!(tokens[5].token_type, TokenType::NotEqual);
    assert_eq!(tokens[6].token_type, TokenType::LessEqual);
    assert_eq!(tokens[7].token_type, TokenType::GreaterEqual);
    assert_eq!(tokens[8].token_type, TokenType::And);
    assert_eq!(tokens[9].token_type, TokenType::Or);
    assert_eq!(tokens[10].token_type, TokenType::Not);
    assert_eq!(tokens[11].token_type, TokenType::In);
    assert!(matches!(tokens[12].token_type, TokenType::Eof));
}

#[test]
fn test_tokenize_collections() {
    let mut lexer = Lexer::new(r#"[1, 2, 3] {"key": "value"}"#);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens.len(), 13); // [ 1 , 2 , 3 ] { "key" : "value" } EOF
    assert_eq!(tokens[0].token_type, TokenType::LeftBracket);
    assert_eq!(tokens[1].token_type, TokenType::Integer(1));
    assert_eq!(tokens[2].token_type, TokenType::Comma);
    assert_eq!(tokens[3].token_type, TokenType::Integer(2));
    assert_eq!(tokens[4].token_type, TokenType::Comma);
    assert_eq!(tokens[5].token_type, TokenType::Integer(3));
    assert_eq!(tokens[6].token_type, TokenType::RightBracket);
    assert_eq!(tokens[7].token_type, TokenType::LeftBrace);
    assert_eq!(tokens[8].token_type, TokenType::String("key".to_string()));
    assert_eq!(tokens[9].token_type, TokenType::Colon);
    assert_eq!(tokens[10].token_type, TokenType::String("value".to_string()));
    assert_eq!(tokens[11].token_type, TokenType::RightBrace);
    assert!(matches!(tokens[12].token_type, TokenType::Eof));
}