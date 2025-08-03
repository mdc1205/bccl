/// Comprehensive integration tests for BCCL interpreter
/// 
/// This module consolidates all the binary test files into proper cargo tests,
/// providing better organization and integration with the standard test framework.

use bccl::{Lexer, Parser, Evaluator, Value};

/// Helper function to evaluate BCCL code and return the result
fn eval_code(code: &str) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    let mut evaluator = Evaluator::new();
    Ok(evaluator.evaluate_program(&program)?)
}

/// Helper function to evaluate code and expect a specific numeric result
fn eval_number(code: &str, expected: f64) {
    match eval_code(code) {
        Ok(Some(value)) => {
            if let Some(num) = value.as_number() {
                assert!((num - expected).abs() < f64::EPSILON, 
                    "Expected {}, got {} for input: {}", expected, num, code);
            } else {
                panic!("Expected number, got {} for input: {}", value.type_name(), code);
            }
        }
        Ok(None) => panic!("Expected result but got None for input: {}", code),
        Err(e) => panic!("Evaluation failed for input '{}': {}", code, e),
    }
}

/// Helper function to evaluate code and expect an error
fn eval_error(code: &str) {
    match eval_code(code) {
        Ok(_) => panic!("Expected error but evaluation succeeded for input: {}", code),
        Err(_) => {} // Expected error
    }
}

/// Helper function to evaluate code with a persistent evaluator
fn eval_with_evaluator(evaluator: &mut Evaluator, code: &str) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    Ok(evaluator.evaluate_program(&program)?)
}

#[cfg(test)]
mod function_tests {
    use super::*;

    #[test]
    fn test_max_function() {
        eval_number("max(5, 10)", 10.0);
        eval_number("max(10, 5)", 10.0);
        eval_number("max(3.14, 2.7)", 3.14);
        eval_number("max(-5, -10)", -5.0);
        eval_number("max(0, 0)", 0.0);
    }

    #[test]
    fn test_min_function() {
        eval_number("min(5, 10)", 5.0);
        eval_number("min(10, 5)", 5.0);
        eval_number("min(3.14, 2.7)", 2.7);
        eval_number("min(-5, -10)", -10.0);
        eval_number("min(0, 0)", 0.0);
    }

    #[test]
    fn test_function_with_variables() {
        eval_number("result = max(15, 7); result", 15.0);
        eval_number("x = 5; y = 10; max(x, y)", 10.0);
        eval_number("a = min(3, 8); b = max(a, 10); b", 10.0);
    }

    #[test]
    fn test_function_kwargs() {
        eval_number("max(a=5, b=10)", 10.0);
        eval_number("max(5, b=10)", 10.0);
        eval_number("min(a=3, b=8)", 3.0);
        eval_number("min(3, b=8)", 3.0);
    }

    #[test]
    fn test_function_errors() {
        // Wrong argument count
        eval_error("max(5)");
        eval_error("max(5, 10, 15)");
        eval_error("min()");
        
        // Wrong argument types
        eval_error("max(5, true)");
        eval_error("min(\"hello\", 5)");
        
        // Unknown function
        eval_error("unknown_function(5)");
        
        // Invalid kwargs
        eval_error("max(x=5, y=10)");  // Unknown parameters
        eval_error("max(5, a=10)");    // Duplicate parameter
    }
}

#[cfg(test)]
mod variadic_function_tests {
    use super::*;

    #[test]
    fn test_sum_function_basic() {
        eval_number("sum(5)", 5.0);
        eval_number("sum(1, 2)", 3.0);
        eval_number("sum(1, 2, 3)", 6.0);
        eval_number("sum(1, 2, 3, 4, 5)", 15.0);
    }

    #[test]
    fn test_sum_function_mixed_types() {
        eval_number("sum(1, 2.5, 3)", 6.5);
        eval_number("sum(10, -5, 3.14)", 8.14);
        eval_number("sum(0, 0, 0)", 0.0);
    }

    #[test]
    fn test_sum_function_with_variables() {
        eval_number("x = 5; y = 10; sum(x, y, 15)", 30.0);
        eval_number("values = 42; sum(values, 8)", 50.0);
    }

