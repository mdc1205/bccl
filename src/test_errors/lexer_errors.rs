/// Test lexer error scenarios
use crate::{Lexer, BcclError};

#[test]
fn test_invalid_character_error() {
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
fn test_malformed_number_error() {
    let mut lexer = Lexer::new("12.34.56");
    let result = lexer.tokenize();
    
    assert!(result.is_err());
    if let Err(BcclError::MalformedNumber { message, .. }) = result {
        assert!(message.contains("12.34.56"));
    } else {
        panic!("Expected MalformedNumber error for double decimal");
    }
}

#[test]
fn test_multiple_invalid_characters() {
    let test_cases = vec![
        ("x # y", '#'),
        ("a $ b", '$'),
        ("test & value", '&'),
        ("foo % bar", '%'),
    ];

    for (input, expected_char) in test_cases {
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();
        
        assert!(result.is_err(), "Should error on input: {}", input);
        if let Err(BcclError::LexError { message, .. }) = result {
            assert!(message.contains(&format!("Invalid character '{}'", expected_char)));
        }
    }
}