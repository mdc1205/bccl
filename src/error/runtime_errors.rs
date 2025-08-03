use miette::SourceSpan;
use super::span::Span;
use super::types::BcclError;

impl BcclError {
    /// Create a runtime error for undefined variables
    pub fn undefined_variable(name: &str, span: Span, available_vars: &[String]) -> Self {
        let suggestion = if available_vars.is_empty() {
            Some("No variables are currently defined".to_string())
        } else {
            // Simple suggestion based on edit distance or similar names
            let similar = available_vars.iter()
                .find(|var| var.len() > 2 && name.len() > 2 && 
                     var.chars().take(2).collect::<String>() == name.chars().take(2).collect::<String>());
            
            if let Some(similar_var) = similar {
                Some(format!("Did you mean '{}'?", similar_var))
            } else {
                Some(format!("Available variables: {}", available_vars.join(", ")))
            }
        };

        Self::UndefinedVariable {
            name: name.to_string(),
            span: span.into(),
            suggestion,
        }
    }

    /// Create a runtime error for division by zero
    pub fn division_by_zero(op_span: Span, divisor_span: Span) -> Self {
        Self::DivisionByZero {
            span: op_span.into(),
            divisor_span: divisor_span.into(),
        }
    }

    /// Create a type error
    pub fn type_error(message: &str, span: Span, expected: &str, actual: &str) -> Self {
        Self::TypeError {
            message: message.to_string(),
            span: span.into(),
            expected_type: expected.to_string(),
            actual_type: actual.to_string(),
        }
    }

    /// Create a general evaluation error
    pub fn evaluation_error(message: &str, span: Span, suggestion: Option<String>) -> Self {
        Self::EvaluationError {
            message: message.to_string(),
            span: span.into(),
            suggestion,
        }
    }

    /// Create an assignment error
    pub fn assignment_error(message: &str, span: Span, variable_name: &str) -> Self {
        Self::AssignmentError {
            message: message.to_string(),
            span: span.into(),
            variable_name: variable_name.to_string(),
        }
    }

    /// Create an undefined function error
    pub fn undefined_function(name: &str, span: Span, available_functions: &[String]) -> Self {
        let suggestion = if available_functions.is_empty() {
            Some("No functions are currently available".to_string())
        } else {
            Some(format!("Available functions: {}", available_functions.join(", ")))
        };

        Self::UndefinedFunction {
            name: name.to_string(),
            span: span.into(),
            suggestion,
        }
    }

    /// Create a wrong argument count error
    pub fn wrong_argument_count(function_name: &str, expected: usize, actual: usize) -> Self {
        Self::WrongArgumentCount {
            function_name: function_name.to_string(),
            expected,
            actual,
            span: SourceSpan::new(0.into(), 0), // This will be updated when we have better span tracking
        }
    }

    /// Create a function argument type error with proper span and value information
    pub fn function_argument_type_error_with_span(function_name: &str, arg_number: usize, expected_type: &str, actual_type: &str, actual_value: &str, span: Span) -> Self {
        Self::FunctionArgumentTypeError {
            function_name: function_name.to_string(),
            arg_number,
            expected_type: expected_type.to_string(),
            actual_type: format!("{} (value: {})", actual_type, actual_value),
            span: span.into(),
        }
    }

    /// Create a function argument type error (legacy - for backward compatibility)
    pub fn function_argument_type_error(function_name: &str, arg_number: usize, expected_type: &str, actual_type: &str) -> Self {
        Self::FunctionArgumentTypeError {
            function_name: function_name.to_string(),
            arg_number,
            expected_type: expected_type.to_string(),
            actual_type: actual_type.to_string(),
            span: SourceSpan::new(0.into(), 0), // This will be updated when we have better span tracking
        }
    }

    /// Create an index out of bounds error
    pub fn index_out_of_bounds(collection_type: &str, index: usize, length: usize, span: Span) -> Self {
        Self::IndexOutOfBounds {
            collection_type: collection_type.to_string(),
            index,
            length,
            span: span.into(),
        }
    }

