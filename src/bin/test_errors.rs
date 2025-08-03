use bccl::{Lexer, Parser, Evaluator, ErrorContext};
use miette::{GraphicalReportHandler, GraphicalTheme};

fn main() {
    println!("Testing basic data types...\n");
    
    let test_inputs = vec![
        "42",            // Integer
        "3.14",          // Float 
        "true",          // Boolean true
        "false",         // Boolean false
        "\"hello\"",     // String
        "x = 10",        // Integer assignment
        "y = \"test\"",  // String assignment
        "z = true",      // Boolean assignment
        "x == 10",       // Integer equality (will fail - x not defined in this scope)
        "10 == 10",      // Integer equality
        "\"test\" == \"test\"", // String equality
        "true != false", // Boolean inequality
        "10 == 10.0",    // Mixed number comparison
    ];
    
    for test_input in test_inputs {
        println!("Input: {}", test_input);
        
        let context = ErrorContext::new(test_input.to_string());
        let mut evaluator = Evaluator::new();
        
        match evaluate_input(test_input, &mut evaluator) {
            Ok(result) => {
                if let Some(result) = result {
                    println!("Result: {}", result);
                } else {
                    println!("Assignment completed");
                }
            }
            Err(error) => {
                println!("Error occurred:");
                let report = miette::Report::new(error).with_source_code(context.source);
                
                // Try to force graphical output
                let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode());
                let mut output = String::new();
                handler.render_report(&mut output, report.as_ref()).unwrap();
                println!("{}", output);
            }
        }
        println!("---\n");
    }
}

fn evaluate_input(input: &str, evaluator: &mut Evaluator) -> bccl::BcclResult<Option<String>> {
    // Tokenize
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    
    // Parse
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    // Evaluate
    match evaluator.evaluate_program(&program)? {
        Some(value) => Ok(Some(value.display())),
        None => Ok(None),
    }
}