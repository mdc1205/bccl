/// Test error recovery and state consistency
use crate::{Lexer, Parser, Evaluator};

#[test]
fn test_state_preservation_after_error() {
    let mut evaluator = Evaluator::new();
    
    // Successfully define a variable
    let mut lexer = Lexer::new("x = 42");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    evaluator.evaluate_program(&program).unwrap();
    
    // Attempt an operation that will fail
    let mut lexer = Lexer::new("y = x / 0");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    // Verify that x is still defined and y is not
    assert!(evaluator.get_variable("x").is_some());
    assert!(evaluator.get_variable("y").is_none());
    
    // Verify we can still use x
    let mut lexer = Lexer::new("z = x + 1");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_ok());
    assert!(evaluator.get_variable("z").is_some());
}

#[test]
fn test_error_recovery_with_compound_assignment() {
    let mut evaluator = Evaluator::new();
    
    // Define a variable
    let mut lexer = Lexer::new("counter = 10");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    evaluator.evaluate_program(&program).unwrap();
    
    // Try a compound assignment that should fail due to type error
    let mut lexer = Lexer::new(r#"counter += "invalid""#);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_err());
    
    // Verify that counter still has its original value
    match evaluator.get_variable("counter").unwrap() {
        crate::evaluator::Value::Integer(i) => assert_eq!(*i, 10),
        _ => panic!("Expected counter to still be 10"),
    }
    
    // Verify we can still perform valid operations
    let mut lexer = Lexer::new("counter += 5");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let result = evaluator.evaluate_program(&program);
    assert!(result.is_ok());
    
    match evaluator.get_variable("counter").unwrap() {
        crate::evaluator::Value::Number(n) => assert_eq!(*n, 15.0),
        _ => panic!("Expected counter to be 15 after successful compound assignment"),
    }
}