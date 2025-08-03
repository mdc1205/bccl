/// Test evaluator error scenarios
use crate::{Lexer, Parser, Evaluator, BcclError};

#[test]
fn test_undefined_variable_error() {
    let mut lexer = Lexer::new("undefined_var");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::UndefinedVariable { name, suggestion, .. }) = result {
        assert_eq!(name, "undefined_var");
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("No variables are currently defined"));
    } else {
        panic!("Expected UndefinedVariable error");
    }
}

#[test]
fn test_undefined_variable_with_suggestions() {
    let mut evaluator = Evaluator::new();
    
    // Define some variables
    let mut lexer = Lexer::new("variable1 = 10; variable2 = 20");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    evaluator.evaluate_program(&program).unwrap();
    
    // Now try to use a similar but undefined variable
    let mut lexer = Lexer::new("variable3");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::UndefinedVariable { name, suggestion, .. }) = result {
        assert_eq!(name, "variable3");
        assert!(suggestion.is_some());
        let suggestion_text = suggestion.unwrap();
        assert!(suggestion_text.contains("variable1") || suggestion_text.contains("variable2"));
    }
}

#[test]
fn test_division_by_zero_error() {
    let mut lexer = Lexer::new("10 / 0");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::DivisionByZero { .. }) = result {
        // Success - got the expected error
    } else {
        panic!("Expected DivisionByZero error");
    }
}

#[test]
fn test_division_by_zero_with_variable() {
    let mut evaluator = Evaluator::new();
    
    // Set up zero variable
    let mut lexer = Lexer::new("zero = 0");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    evaluator.evaluate_program(&program).unwrap();
    
    // Try division by zero variable
    let mut lexer = Lexer::new("42 / zero");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::DivisionByZero { .. }) = result {
        // Success
    } else {
        panic!("Expected DivisionByZero error with variable");
    }
}

#[test]
fn test_compound_assignment_error() {
    let mut lexer = Lexer::new("undefined_var += 5");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::UndefinedVariable { name, .. }) = result {
        assert_eq!(name, "undefined_var");
    } else {
        panic!("Expected UndefinedVariable error for compound assignment");
    }
}

#[test]
fn test_type_error_in_arithmetic() {
    let mut lexer = Lexer::new(r#"5 + "hello""#);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::TypeError { .. }) = result {
        // Success - got the expected error
    } else {
        panic!("Expected TypeError for mixed types in arithmetic");
    }
}

#[test]
fn test_index_out_of_bounds() {
    let mut lexer = Lexer::new("[1, 2, 3][10]");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::IndexOutOfBounds { index, length, .. }) = result {
        assert_eq!(index, 10);
        assert_eq!(length, 3);
    } else {
        panic!("Expected IndexOutOfBounds error");
    }
}

#[test]
fn test_key_not_found() {
    let mut lexer = Lexer::new(r#"{"name": "John"}["age"]"#);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::KeyNotFound { key, .. }) = result {
        assert_eq!(key, "age");
    } else {
        panic!("Expected KeyNotFound error");
    }
}