    #[test]
    fn test_sum_function_kwargs() {
        eval_number("sum(values=5, values=10, values=15)", 30.0);
        eval_number("sum(5, values=10, values=15)", 30.0);
        eval_number("sum(values=1, values=2, values=3, values=4)", 10.0);
    }

    #[test]
    fn test_product_function_basic() {
        eval_number("product(5)", 5.0);
        eval_number("product(2, 3)", 6.0);
        eval_number("product(2, 3, 4)", 24.0);
        eval_number("product(1, 2, 3, 4, 5)", 120.0);
    }

    #[test]
    fn test_product_function_mixed_types() {
        eval_number("product(2, 2.5, 2)", 10.0);
        eval_number("product(1, -1, 5)", -5.0);
        eval_number("product(0, 5, 10)", 0.0);
    }

    #[test]
    fn test_product_function_with_variables() {
        eval_number("x = 2; y = 3; product(x, y, 4)", 24.0);
        eval_number("factor = 5; product(factor, 2)", 10.0);
    }

    #[test]
    fn test_product_function_kwargs() {
        eval_number("product(values=2, values=3, values=4)", 24.0);
        eval_number("product(2, values=3, values=4)", 24.0);
        eval_number("product(values=1, values=2, values=3)", 6.0);
    }

    #[test]
    fn test_variadic_function_errors() {
        // No arguments
        eval_error("sum()");
        eval_error("product()");
        
        // Wrong argument types
        eval_error("sum(5, true)");
        eval_error("sum(\"hello\", 5)");
        eval_error("product(5, [1, 2, 3])");
        eval_error("product(true, false)");
        
        // Invalid kwargs
        eval_error("sum(x=5, y=10)");      // Unknown parameters
        eval_error("product(items=5)");    // Unknown parameter name
    }

    #[test]
    fn test_variadic_large_argument_lists() {
        // Test with many arguments to ensure the system handles large lists
        eval_number("sum(1, 1, 1, 1, 1, 1, 1, 1, 1, 1)", 10.0);
        eval_number("product(1, 1, 1, 1, 1, 1, 1, 1, 1, 2)", 2.0);
        
        // Test mixed positional and keyword arguments with many values
        eval_number("sum(1, 2, values=3, values=4, values=5)", 15.0);
        eval_number("product(2, values=1, values=1, values=1, values=5)", 10.0);
    }

    #[test]
    fn test_variadic_with_complex_expressions() {
        eval_number("sum(1 + 2, 3 * 4, 5 - 1)", 19.0);
        eval_number("product(2 + 2, 3 - 1, 1 * 5)", 40.0);
        eval_number("x = 5; sum(x, x * 2, x + 1)", 21.0);
    }
}

#[cfg(test)]
mod compound_assignment_tests {
    use super::*;

    #[test]
    fn test_basic_compound_assignment() {
        eval_number("x = 10; x += 5; x", 15.0);
        eval_number("y = 20; y -= 8; y", 12.0);
        eval_number("z = 3; z *= 4; z", 12.0);
        eval_number("w = 15; w /= 3; w", 5.0);
    }

    #[test]
    fn test_compound_assignment_with_expressions() {
        eval_number("a = 5; a += 2 * 3; a", 11.0);
        eval_number("b = 10; b -= (3 + 2); b", 5.0);
        eval_number("c = 2; c *= (3 + 4); c", 14.0);
        eval_number("d = 20; d /= (2 + 2); d", 5.0);
    }

    #[test]
    fn test_compound_assignment_sequence() {
        let mut evaluator = Evaluator::new();
        
        eval_with_evaluator(&mut evaluator, "x = 10").unwrap();
        eval_with_evaluator(&mut evaluator, "x += 5").unwrap();
        assert_eq!(evaluator.get_variable("x").unwrap().as_number().unwrap(), 15.0);
        
        eval_with_evaluator(&mut evaluator, "x -= 3").unwrap();
        assert_eq!(evaluator.get_variable("x").unwrap().as_number().unwrap(), 12.0);
        
        eval_with_evaluator(&mut evaluator, "x *= 2").unwrap();
        assert_eq!(evaluator.get_variable("x").unwrap().as_number().unwrap(), 24.0);
        
        eval_with_evaluator(&mut evaluator, "x /= 4").unwrap();
        assert_eq!(evaluator.get_variable("x").unwrap().as_number().unwrap(), 6.0);
    }

