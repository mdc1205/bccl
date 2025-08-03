use crate::{Lexer, Parser, Evaluator, ErrorContext};
use miette::{GraphicalReportHandler, GraphicalTheme};

/// Demo function to show various error formatting examples
pub fn demo_errors() {
    println!("=== Error Formatting Demo ===\n");
    
    let examples = vec![
        ("(5 + 3", "Missing closing parenthesis"),
        ("12.34.56", "Malformed number"),
        ("x + undefined_var", "Undefined variable"),
        ("5 / 0", "Division by zero"),
        ("5 +", "Unexpected end of input"),
    ];
    
    for (input, description) in examples {
        println!("Example: {} ({})", input, description);
        println!("Input: {}", input);
        
        let context = ErrorContext::new(input.to_string());
        let mut evaluator = Evaluator::new();
        
        if let Err(error) = evaluate_demo_input(input, &mut evaluator) {
            println!("Error:");
            let report = miette::Report::new(error).with_source_code(context.source);
            let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode());
            let mut output = String::new();
            if handler.render_report(&mut output, report.as_ref()).is_ok() {
                println!("{}", output);
            } else {
                eprintln!("{}", report);
            }
        }
        
        println!();
    }
}

fn evaluate_demo_input(input: &str, evaluator: &mut Evaluator) -> crate::BcclResult<()> {
    // Tokenize
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    
    // Parse
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    // Evaluate
    evaluator.evaluate_program(&program)?;
    
    Ok(())
}