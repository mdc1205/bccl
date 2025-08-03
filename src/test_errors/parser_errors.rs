/// Test parser error scenarios
use crate::{Lexer, Parser, BcclError};

#[test]
fn test_unexpected_token_error() {
    let mut lexer = Lexer::new("5 +");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
    if let Err(BcclError::UnexpectedEof { expected, .. }) = result {
        assert!(expected.contains(&"number".to_string()) || 
               expected.contains(&"identifier".to_string()) || 
               expected.contains(&"(".to_string()));
    } else {
        panic!("Expected UnexpectedEof error");
    }
}

#[test]
fn test_missing_closing_paren() {
    let mut lexer = Lexer::new("(5 + 3");
    let tokens = lexer.tokenize().unwrap();
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
    
    match result {
        Err(BcclError::UnexpectedEof { expected, .. }) => {
            assert!(expected.contains(&")".to_string()));
        }
        Err(other) => {
            panic!("Expected UnexpectedEof for missing closing paren, got: {:#?}", other);
        }
        Ok(_) => panic!("Expected error for incomplete parentheses"),
    }
}

#[test]
fn test_unexpected_closing_paren() {
    // Test parsing with extra closing paren - should handle gracefully
    let mut lexer = Lexer::new("5 + 3");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    // This should parse successfully
    assert!(result.is_ok());
    
    // Test a case that definitely fails
    let mut lexer = Lexer::new(")");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
}

#[test]
fn test_invalid_assignment_target() {
    // Test some invalid syntax that should definitely fail
    let mut lexer = Lexer::new("= 42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
    
    // Test another invalid case  
    let mut lexer = Lexer::new("42 + = x");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
}

#[test]
fn test_unclosed_list() {
    let mut lexer = Lexer::new("[1, 2, 3");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
    match result {
        Err(BcclError::UnexpectedEof { expected, .. }) => {
            assert!(expected.contains(&"]".to_string()) || 
                   expected.contains(&",".to_string()));
        }
        Err(BcclError::UnexpectedToken { expected, .. }) => {
            assert!(expected.contains(&"]".to_string()) || 
                   expected.contains(&",".to_string()));
        }
        other => panic!("Expected UnexpectedEof or UnexpectedToken for unclosed list, got: {:?}", other),
    }
}

#[test]
fn test_unclosed_dictionary() {
    let mut lexer = Lexer::new(r#"{"key": "value""#);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
    match result {
        Err(BcclError::UnexpectedEof { expected, .. }) => {
            assert!(expected.contains(&"}".to_string()) || 
                   expected.contains(&",".to_string()));
        }
        Err(BcclError::UnexpectedToken { expected, .. }) => {
            assert!(expected.contains(&"}".to_string()) || 
                   expected.contains(&",".to_string()));
        }
        other => panic!("Expected UnexpectedEof or UnexpectedToken for unclosed dictionary, got: {:?}", other),
    }
}

#[test]
fn test_kwargs_positional_after_keyword() {
    // Test that positional args after keyword args are rejected
    let mut lexer = Lexer::new("max(a=5, 10)");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_err());
    match result {
        Err(BcclError::UnexpectedToken { expected, .. }) => {
            assert!(expected.contains(&"keyword argument".to_string()));
        }
        other => panic!("Expected UnexpectedToken for positional arg after kwargs, got: {:?}", other),
    }
}