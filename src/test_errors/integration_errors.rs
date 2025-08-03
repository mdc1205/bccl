/// Test complex error scenarios that combine multiple components
use crate::{Lexer, Parser, Evaluator, BcclError};

#[test]
fn test_complex_expression_with_undefined_variable() {
    let mut lexer = Lexer::new("(x + 5) * (y - 2)");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    // Should fail on first undefined variable (x)
    if let Err(BcclError::UndefinedVariable { name, .. }) = result {
        assert_eq!(name, "x");
    } else {
        panic!("Expected UndefinedVariable error for x");
    }
}

#[test]
fn test_nested_division_by_zero() {
    let mut lexer = Lexer::new("10 / (5 - 5)");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::DivisionByZero { .. }) = result {
        // Success
    } else {
        panic!("Expected DivisionByZero error in nested expression");
    }
}

#[test]
fn test_assignment_with_error_in_value() {
    let mut lexer = Lexer::new("result = 10 / undefined_var");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    // Should fail due to undefined variable, not assignment
    if let Err(BcclError::UndefinedVariable { name, .. }) = result {
        assert_eq!(name, "undefined_var");
    } else {
        panic!("Expected UndefinedVariable error in assignment value");
    }
    
    // Verify that the assignment didn't happen
    assert!(evaluator.get_variable("result").is_none());
}

#[test]
fn test_nested_collection_errors() {
    let mut lexer = Lexer::new(r#"data = {"items": [1, 2, 3]}; data["items"][10]"#);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    if let Err(BcclError::IndexOutOfBounds { .. }) = result {
        // Success
    } else {
        panic!("Expected IndexOutOfBounds error in nested collection access");
    }
}

#[test]
fn test_function_with_wrong_args() {
    let mut lexer = Lexer::new("max(1)");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    // With our new function signature system, providing insufficient args
    // results in a MissingParameter error rather than WrongArgumentCount
    match result {
        Err(BcclError::FunctionArgumentError { function_name, .. }) => {
            assert_eq!(function_name, "max");
        }
        Err(BcclError::WrongArgumentCount { function_name, expected, actual, .. }) => {
            assert_eq!(function_name, "max");
            assert_eq!(expected, 2);
            assert_eq!(actual, 1);
        }
        other => {
            panic!("Expected FunctionArgumentError or WrongArgumentCount error, got: {:?}", other);
        }
    }
}