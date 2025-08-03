use bccl::lexer::Lexer;
use bccl::parser::Parser;
use bccl::evaluator::Evaluator;

fn main() {
    let test_cases = vec![
        "unknown_function(5)",
        "max(5)",              // wrong argument count
        "max(5, 10, 15)",      // wrong argument count  
        "max(\"hello\", 5)",   // wrong argument type
    ];

    let mut evaluator = Evaluator::new();

    for (i, input) in test_cases.iter().enumerate() {
        println!("Error Test {}: {}", i + 1, input);
        
        let mut lexer = Lexer::new(input);
        match lexer.tokenize() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(program) => {
                        match evaluator.evaluate_program(&program) {
                            Ok(Some(value)) => println!("  Unexpected Success: {}", value.display()),
                            Ok(None) => println!("  Unexpected Success: (no value)"),
                            Err(e) => println!("  Expected Error: {}", e),
                        }
                    }
                    Err(e) => println!("  Parse Error: {}", e),
                }
            }
            Err(e) => println!("  Lex Error: {}", e),
        }
        println!();
    }
}