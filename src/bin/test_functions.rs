use bccl::lexer::Lexer;
use bccl::parser::Parser;
use bccl::evaluator::Evaluator;

fn main() {
    let test_cases = vec![
        "max(5, 10)",
        "max(3.14, 2)",
        "min(5, 10)", 
        "min(3.14, 2)",
        "result = max(15, 7); result",
    ];

    let mut evaluator = Evaluator::new();

    for (i, input) in test_cases.iter().enumerate() {
        println!("Test {}: {}", i + 1, input);
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        match evaluator.evaluate_program(&program) {
            Ok(Some(value)) => println!("  Result: {}", value.display()),
            Ok(None) => println!("  Result: (no value)"),
            Err(e) => println!("  Error: {}", e),
        }
        println!();
    }
}