    #[test]
    fn test_compound_assignment_errors() {
        // Undefined variable
        eval_error("undefined_var += 5");
        
        // Division by zero
        eval_error("x = 10; x /= 0");
        
        // Type errors
        eval_error("x = \"hello\"; x += 5");
        eval_error("y = true; y *= 2");
    }
}

#[cfg(test)]
mod collection_tests {
    use super::*;

    #[test]
    fn test_list_operations() {
        // List creation and indexing
        match eval_code("[1, 2, 3][0]").unwrap().unwrap() {
            Value::Integer(1) => {},
            other => panic!("Expected Integer(1), got {:?}", other),
        }
        
        match eval_code("[\"a\", \"b\", \"c\"][1]").unwrap().unwrap() {
            Value::String(s) if s == "b" => {},
            other => panic!("Expected String(\"b\"), got {:?}", other),
        }
        
        // Nested lists
        match eval_code("[[1, 2], [3, 4]][0][1]").unwrap().unwrap() {
            Value::Integer(2) => {},
            other => panic!("Expected Integer(2), got {:?}", other),
        }
    }

    #[test]
    fn test_dictionary_operations() {
        // Dictionary creation and lookup
        match eval_code("{\"name\": \"John\"}[\"name\"]").unwrap().unwrap() {
            Value::String(s) if s == "John" => {},
            other => panic!("Expected String(\"John\"), got {:?}", other),
        }
        
        match eval_code("{\"x\": 42, \"y\": 24}[\"x\"]").unwrap().unwrap() {
            Value::Integer(42) => {},
            other => panic!("Expected Integer(42), got {:?}", other),
        }
    }

    #[test]
    fn test_collection_errors() {
        // Index out of bounds
        eval_error("[1, 2, 3][5]");
        
        // Key not found
        eval_error("{\"a\": 1}[\"b\"]");
        
        // Wrong index type
        eval_error("[1, 2, 3][\"not_a_number\"]");
        eval_error("{\"a\": 1}[42]");
        
        // Indexing non-collection
        eval_error("42[0]");
        eval_error("\"hello\"[0]");
    }

    #[test]
    fn test_membership_operations() {
        // List membership
        match eval_code("5 in [1, 2, 5, 8]").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        match eval_code("3 not in [1, 2, 5, 8]").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        // Dictionary membership (keys)
        match eval_code("\"name\" in {\"name\": \"John\", \"age\": 30}").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        match eval_code("\"city\" not in {\"name\": \"John\", \"age\": 30}").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
    }
}

#[cfg(test)]
mod logical_operator_tests {
    use super::*;

    #[test]
    fn test_logical_and() {
        match eval_code("true and true").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        match eval_code("true and false").unwrap().unwrap() {
            Value::Boolean(false) => {},
            other => panic!("Expected Boolean(false), got {:?}", other),
        }
        
        // Short-circuit evaluation
        match eval_code("false and \"hello\"").unwrap().unwrap() {
            Value::Boolean(false) => {},
            other => panic!("Expected Boolean(false), got {:?}", other),
        }
        
        match eval_code("true and \"hello\"").unwrap().unwrap() {
            Value::String(s) if s == "hello" => {},
            other => panic!("Expected String(\"hello\"), got {:?}", other),
        }
    }

    #[test]
    fn test_logical_or() {
        match eval_code("false or true").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        match eval_code("false or false").unwrap().unwrap() {
            Value::Boolean(false) => {},
            other => panic!("Expected Boolean(false), got {:?}", other),
        }
        
        // Short-circuit evaluation
        match eval_code("true or \"hello\"").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        match eval_code("false or \"hello\"").unwrap().unwrap() {
            Value::String(s) if s == "hello" => {},
            other => panic!("Expected String(\"hello\"), got {:?}", other),
        }
    }

    #[test]
    fn test_logical_not() {
        match eval_code("not true").unwrap().unwrap() {
            Value::Boolean(false) => {},
            other => panic!("Expected Boolean(false), got {:?}", other),
        }
        
        match eval_code("not false").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        // Truthiness
        match eval_code("not 0").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        match eval_code("not \"\"").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        match eval_code("not \"hello\"").unwrap().unwrap() {
            Value::Boolean(false) => {},
            other => panic!("Expected Boolean(false), got {:?}", other),
        }
    }