    /// Create a key not found error
    pub fn key_not_found(key: &str, span: Span, available_keys: &[String]) -> Self {
        let suggestion = if available_keys.is_empty() {
            "Dictionary is empty. Add some key-value pairs first.".to_string()
        } else {
            // Try to find similar keys
            let similar = available_keys.iter()
                .find(|available_key| {
                    // Simple similarity check: same length or common prefix
                    available_key.len() == key.len() || 
                    (available_key.len() > 2 && key.len() > 2 && 
                     available_key.chars().take(2).collect::<String>() == key.chars().take(2).collect::<String>())
                });
            
            if let Some(similar_key) = similar {
                format!("Did you mean '{}'? Available keys: {}", similar_key, available_keys.join(", "))
            } else if available_keys.len() <= 5 {
                format!("Available keys: {}", available_keys.join(", "))
            } else {
                format!("Dictionary has {} keys. Some examples: {}", 
                       available_keys.len(), 
                       available_keys.iter().take(3).cloned().collect::<Vec<_>>().join(", "))
            }
        };
        
        Self::KeyNotFound {
            key: key.to_string(),
            span: span.into(),
            available_keys: available_keys.to_vec(),
            suggestion,
        }
    }

    /// Create a compound assignment error with enhanced diagnostics
    pub fn compound_assignment_error(variable: &str, operator: &str, message: &str, span: Span) -> Self {
        let suggestion = match operator {
            "+=" => format!("Make sure '{}' is a number. Use regular assignment if setting a new value.", variable),
            "-=" | "*=" | "/=" => format!("Ensure '{}' contains a numeric value before using '{}'.", variable, operator),
            _ => format!("Check that '{}' is defined and contains a compatible value for {} operation.", variable, operator)
        };
        
        Self::CompoundAssignmentError {
            message: message.to_string(),
            variable_name: variable.to_string(),
            operator: operator.to_string(),
            span: span.into(),
            suggestion,
        }
    }

    /// Create a logical operation error with helpful suggestions
    pub fn logical_operation_error(operator: &str, message: &str, span: Span) -> Self {
        let suggestion = match operator {
            "and" | "or" => "Both operands can be any type. Results follow Python-like truthiness rules.".to_string(),
            "not" => "The 'not' operator works with any value type and returns a boolean.".to_string(),
            "in" | "not in" => "Use 'value in collection' for lists or 'key in dictionary' for dictionaries.".to_string(),
            "<" | ">" | "<=" | ">=" => "Comparison operators require both operands to be numbers.".to_string(),
            _ => "Check the operator documentation for usage examples.".to_string()
        };
        
        Self::LogicalOperationError {
            message: message.to_string(),
            operator: operator.to_string(),
            span: span.into(),
            suggestion,
        }
    }

    /// Create a collection operation error with detailed help
    pub fn collection_operation_error(operation: &str, message: &str, span: Span) -> Self {
        let suggestion = match operation {
            "index" => "Use integers for list indexing: list[0], list[1]. Use strings for dictionary keys: dict[\"key\"].".to_string(),
            "membership" => "Use 'item in list' to check if item exists in list. Use 'key in dict' to check if key exists in dictionary.".to_string(),
            _ => "Check the collection operation syntax and types.".to_string()
        };
        
        Self::CollectionOperationError {
            message: message.to_string(),
            operation: operation.to_string(),
            span: span.into(),
            suggestion,
        }
    }

    /// Create a duplicate parameter error
    pub fn duplicate_parameter(function_name: &str, parameter_name: &str, span: Span) -> Self {
        Self::FunctionArgumentError {
            message: format!("Parameter '{}' specified multiple times in call to function '{}'", parameter_name, function_name),
            function_name: function_name.to_string(),
            span: span.into(),
            suggestion: format!("Remove the duplicate '{}' parameter. Each parameter can only be specified once.", parameter_name),
        }
    }

    /// Create an unknown parameter error
    pub fn unknown_parameter(function_name: &str, parameter_name: &str, span: Span, valid_params: &[String]) -> Self {
        let suggestion = if valid_params.is_empty() {
            format!("Function '{}' takes no parameters", function_name)
        } else {
            format!("Valid parameters for '{}' are: {}", function_name, valid_params.join(", "))
        };
        
        Self::FunctionArgumentError {
            message: format!("Unknown parameter '{}' in call to function '{}'", parameter_name, function_name),
            function_name: function_name.to_string(),
            span: span.into(),
            suggestion,
        }
    }

    /// Create a missing parameter error
    pub fn missing_parameter(function_name: &str, parameter_name: &str, span: Span) -> Self {
        Self::FunctionArgumentError {
            message: format!("Missing required parameter '{}' in call to function '{}'", parameter_name, function_name),
            function_name: function_name.to_string(),
            span: span.into(),
            suggestion: format!("Provide a value for parameter '{}' either positionally or using '{} = value'", parameter_name, parameter_name),
        }
    }
}