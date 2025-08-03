/// Comprehensive error testing module for BCCL interpreter
/// 
/// This module contains test cases that demonstrate the enhanced error handling
/// and diagnostics capabilities of the BCCL interpreter.

use crate::{Lexer, Parser, Evaluator, BcclError};

mod lexer_errors;
mod parser_errors;
mod evaluator_errors;
mod integration_errors;
mod error_recovery;

// Basic error tests that don't fit into specific categories
#[test]
fn test_specific_paren_issue() {
    let mut lexer = Lexer::new("(5 + 3");
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => panic!("Lexer should not fail: {:?}", e),
    };
    
    println!("Tokens: {:?}", tokens);
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    println!("Parser result: {:?}", result);
    
    match result {
        Err(BcclError::UnexpectedEof { expected, .. }) => {
            assert!(expected.contains(&")".to_string()), "Should expect ')' but got: {:?}", expected);
        }
        other => panic!("Expected UnexpectedEof error, got: {:?}", other),
    }
}

#[test]
fn test_incomplete_plus_expression() {
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
        other => panic!("Expected UnexpectedEof error for incomplete expression, got: {:?}", other),
    }
}

#[test]
fn test_invalid_character() {
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
fn test_malformed_number() {
    let mut lexer = Lexer::new("12.34.56");
    let result = lexer.tokenize();
    
    match result {
        Err(BcclError::MalformedNumber { message, .. }) => {
            assert!(message.contains("12.34.56"));
        }
        other => panic!("Expected MalformedNumber error, got: {:?}", other),
    }
}