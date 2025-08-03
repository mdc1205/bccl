#[cfg(test)]
mod tests {
    use super::super::{Evaluator, Value};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::error::{BcclError, BcclResult};

    fn evaluate_from_str(input: &str) -> BcclResult<Option<Value>> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        let mut evaluator = Evaluator::new();
        evaluator.evaluate_program(&program)
    }

    #[test]
    fn test_evaluate_number() {
        let result = evaluate_from_str("42.0").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_integer() {
        let result = evaluate_from_str("42").unwrap().unwrap();
        match result {
            Value::Integer(i) => assert_eq!(i, 42),
            _ => panic!("Expected Integer value"),
        }
    }

    #[test]
    fn test_evaluate_boolean() {
        let result = evaluate_from_str("true").unwrap().unwrap();
        match result {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected Boolean value"),
        }

        let result = evaluate_from_str("false").unwrap().unwrap();
        match result {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn test_evaluate_string() {
        let result = evaluate_from_str(r#""hello world""#).unwrap().unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("Expected String value"),
        }
    }

    #[test]
    fn test_evaluate_addition() {
        let result = evaluate_from_str("2 + 3").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 5.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_subtraction() {
        let result = evaluate_from_str("10 - 4").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 6.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_multiplication() {
        let result = evaluate_from_str("6 * 7").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_division() {
        let result = evaluate_from_str("15 / 3").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 5.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let result = evaluate_from_str("5 / 0");
        assert!(matches!(result, Err(BcclError::DivisionByZero { .. })));
    }

    #[test]
    fn test_evaluate_operator_precedence() {
        let result = evaluate_from_str("2 + 3 * 4").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 14.0), // 2 + (3 * 4) = 14
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_parentheses() {
        let result = evaluate_from_str("(2 + 3) * 4").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 20.0), // (2 + 3) * 4 = 20
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_unary_minus() {
        let result = evaluate_from_str("-5").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, -5.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_unary_plus() {
        let result = evaluate_from_str("+42").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_unary_not() {
        let result = evaluate_from_str("not true").unwrap().unwrap();
        match result {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("Expected Boolean value"),
        }

        let result = evaluate_from_str("not false").unwrap().unwrap();
        match result {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn test_evaluate_complex_expression() {
        let result = evaluate_from_str("2 * (3 + 4) - 5 / 1").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 9.0), // 2 * 7 - 5 = 14 - 5 = 9
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_assignment() {
        let result = evaluate_from_str("x = 10").unwrap().unwrap();
        match result {
            Value::Integer(i) => assert_eq!(i, 10),
            _ => panic!("Expected Integer value"),
        }
    }

    #[test]
    fn test_evaluate_variable_usage() {
        let result = evaluate_from_str("x = 5; x + 3").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 8.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_multiple_assignments() {
        let mut lexer = Lexer::new("x = 5; y = x + 3; z = x * y");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        
        evaluator.evaluate_program(&program).unwrap();
        
        match evaluator.get_variable("x").unwrap() {
            Value::Integer(i) => assert_eq!(*i, 5),
            _ => panic!("Expected Integer value for x"),
        }
        match evaluator.get_variable("y").unwrap() {
            Value::Number(n) => assert_eq!(*n, 8.0),
            _ => panic!("Expected Number value for y"),
        }
        match evaluator.get_variable("z").unwrap() {
            Value::Number(n) => assert_eq!(*n, 40.0),
            _ => panic!("Expected Number value for z"),
        }
    }

    #[test]
    fn test_evaluate_undefined_variable() {
        let result = evaluate_from_str("unknown_var");
        assert!(matches!(result, Err(BcclError::UndefinedVariable { .. })));
    }

    #[test]
    fn test_evaluate_variable_reassignment() {
        let result = evaluate_from_str("x = 10; x = 20; x").unwrap().unwrap();
        match result {
            Value::Integer(i) => assert_eq!(i, 20),
            _ => panic!("Expected Integer value"),
        }
    }

    #[test]
    fn test_evaluate_expression_with_variables() {
        let result = evaluate_from_str("a = 2; b = 3; c = 4; a + b * c").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 14.0), // 2 + 3 * 4 = 14
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_nested_expressions() {
        let result = evaluate_from_str("((2 + 3) * (4 - 1)) / 5").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 3.0), // ((5) * (3)) / 5 = 15 / 5 = 3
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_compound_assignment() {
        let result = evaluate_from_str("x = 10; x += 5; x").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 15.0),
            _ => panic!("Expected Number value"),
        }
        
        let result = evaluate_from_str("y = 20; y -= 5; y").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 15.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_logical_operators() {
        let result = evaluate_from_str("true and false").unwrap().unwrap();
        match result {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("Expected Boolean value"),
        }

        let result = evaluate_from_str("true or false").unwrap().unwrap();
        match result {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn test_evaluate_comparison_operators() {
        let result = evaluate_from_str("5 > 3").unwrap().unwrap();
        match result {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected Boolean value"),
        }

        let result = evaluate_from_str("5 <= 3").unwrap().unwrap();
        match result {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("Expected Boolean value"),
        }
    }

    #[test]
    fn test_evaluate_lists() {
        let result = evaluate_from_str("[1, 2, 3]").unwrap().unwrap();
        match result {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                assert!(matches!(items[0], Value::Integer(1)));
                assert!(matches!(items[1], Value::Integer(2)));
                assert!(matches!(items[2], Value::Integer(3)));
            }
            _ => panic!("Expected List value"),
        }
    }

    #[test]
    fn test_evaluate_dictionaries() {
        let result = evaluate_from_str(r#"{"name": "John", "age": 30}"#).unwrap().unwrap();
        match result {
            Value::Dictionary(dict) => {
                assert_eq!(dict.len(), 2);
                assert!(matches!(dict.get("name"), Some(Value::String(s)) if s == "John"));
                assert!(matches!(dict.get("age"), Some(Value::Integer(30))));
            }
            _ => panic!("Expected Dictionary value"),
        }
    }

    #[test]
    fn test_evaluate_indexing() {
        let result = evaluate_from_str("[10, 20, 30][1]").unwrap().unwrap();
        match result {
            Value::Integer(i) => assert_eq!(i, 20),
            _ => panic!("Expected Integer value"),
        }

        let result = evaluate_from_str(r#"{"key": "value"}["key"]"#).unwrap().unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "value"),
            _ => panic!("Expected String value"),
        }
    }

    #[test]
    fn test_evaluate_function_calls() {
        let result = evaluate_from_str("max(10, 5)").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 10.0),
            _ => panic!("Expected Number value"),
        }

        let result = evaluate_from_str("min(10, 5)").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 5.0),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_evaluate_function_kwargs() {
        // Test pure keyword arguments
        let result = evaluate_from_str("max(a=5, b=10)").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 10.0),
            _ => panic!("Expected Number value"),
        }

        // Test mixed positional and keyword arguments
        let result = evaluate_from_str("max(5, b=10)").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 10.0),
            _ => panic!("Expected Number value"),
        }

        // Test kwargs with min function
        let result = evaluate_from_str("min(x=3, y=7)").unwrap().unwrap();
        match result {
            Value::Number(n) => assert_eq!(n, 3.0),
            _ => panic!("Expected Number value"),
        }
    }
}