    #[test]
    fn test_complex_logical_expressions() {
        match eval_code("(5 > 3) and (2 < 4)").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        match eval_code("(5 < 3) or (2 < 4)").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        match eval_code("not (5 > 3)").unwrap().unwrap() {
            Value::Boolean(false) => {},
            other => panic!("Expected Boolean(false), got {:?}", other),
        }
    }
}

#[cfg(test)]
mod comprehensive_tests {
    use super::*;

    #[test]
    fn test_complex_expressions() {
        eval_number("(3 + 4) * (2 - 1)", 7.0);
        eval_number("10 / 2 + 3 * 4", 17.0);
        eval_number("2 * 3 + 4 * 5", 26.0);
        eval_number("(10 + 20) / (3 + 2)", 6.0);
    }

    #[test]
    fn test_variable_interactions() {
        eval_number("x = 5; y = x * 2; z = y + x; z", 15.0);
        eval_number("a = 10; b = a; c = a + b; c", 20.0);
    }

    #[test]
    fn test_mixed_operations() {
        // Arithmetic + functions
        eval_number("x = max(5, 8); y = min(3, 7); x + y", 11.0);
        
        // Collections + arithmetic
        eval_number("nums = [1, 2, 3]; nums[0] + nums[2]", 4.0);
        
        // Functions + collections
        match eval_code("max(5, 8) in [1, 8, 3]").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
        
        // Variadic functions + arithmetic
        eval_number("sum(1, 2, 3) + product(2, 2)", 10.0);
        eval_number("max(sum(1, 2), product(2, 3))", 6.0);
        
        // Variadic functions with collections
        eval_number("x = [1, 2, 3]; sum(x[0], x[1], x[2])", 6.0);
    }

    #[test]
    fn test_operator_precedence() {
        eval_number("2 + 3 * 4", 14.0);        // Not 20
        eval_number("(2 + 3) * 4", 20.0);      // Explicit grouping
        eval_number("2 * 3 + 4", 10.0);        // Not 14
        eval_number("2 + 3 * 4 - 1", 13.0);    // 2 + 12 - 1
    }

    #[test]
    fn test_type_coercion() {
        eval_number("42 + 3.14", 45.14);       // Integer + Float
        eval_number("10.0 - 3", 7.0);          // Float - Integer
        
        match eval_code("42 == 42.0").unwrap().unwrap() {
            Value::Boolean(true) => {},
            other => panic!("Expected Boolean(true), got {:?}", other),
        }
    }

    #[test]
    fn test_error_recovery() {
        // These should all fail but not crash
        eval_error("1 + ");            // Incomplete expression
        eval_error("(1 + 2");          // Missing closing paren
        eval_error("[1, 2, ");         // Incomplete list
        eval_error("{\"a\": }");        // Incomplete dictionary
        eval_error("1 + true");        // Type error
        eval_error("undefined_var");   // Undefined variable
        
        // Variadic function error recovery
        eval_error("sum(");            // Incomplete function call
        eval_error("product(1, ");     // Incomplete argument list
        eval_error("sum(1, 2, ");      // Incomplete variadic call
    }
}

#[cfg(test)]
mod error_diagnostic_tests {
    use super::*;
    use bccl::ErrorContext;

    #[test]
    fn test_error_context_creation() {
        let input = "x + undefined_variable";
        let context = ErrorContext::new(input.to_string());
        assert_eq!(context.source.to_string(), input);
    }

    #[test]
    fn test_specific_error_cases() {
        // Test cases that should produce specific error types
        let error_cases = vec![
            ("@", "Invalid character"),           // Lexer error
            ("1 +", "Unexpected end"),           // Parser error  
            ("undefined", "Undefined variable"), // Runtime error
            ("1 / 0", "Division by zero"),       // Math error
            ("max()", "Wrong argument count"),   // Function error
        ];

        for (input, _description) in error_cases {
            match eval_code(input) {
                Ok(_) => panic!("Expected error for input: {}", input),
                Err(_) => {} // Expected - we're just testing that errors occur
            }
        }
    }
}