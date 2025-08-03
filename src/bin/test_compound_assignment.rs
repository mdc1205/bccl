use bccl::{Lexer, Parser, Evaluator};

fn test_compound_assignment(input: &str, expected: f64) {
    println!("Testing: {}", input);
    
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("  Lexer error: {:?}", e);
            return;
        }
    };
    
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            println!("  Parser error: {:?}", e);
            return;
        }
    };
    
    let mut evaluator = Evaluator::new();
    match evaluator.evaluate_program(&program) {
        Ok(Some(value)) => {
            println!("  Result: {}", value.display());
            if let Some(num) = value.as_number() {
                if (num - expected).abs() < f64::EPSILON {
                    println!("  ✅ PASS");
                } else {
                    println!("  ❌ FAIL: Expected {}, got {}", expected, num);
                }
            } else {
                println!("  ❌ FAIL: Expected number, got {}", value.type_name());
            }
        }
        Ok(None) => println!("  No result"),
        Err(e) => println!("  Evaluation error: {:?}", e),
    }
    println!();
}

fn test_compound_sequence() {
    println!("Testing compound assignment sequence:");
    let mut evaluator = Evaluator::new();
    
    let test_cases = vec![
        ("x = 10", 10.0),
        ("x += 5", 15.0),
        ("x -= 3", 12.0),
        ("x *= 2", 24.0),
        ("x /= 4", 6.0),
    ];
    
    for (input, expected) in test_cases {
        println!("  {}", input);
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        match evaluator.evaluate_program(&program) {
            Ok(Some(value)) => {
                println!("    Result: {}", value.display());
                if let Some(num) = value.as_number() {
                    if (num - expected).abs() < f64::EPSILON {
                        println!("    ✅ PASS");
                    } else {
                        println!("    ❌ FAIL: Expected {}, got {}", expected, num);
                    }
                } else {
                    println!("    ❌ FAIL: Expected number, got {}", value.type_name());
                }
            }
            Ok(None) => println!("    No result"),
            Err(e) => println!("    Evaluation error: {:?}", e),
        }
    }
    println!();
}

fn test_error_cases() {
    println!("Testing error cases:");
    
    // Test undefined variable
    println!("  Undefined variable:");
    let mut lexer = Lexer::new("undefined_var += 5");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    match evaluator.evaluate_program(&program) {
        Ok(_) => println!("    ❌ FAIL: Should have failed"),
        Err(e) => println!("    ✅ PASS: Error - {:?}", e),
    }
    
    // Test division by zero
    println!("  Division by zero:");
    let mut evaluator = Evaluator::new();
    
    // Set up variable
    let mut lexer = Lexer::new("x = 10");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    evaluator.evaluate_program(&program).unwrap();
    
    // Test division by zero
    let mut lexer = Lexer::new("x /= 0");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    match evaluator.evaluate_program(&program) {
        Ok(_) => println!("    ❌ FAIL: Should have failed"),
        Err(e) => println!("    ✅ PASS: Error - {:?}", e),
    }
    
    println!();
}

fn main() {
    println!("Testing Compound Assignment Operators\n");
    
    // Test individual compound assignments
    test_compound_assignment("x = 10; x += 5; x", 15.0);
    test_compound_assignment("y = 20; y -= 8; y", 12.0);
    test_compound_assignment("z = 3; z *= 4; z", 12.0);
    test_compound_assignment("w = 15; w /= 3; w", 5.0);
    
    // Test compound assignment with expressions
    test_compound_assignment("a = 5; a += 2 * 3; a", 11.0);
    test_compound_assignment("b = 10; b -= (3 + 2); b", 5.0);
    test_compound_assignment("c = 2; c *= (3 + 4); c", 14.0);
    test_compound_assignment("d = 20; d /= (2 + 2); d", 5.0);
    
    // Test sequence of compound operations
    test_compound_sequence();
    
    // Test error cases
    test_error_cases();
    
    println!("Compound assignment testing complete!");
}