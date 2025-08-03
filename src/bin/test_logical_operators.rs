use bccl::{Lexer, Parser, Evaluator};

fn test_expression(input: &str, expected_result: &str) {
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
            let result = value.display();
            println!("  Result: {}", result);
            if result == expected_result {
                println!("  ✅ PASS");
            } else {
                println!("  ❌ FAIL: Expected {}, got {}", expected_result, result);
            }
        }
        Ok(None) => println!("  No result"),
        Err(e) => println!("  Evaluation error: {:?}", e),
    }
    println!();
}

fn test_error_case(input: &str, description: &str) {
    println!("Testing error case: {} ({})", input, description);
    
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
        Ok(_) => println!("  ❌ FAIL: Should have failed"),
        Err(e) => println!("  ✅ PASS: Error - {:?}", e),
    }
    println!();
}

fn main() {
    println!("Testing Logical Operators\n");
    
    // Test logical AND
    println!("=== Logical AND ===");
    test_expression("true and true", "true");
    test_expression("true and false", "false");
    test_expression("false and true", "false");
    test_expression("false and false", "false");
    
    // Test short-circuit evaluation for AND
    test_expression("false and 42", "false");
    test_expression("true and 42", "42");
    test_expression("0 and 5", "0");
    test_expression("5 and 10", "10");
    
    // Test logical OR
    println!("=== Logical OR ===");
    test_expression("true or true", "true");
    test_expression("true or false", "true");
    test_expression("false or true", "true");
    test_expression("false or false", "false");
    
    // Test short-circuit evaluation for OR
    test_expression("true or 42", "true");
    test_expression("false or 42", "42");
    test_expression("5 or 10", "5");
    test_expression("0 or 10", "10");
    
    // Test logical NOT
    println!("=== Logical NOT ===");
    test_expression("not true", "false");
    test_expression("not false", "true");
    test_expression("not 0", "true");
    test_expression("not 5", "false");
    test_expression("not \"\"", "true");
    test_expression("not \"hello\"", "false");
    test_expression("not []", "true");
    test_expression("not [1, 2]", "false");
    test_expression("not {}", "true");
    test_expression(r#"not {"a": 1}"#, "false");
    
    // Test comparison operators
    println!("=== Comparison Operators ===");
    test_expression("5 < 10", "true");
    test_expression("10 < 5", "false");
    test_expression("5 > 10", "false");
    test_expression("10 > 5", "true");
    test_expression("5 <= 5", "true");
    test_expression("5 <= 10", "true");
    test_expression("10 <= 5", "false");
    test_expression("5 >= 5", "true");
    test_expression("10 >= 5", "true");
    test_expression("5 >= 10", "false");
    
    // Test membership operators
    println!("=== Membership Operators ===");
    test_expression("1 in [1, 2, 3]", "true");
    test_expression("4 in [1, 2, 3]", "false");
    test_expression("1 not in [1, 2, 3]", "false");
    test_expression("4 not in [1, 2, 3]", "true");
    
    test_expression(r#""name" in {"name": "John", "age": 30}"#, "true");
    test_expression(r#""city" in {"name": "John", "age": 30}"#, "false");
    test_expression(r#""name" not in {"name": "John", "age": 30}"#, "false");
    test_expression(r#""city" not in {"name": "John", "age": 30}"#, "true");
    
    // Test complex expressions
    println!("=== Complex Expressions ===");
    test_expression("true and (false or true)", "true");
    test_expression("false or (true and false)", "false");
    test_expression("not (true and false)", "true");
    test_expression("not (false or false)", "true");
    
    // Test with variables
    println!("=== With Variables ===");
    test_expression("x = 5; y = 10; x < y and y > 0", "true");
    test_expression("a = true; b = false; a or b", "true");
    test_expression("nums = [1, 2, 3]; 2 in nums", "true");
    test_expression(r#"person = {"name": "Alice"}; "name" in person"#, "true");
    
    // Test operator precedence
    println!("=== Operator Precedence ===");
    test_expression("true or false and false", "true"); // or has lower precedence than and
    test_expression("false and true or true", "true");  // left-to-right evaluation
    test_expression("not false and true", "true");      // not has higher precedence
    test_expression("5 > 3 and 2 < 4", "true");         // comparison has higher precedence than logical
    
    // Test error cases
    println!("=== Error Cases ===");
    test_error_case(r#"5 < "hello""#, "comparison with non-number");
    test_error_case(r#""hello" > 10"#, "comparison with non-number");
    test_error_case("5 in 10", "membership with non-collection");
    test_error_case(r#"5 in {"name": "John"}"#, "non-string key in dictionary");
    
    println!("Logical operators testing complete!");
}