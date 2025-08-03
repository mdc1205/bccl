use std::io::{self, Write};
use bccl::{Lexer, Parser, Evaluator, ErrorContext};
use miette::{IntoDiagnostic, Result, GraphicalReportHandler, GraphicalTheme};

fn strip_ansi_codes(input: &str) -> String {
    // Remove ANSI escape sequences for consistent output
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(input, "").to_string()
}

fn main() -> Result<()> {
    println!("BCCL Interpreter v0.2.0 - Enhanced Error Diagnostics");
    println!("Type expressions and assignments. Use Ctrl+C to exit.\n");
    
    // Show some examples
    println!("Examples:");
    println!("  x = 42        # Variable assignment");
    println!("  y = x * 2     # Using variables");
    println!("  (3 + 4) * 5   # Complex expressions");
    println!("  z            # Show variable value");
    println!();
    
    let mut evaluator = Evaluator::new();
    
    loop {
        print!("> ");
        io::stdout().flush().into_diagnostic()?;
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                
                // Special commands
                if input == ":help" {
                    show_help();
                    continue;
                }
                if input == ":vars" {
                    show_variables(&evaluator);
                    continue;
                }
                if input == ":clear" {
                    evaluator = Evaluator::new();
                    println!("Variables cleared.");
                    continue;
                }
                if input == ":quit" || input == ":exit" {
                    break;
                }
                if input == ":demo" {
                    bccl::demo_errors::demo_errors();
                    continue;
                }
                
                // Create error context for rich diagnostics
                let context = ErrorContext::new(input.to_string());
                
                // Evaluate the input with comprehensive error handling
                if let Err(error) = evaluate_input(input, &mut evaluator) {
                    // Use miette to display rich error diagnostics with proper formatting
                    let report = miette::Report::new(error).with_source_code(context.source);
                    
                    // Configure miette for consistent output regardless of execution context
                    // Always use ASCII theme for consistent display across different execution contexts
                    let theme = GraphicalTheme::ascii();
                    
                    let handler = GraphicalReportHandler::new_themed(theme);
                    let mut output = String::new();
                    match handler.render_report(&mut output, report.as_ref()) {
                        Ok(()) => {
                            // Strip ANSI color codes for consistent output
                            let clean_output = strip_ansi_codes(&output);
                            eprint!("{}", clean_output);
                        }
                        Err(_) => {
                            // Simple fallback that should always work
                            eprintln!("{}", report);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

fn evaluate_input(input: &str, evaluator: &mut Evaluator) -> bccl::BcclResult<()> {
    // Tokenize
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    
    // Parse
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    // Evaluate
    match evaluator.evaluate_program(&program)? {
        Some(value) => println!("{}", value.display()),
        None => {}
    }
    
    Ok(())
}

fn show_help() {
    println!("BCCL Commands:");
    println!("  :help     - Show this help message");
    println!("  :vars     - Show all defined variables");
    println!("  :clear    - Clear all variables");
    println!("  :demo     - Show error formatting examples");
    println!("  :quit     - Exit the interpreter");
    println!("  :exit     - Exit the interpreter");
    println!();
    println!("Syntax:");
    println!("  Numbers:     42, 3.14, -5");
    println!("  Variables:   x, my_var, _private");
    println!("  Assignment:  x = 10");
    println!("  Operators:   +, -, *, /");
    println!("  Grouping:    (expression)");
    println!();
}

fn show_variables(evaluator: &Evaluator) {
    let var_names = evaluator.get_variable_names();
    if var_names.is_empty() {
        println!("No variables defined.");
    } else {
        println!("Variables:");
        for name in var_names {
            if let Some(value) = evaluator.get_variable(&name) {
                println!("  {} = {}", name, value.display());
            }
        }
    }
}
