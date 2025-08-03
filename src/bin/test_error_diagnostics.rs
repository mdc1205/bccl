use bccl::{Lexer, Parser, Evaluator};
use miette::{IntoDiagnostic, Result};

fn test_error_diagnostic(name: &str, input: &str) {
    println!("\n=== {} ===", name);
    println!("Input: {}", input);
    
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("Lexer Error Diagnostic:");
            let report = miette::Report::new(e).with_source_code(input.to_string());
            println!("{:?}", report);
            return;
        }
    };
    
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            println!("Parser Error Diagnostic:");
            let report = miette::Report::new(e).with_source_code(input.to_string());
            println!("{:?}", report);
            return;
        }
    };
    
    let mut evaluator = Evaluator::new();
    match evaluator.evaluate_program(&program) {
        Ok(_) => {
            println!("❌ Expected error but execution succeeded");
        }
        Err(e) => {
            println!("Runtime Error Diagnostic:");
            let report = miette::Report::new(e).with_source_code(input.to_string());
            println!("{:?}", report);
        }
    }
}

fn main() -> Result<()> {
    println!("BCCL Error Diagnostics Testing");
    println!("==============================");
    
    // Test compound assignment errors
    test_error_diagnostic(
        "Compound Assignment - Undefined Variable",
        "undefined_var += 5"
    );
    
    test_error_diagnostic(
        "Compound Assignment - Division by Zero", 
        "x = 10; x /= 0"
    );
    
    test_error_diagnostic(
        "Compound Assignment - Type Error",
        "x = \"hello\"; x += 5"
    );
    
    // Test logical operator errors
    test_error_diagnostic(
        "Comparison - Type Mismatch",
        "5 < \"hello\""
    );
    
    test_error_diagnostic(
        "Membership - Wrong Collection Type",
        "5 in 10"
    );
    
    test_error_diagnostic(
        "Membership - Wrong Key Type for Dictionary",
        "5 in {\"name\": \"John\"}"
    );
    
    // Test collection errors
    test_error_diagnostic(
        "List Index Out of Bounds",
        "[1, 2, 3][10]"
    );
    
    test_error_diagnostic(
        "Dictionary Key Not Found",
        "{\"a\": 1}[\"nonexistent\"]"
    );
    
    test_error_diagnostic(
        "List Index Wrong Type",
        "[1, 2, 3][\"string\"]"
    );
    
    test_error_diagnostic(
        "Dictionary Key Wrong Type",
        "{\"a\": 1}[123]"
    );
    
    // Test function call errors
    test_error_diagnostic(
        "Function Wrong Argument Count",
        "max(1)"
    );
    
    test_error_diagnostic(
        "Function Undefined",
        "unknown_function(1, 2)"
    );
    
    test_error_diagnostic(
        "Function Argument Type Error",
        "max(\"hello\", \"world\")"
    );
    
    // Test parse errors for new syntax
    test_error_diagnostic(
        "Unclosed List",
        "[1, 2, 3"
    );
    
    test_error_diagnostic(
        "Unclosed Dictionary",
        "{\"key\": \"value\""
    );
    
    test_error_diagnostic(
        "Invalid Dictionary Key",
        "{123: \"value\"}"
    );
    
    test_error_diagnostic(
        "Missing Dictionary Value",
        "{\"key\":}"
    );
    
    // Test complex nested errors
    test_error_diagnostic(
        "Nested Collection Error",
        "data = {\"users\": [{\"name\": \"Alice\"}]}; data[\"users\"][0][\"age\"]"
    );
    
    test_error_diagnostic(
        "Complex Logical Expression Error",
        "result = (5 > \"text\") and (3 < 7)"
    );
    
    Ok(())
}