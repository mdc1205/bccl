//! Verification tests for the fixed error handling
//! These tests verify that the error examples in the documentation work correctly

use crate::{Lexer, Parser, Evaluator, BcclError};

#[test]
fn verify_missing_closing_paren() {
    let mut lexer = Lexer::new("(5 + 3");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    match result {
        Err(BcclError::UnexpectedEof { expected, .. }) => {
            assert!(expected.contains(&")".to_string()));
        }
        other => panic!("Expected UnexpectedEof for missing paren, got: {:?}", other),
    }
}

#[test]
fn verify_incomplete_expression() {
    let mut lexer = Lexer::new("5 +");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    match result {
        Err(BcclError::UnexpectedEof { expected, .. }) => {
            assert!(expected.contains(&"number".to_string()) || 
                   expected.contains(&"identifier".to_string()) || 
                   expected.contains(&"(".to_string()));
        }
        other => panic!("Expected UnexpectedEof for incomplete expression, got: {:?}", other),
    }
}

#[test]
fn verify_malformed_number() {
    let mut lexer = Lexer::new("12.34.56");
    let result = lexer.tokenize();
    
    match result {
        Err(BcclError::MalformedNumber { message, .. }) => {
            assert!(message.contains("12.34.56"));
        }
        other => panic!("Expected MalformedNumber error, got: {:?}", other),
    }
}

#[test]
fn verify_invalid_character() {
    let mut lexer = Lexer::new("x @ 5");
    let result = lexer.tokenize();
    
    match result {
        Err(BcclError::LexError { message, .. }) => {
            assert!(message.contains("Invalid character '@'"));
        }
        other => panic!("Expected LexError for invalid character, got: {:?}", other),
    }
}

#[test]
fn verify_undefined_variable() {
    let mut lexer = Lexer::new("undefined_var");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    match result {
        Err(BcclError::UndefinedVariable { name, .. }) => {
            assert_eq!(name, "undefined_var");
        }
        other => panic!("Expected UndefinedVariable error, got: {:?}", other),
    }
}

#[test]
fn verify_division_by_zero() {
    let mut lexer = Lexer::new("10 / 0");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    
    let result = evaluator.evaluate_program(&program);
    match result {
        Err(BcclError::DivisionByZero { .. }) => {
            // Success
        }
        other => panic!("Expected DivisionByZero error, got: {:?}", other),
    }
}