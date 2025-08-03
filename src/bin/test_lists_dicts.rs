use bccl::{Lexer, Parser, Evaluator};

fn test_expression(input: &str) {
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
        Ok(Some(value)) => println!("  Result: {}", value.display()),
        Ok(None) => println!("  No result"),
        Err(e) => println!("  Evaluation error: {:?}", e),
    }
    println!();
}

fn main() {
    println!("Testing Lists and Dictionaries Implementation\n");
    
    // Test basic lists
    test_expression("[1, 2, 3]");
    test_expression("[]");
    test_expression("[42]");
    test_expression("[1, 2, 3, 4, 5]");
    
    // Test basic dictionaries
    test_expression(r#"{"name": "John"}"#);
    test_expression("{}");
    test_expression(r#"{"key1": "value1", "key2": "value2"}"#);
    
    // Test indexing
    test_expression("[1, 2, 3][0]");
    test_expression("[1, 2, 3][2]");
    test_expression(r#"{"name": "Alice"}["name"]"#);
    
    // Test with variables
    println!("Testing with variables:");
    let mut evaluator = Evaluator::new();
    
    // Test list assignment
    let input = "nums = [10, 20, 30]";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    evaluator.evaluate_program(&program).unwrap();
    
    // Test list indexing with variable
    let input = "nums[1]";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let result = evaluator.evaluate_program(&program).unwrap();
    println!("nums[1] = {}", result.unwrap().display());
    
    // Test dictionary assignment
    let input = r#"person = {"name": "Bob", "age": 25}"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    evaluator.evaluate_program(&program).unwrap();
    
    // Test dictionary indexing with variable
    let input = r#"person["name"]"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let result = evaluator.evaluate_program(&program).unwrap();
    println!(r#"person["name"] = {}"#, result.unwrap().display());
    
    println!("\nTesting error cases:");
    
    // Test index out of bounds
    test_expression("[1, 2][5]");
    
    // Test key not found
    test_expression(r#"{"a": 1}["b"]"#);
    
    // Test wrong index type
    test_expression(r#"[1, 2]["invalid"]"#);
    test_expression(r#"{"a": 1}[0]"#);
}