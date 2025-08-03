//! # Built-in Functions Module (Enhanced with Variadic Support)
//!
//! This module implements BCCL's enhanced built-in function system with support for:
//! - Fixed parameter functions: `max(a, b)`
//! - Variadic functions: `sum(values...)`
//! - Mixed functions: `clamp(value, bounds...)`
//!
//! ## Function System Architecture
//!
//! - **ParameterSpec**: Defines whether a function has fixed or variadic parameters
//! - **FunctionSignature**: Enhanced to handle both parameter types
//! - **Parameter Validation**: Ensures correct argument count and types for both cases
//! - **Keyword Arguments**: Support for mixed positional and keyword arguments
//! - **Span-aware Errors**: Precise error reporting with source location information

use crate::error::{BcclError, BcclResult, Span};
use super::value::Value;
use std::collections::HashMap;

/// Defines the parameter requirements for a function.
#[derive(Debug, Clone)]
pub enum ParameterSpec {
    /// Fixed number of parameters with specific names
    /// Example: max(a, b) -> Fixed(["a", "b"])
    Fixed(Vec<String>),
    /// Minimum required parameters plus variadic parameters
    /// Example: sum(values...) -> Variadic { required: [], variadic_name: "values" }
    /// Example: clamp(value, bounds...) -> Variadic { required: ["value"], variadic_name: "bounds" }
    Variadic { 
        required: Vec<String>, 
        variadic_name: String 
    },
}

/// Represents a function signature with parameter validation and implementation.
/// 
/// Function signatures define the contract for calling a function, including:
/// - Function name (for error messages)
/// - Parameter specification (fixed or variadic)
/// - Implementation function pointer
/// 
/// # Design
/// 
/// The signature system enables:
/// - **Fixed parameter functions**: `max(a, b)` - exactly 2 parameters
/// - **Variadic functions**: `sum(values...)` - 1 or more parameters
/// - **Mixed functions**: `substring(text, start, end...)` - required + optional
/// - **Keyword arguments**: Functions can be called with kwargs for any parameter type
/// - **Rich errors**: Detailed error messages with parameter information
/// 
/// # Examples
/// 
/// ```rust
/// // Fixed parameters: max(a, b)
/// let max_sig = FunctionSignature::new_fixed("max", vec!["a", "b"], builtin_max_impl);
/// 
/// // Variadic: sum(values...)
/// let sum_sig = FunctionSignature::new_variadic("sum", vec![], "values", builtin_sum_impl);
/// 
/// // Mixed: clamp(value, bounds...)
/// let clamp_sig = FunctionSignature::new_variadic("clamp", vec!["value"], "bounds", builtin_clamp_impl);
/// ```
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Function name (used in error messages)
    pub name: String,
    /// Parameter specification (fixed or variadic)
    pub parameters: ParameterSpec,
    /// Function implementation
    pub function: fn(&[Value]) -> BcclResult<Value>,
}

impl FunctionSignature {
    /// Creates a new function signature with fixed parameters.
    /// 
    /// # Arguments
    /// 
    /// * `name` - Function name (for error messages)
    /// * `parameters` - Fixed parameter names
    /// * `function` - Implementation function
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let max_sig = FunctionSignature::new_fixed("max", vec!["a", "b"], builtin_max_impl);
    /// ```
    pub fn new_fixed(name: &str, parameters: Vec<&str>, function: fn(&[Value]) -> BcclResult<Value>) -> Self {
        Self {
            name: name.to_string(),
            parameters: ParameterSpec::Fixed(
                parameters.into_iter().map(|s| s.to_string()).collect()
            ),
            function,
        }
    }
    
    /// Creates a new function signature with variadic parameters.
    /// 
    /// # Arguments
    /// 
    /// * `name` - Function name (for error messages)
    /// * `required` - Required parameter names (must be provided)
    /// * `variadic_name` - Name for the variadic parameters (for kwargs)
    /// * `function` - Implementation function
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// // sum(values...) - no required parameters, all are variadic
    /// let sum_sig = FunctionSignature::new_variadic("sum", vec![], "values", builtin_sum_impl);
    /// 
    /// // clamp(value, bounds...) - one required, rest variadic
    /// let clamp_sig = FunctionSignature::new_variadic("clamp", vec!["value"], "bounds", builtin_clamp_impl);
    /// ```
    pub fn new_variadic(name: &str, required: Vec<&str>, variadic_name: &str, function: fn(&[Value]) -> BcclResult<Value>) -> Self {
        Self {
            name: name.to_string(),
            parameters: ParameterSpec::Variadic {
                required: required.into_iter().map(|s| s.to_string()).collect(),
                variadic_name: variadic_name.to_string(),
            },
            function,
        }
    }
    
    /// Legacy constructor for backward compatibility.
    /// 
    /// Creates a fixed-parameter function signature.
    /// 
    /// # Deprecated
    /// 
    /// Use `new_fixed()` for better clarity.
    pub fn new(name: &str, parameters: Vec<&str>, function: fn(&[Value]) -> BcclResult<Value>) -> Self {
        Self::new_fixed(name, parameters, function)
    }

    /// Calls the function with positional and keyword arguments, providing span information for precise error reporting.
    /// 
    /// This is the primary method for function invocation in BCCL. It handles:
    /// - Parameter validation and matching for both fixed and variadic functions
    /// - Mixed positional and keyword arguments
    /// - Duplicate parameter detection
    /// - Missing parameter detection
    /// - Rich error messages with source spans
    /// 
    /// # Arguments
    /// 
    /// * `args` - Positional arguments with their source spans
    /// * `kwargs` - Keyword arguments with their source spans
    /// * `span` - Overall function call span (for general errors)
    /// 
    /// # Returns
    /// 
    /// * `Ok(value)` - Function result
    /// * `Err(error)` - Parameter validation or execution errors
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// // Fixed: max(5, 10)
    /// let args = vec![(Value::Integer(5), span1), (Value::Integer(10), span2)];
    /// let result = sig.call_with_spans(&args, &[], call_span)?;
    /// 
    /// // Variadic: sum(1, 2, 3, 4)
    /// let args = vec![(Value::Integer(1), span1), (Value::Integer(2), span2), ...];
    /// let result = sig.call_with_spans(&args, &[], call_span)?;
    /// 
    /// // Mixed: clamp(value=5, bounds=1, bounds=2, bounds=10)
    /// ```
    /// 
    /// # Error Cases
    /// 
    /// - Fixed functions: Too many/few arguments, unknown parameters
    /// - Variadic functions: Too few required arguments, unknown parameters
    pub fn call_with_spans(&self, args: &[(Value, Span)], kwargs: &[(String, Value, Span)], span: Span) -> BcclResult<Value> {
        match &self.parameters {
            ParameterSpec::Fixed(params) => {
                self.call_fixed_with_spans(params, args, kwargs, span)
            }
            ParameterSpec::Variadic { required, variadic_name } => {
                self.call_variadic_with_spans(required, variadic_name, args, kwargs, span)
            }
        }
    }

    /// Handles fixed-parameter function calls.
    fn call_fixed_with_spans(&self, params: &[String], args: &[(Value, Span)], kwargs: &[(String, Value, Span)], span: Span) -> BcclResult<Value> {
        // Build final argument list respecting parameter order
        let mut final_args = vec![None; params.len()];
        let mut final_spans = vec![span; params.len()];
        
        // First, fill in positional arguments
        if args.len() > params.len() {
            return Err(BcclError::wrong_argument_count(&self.name, params.len(), args.len() + kwargs.len()));
        }
        
        for (i, (value, arg_span)) in args.iter().enumerate() {
            final_args[i] = Some(value.clone());
            final_spans[i] = *arg_span;
        }
        
        // Then, fill in keyword arguments
        for (param_name, value, arg_span) in kwargs {
            // Find the parameter index
            if let Some(param_index) = params.iter().position(|p| p == param_name) {
                // Check if this parameter was already provided positionally
                if final_args[param_index].is_some() {
                    return Err(BcclError::duplicate_parameter(&self.name, param_name, *arg_span));
                }
                final_args[param_index] = Some(value.clone());
                final_spans[param_index] = *arg_span;
            } else {
                // Parameter name not found in function signature
                return Err(BcclError::unknown_parameter(&self.name, param_name, *arg_span, params));
            }
        }
        
        // Check that all parameters are provided
        let mut provided_args = Vec::new();
        let mut provided_spans = Vec::new();
        for (i, (arg_opt, arg_span)) in final_args.into_iter().zip(final_spans.into_iter()).enumerate() {
            match arg_opt {
                Some(value) => {
                    provided_args.push(value);
                    provided_spans.push(arg_span);
                }
                None => {
                    return Err(BcclError::missing_parameter(&self.name, &params[i], span));
                }
            }
        }
        
        // Call the actual function with span information
        self.call_impl(&provided_args, &provided_spans)
    }
    
    /// Handles variadic function calls.
    /// 
    /// For variadic functions, arguments are processed as:
    /// 1. Required parameters (filled from positional args first, then kwargs)
    /// 2. Variadic parameters (remaining positional args, plus kwargs with variadic_name)
    fn call_variadic_with_spans(&self, required: &[String], variadic_name: &str, args: &[(Value, Span)], kwargs: &[(String, Value, Span)], span: Span) -> BcclResult<Value> {
        // Check minimum argument count
        if args.len() + kwargs.len() < required.len() {
            return Err(BcclError::wrong_argument_count(&self.name, required.len(), args.len() + kwargs.len()));
        }
        
        let mut final_args = Vec::new();
        let mut final_spans = Vec::new();
        let mut required_filled = vec![false; required.len()];
        
        // Fill required parameters from positional arguments first
        let mut pos_index = 0;
        for (i, req_param) in required.iter().enumerate() {
            if pos_index < args.len() {
                final_args.push(args[pos_index].0.clone());
                final_spans.push(args[pos_index].1);
                required_filled[i] = true;
                pos_index += 1;
            }
        }
        
        // Remaining positional arguments become variadic parameters
        for i in pos_index..args.len() {
            final_args.push(args[i].0.clone());
            final_spans.push(args[i].1);
        }
        
        // Process keyword arguments
        for (param_name, value, arg_span) in kwargs {
            if let Some(req_index) = required.iter().position(|p| p == param_name) {
                // This is a required parameter
                if required_filled[req_index] {
                    return Err(BcclError::duplicate_parameter(&self.name, param_name, *arg_span));
                }
                
                // Find where to insert this required parameter
                if req_index < required.len() {
                    // Insert at the correct position for required parameters
                    final_args.insert(req_index, value.clone());
                    final_spans.insert(req_index, *arg_span);
                    required_filled[req_index] = true;
                }
            } else if param_name == variadic_name {
                // This is a variadic parameter - add to the end
                final_args.push(value.clone());
                final_spans.push(*arg_span);
            } else {
                // Unknown parameter
                let mut all_params = required.to_vec();
                all_params.push(variadic_name.to_string());
                return Err(BcclError::unknown_parameter(&self.name, param_name, *arg_span, &all_params));
            }
        }
        
        // Check that all required parameters are provided
        for (i, &filled) in required_filled.iter().enumerate() {
            if !filled {
                return Err(BcclError::missing_parameter(&self.name, &required[i], span));
            }
        }
        
        // Call the actual function with span information
        self.call_impl(&final_args, &final_spans)
    }
    
    /// Calls the function implementation with span information for enhanced error reporting.
    /// 
    /// This method dispatches to span-aware implementations for built-in functions
    /// that provide better error messages with precise source locations.
    /// 
    /// # Arguments
    /// 
    /// * `args` - Function arguments (required parameters first, then variadic)
    /// * `spans` - Source spans for each argument
    /// 
    /// # Returns
    /// 
    /// Function result or error with precise span information.
    fn call_impl(&self, args: &[Value], spans: &[Span]) -> BcclResult<Value> {
        // For built-in functions, we need to handle type checking with proper spans
        match self.name.as_str() {
            "max" => builtin_max_with_spans(args, spans),
            "min" => builtin_min_with_spans(args, spans),
            "sum" => builtin_sum_with_spans(args, spans),
            "product" => builtin_product_with_spans(args, spans),
            _ => (self.function)(args), // Fallback to legacy function call
        }
    }

    /// Legacy function calling interface without span information.
    /// 
    /// This method is maintained for backward compatibility but provides
    /// less precise error reporting than `call_with_spans()`.
    /// 
    /// # Arguments
    /// 
    /// * `args` - Positional arguments
    /// * `kwargs` - Keyword arguments (without spans)
    /// * `span` - General call location
    /// 
    /// # Returns
    /// 
    /// Function result or error.
    /// 
    /// # Deprecated
    /// 
    /// Prefer `call_with_spans()` for better error reporting.
    pub fn call(&self, args: &[Value], kwargs: &[(String, Value)], span: Span) -> BcclResult<Value> {
        // Convert to span-aware call by using the general span for all arguments
        let args_with_spans: Vec<(Value, Span)> = args.iter().map(|v| (v.clone(), span)).collect();
        let kwargs_with_spans: Vec<(String, Value, Span)> = kwargs.iter().map(|(k, v)| (k.clone(), v.clone(), span)).collect();
        
        self.call_with_spans(&args_with_spans, &kwargs_with_spans, span)
    }
}

// ===== VARIADIC FUNCTION IMPLEMENTATIONS =====

/// Implementation of the `sum(values...)` built-in function with span-aware error reporting.
/// 
/// Returns the sum of all numeric arguments. Accepts any number of arguments (minimum 1).
/// 
/// # Arguments
/// 
/// * `args` - Function arguments (all should be numbers)
/// * `spans` - Source spans for each argument (for error reporting)
/// 
/// # Returns
/// 
/// * `Ok(Value::Number(sum))` - The sum of all values
/// * `Err(type_error)` - If any argument is not numeric
/// 
/// # Examples
/// 
/// - `sum(1, 2, 3)` → `6`
/// - `sum(1.5, 2.5)` → `4.0`
/// - `sum(42)` → `42`
/// 
/// # Type Coercion
/// 
/// Integers are automatically converted to floats for calculation.
/// The result is always a Number (float) to maintain consistency.
fn builtin_sum_with_spans(args: &[Value], spans: &[Span]) -> BcclResult<Value> {
    if args.is_empty() {
        return Err(BcclError::wrong_argument_count("sum", 1, 0));
    }
    
    let mut total = 0.0;
    for (i, arg) in args.iter().enumerate() {
        let num = arg.as_number()
            .ok_or_else(|| BcclError::function_argument_type_error_with_span(
                "sum", i + 1, "number", arg.type_name(), &arg.display(), spans[i]
            ))?;
        total += num;
    }
    
    Ok(Value::Number(total))
}

/// Implementation of the `product(values...)` built-in function with span-aware error reporting.
/// 
/// Returns the product of all numeric arguments. Accepts any number of arguments (minimum 1).
/// 
/// # Arguments
/// 
/// * `args` - Function arguments (all should be numbers)
/// * `spans` - Source spans for each argument (for error reporting)
/// 
/// # Returns
/// 
/// * `Ok(Value::Number(product))` - The product of all values
/// * `Err(type_error)` - If any argument is not numeric
/// 
/// # Examples
/// 
/// - `product(2, 3, 4)` → `24`
/// - `product(1.5, 2)` → `3.0`
/// - `product(42)` → `42`
/// 
/// # Type Coercion
/// 
/// Integers are automatically converted to floats for calculation.
/// The result is always a Number (float) to maintain consistency.
fn builtin_product_with_spans(args: &[Value], spans: &[Span]) -> BcclResult<Value> {
    if args.is_empty() {
        return Err(BcclError::wrong_argument_count("product", 1, 0));
    }
    
    let mut result = 1.0;
    for (i, arg) in args.iter().enumerate() {
        let num = arg.as_number()
            .ok_or_else(|| BcclError::function_argument_type_error_with_span(
                "product", i + 1, "number", arg.type_name(), &arg.display(), spans[i]
            ))?;
        result *= num;
    }
    
    Ok(Value::Number(result))
}

// ===== EXISTING FUNCTIONS (Updated for span-aware dispatch) =====

/// Implementation of the `max(a, b)` built-in function with span-aware error reporting.
/// 
/// Returns the larger of two numeric values. Both arguments must be numbers or integers.
/// 
/// # Arguments
/// 
/// * `args` - Function arguments [a, b]
/// * `spans` - Source spans for each argument (for error reporting)
/// 
/// # Returns
/// 
/// * `Ok(Value::Number(max))` - The larger value
/// * `Err(type_error)` - If either argument is not numeric
/// 
/// # Examples
/// 
/// - `max(5, 10)` → `10`
/// - `max(3.14, 2.7)` → `3.14`
/// - `max(42, 3.14)` → `42` (integer coerced to float)
/// 
/// # Type Coercion
/// 
/// Integers are automatically converted to floats for comparison.
/// The result is always a Number (float) to maintain consistency.
fn builtin_max_with_spans(args: &[Value], spans: &[Span]) -> BcclResult<Value> {
    let a = args[0].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error_with_span(
            "max", 1, "number", args[0].type_name(), &args[0].display(), spans[0]
        ))?;
    let b = args[1].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error_with_span(
            "max", 2, "number", args[1].type_name(), &args[1].display(), spans[1]
        ))?;
    
    Ok(Value::Number(a.max(b)))
}

/// Implementation of the `min(a, b)` built-in function with span-aware error reporting.
/// 
/// Returns the smaller of two numeric values. Both arguments must be numbers or integers.
/// 
/// # Arguments
/// 
/// * `args` - Function arguments [a, b]
/// * `spans` - Source spans for each argument (for error reporting)
/// 
/// # Returns
/// 
/// * `Ok(Value::Number(min))` - The smaller value
/// * `Err(type_error)` - If either argument is not numeric
/// 
/// # Examples
/// 
/// - `min(5, 10)` → `5`
/// - `min(3.14, 2.7)` → `2.7`
/// - `min(42, 3.14)` → `3.14` (integer coerced to float)
/// 
/// # Type Coercion
/// 
/// Integers are automatically converted to floats for comparison.
/// The result is always a Number (float) to maintain consistency.
fn builtin_min_with_spans(args: &[Value], spans: &[Span]) -> BcclResult<Value> {
    let a = args[0].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error_with_span(
            "min", 1, "number", args[0].type_name(), &args[0].display(), spans[0]
        ))?;
    let b = args[1].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error_with_span(
            "min", 2, "number", args[1].type_name(), &args[1].display(), spans[1]
        ))?;
    
    Ok(Value::Number(a.min(b)))
}

// ===== LEGACY IMPLEMENTATIONS =====

// Legacy implementations for backward compatibility
fn builtin_max_impl(args: &[Value]) -> BcclResult<Value> {
    let a = args[0].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error("max", 1, "number", args[0].type_name()))?;
    let b = args[1].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error("max", 2, "number", args[1].type_name()))?;
    
    Ok(Value::Number(a.max(b)))
}

fn builtin_min_impl(args: &[Value]) -> BcclResult<Value> {
    let a = args[0].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error("min", 1, "number", args[0].type_name()))?;
    let b = args[1].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error("min", 2, "number", args[1].type_name()))?;
    
    Ok(Value::Number(a.min(b)))
}

fn builtin_sum_impl(args: &[Value]) -> BcclResult<Value> {
    if args.is_empty() {
        return Err(BcclError::wrong_argument_count("sum", 1, 0));
    }
    
    let mut total = 0.0;
    for (i, arg) in args.iter().enumerate() {
        let num = arg.as_number()
            .ok_or_else(|| BcclError::function_argument_type_error("sum", i + 1, "number", arg.type_name()))?;
        total += num;
    }
    
    Ok(Value::Number(total))
}

fn builtin_product_impl(args: &[Value]) -> BcclResult<Value> {
    if args.is_empty() {
        return Err(BcclError::wrong_argument_count("product", 1, 0));
    }
    
    let mut result = 1.0;
    for (i, arg) in args.iter().enumerate() {
        let num = arg.as_number()
            .ok_or_else(|| BcclError::function_argument_type_error("product", i + 1, "number", arg.type_name()))?;
        result *= num;
    }
    
    Ok(Value::Number(result))
}

/// Legacy `max` function for backward compatibility.
/// 
/// This function provides the old calling interface without span information.
/// New code should use the span-aware version through FunctionSignature.
/// 
/// # Deprecated
/// 
/// Use `FunctionSignature::call_with_spans()` for better error reporting.
pub fn builtin_max(args: &[Value]) -> BcclResult<Value> {
    builtin_max_impl(args)
}

/// Legacy `min` function for backward compatibility.
/// 
/// This function provides the old calling interface without span information.
/// New code should use the span-aware version through FunctionSignature.
/// 
/// # Deprecated
/// 
/// Use `FunctionSignature::call_with_spans()` for better error reporting.
pub fn builtin_min(args: &[Value]) -> BcclResult<Value> {
    builtin_min_impl(args)
}

/// Creates and returns the registry of all built-in functions.
/// 
/// This function initializes all built-in functions with their signatures,
/// parameter names, and implementations. The registry is used by the evaluator
/// to resolve function calls.
/// 
/// # Returns
/// 
/// A HashMap mapping function names to their signatures.
/// 
/// # Built-in Functions
/// 
/// ## Fixed Parameter Functions
/// - **max(a, b)**: Returns the larger of two numbers
/// - **min(a, b)**: Returns the smaller of two numbers
/// 
/// ## Variadic Functions
/// - **sum(values...)**: Returns the sum of all numeric arguments
/// - **product(values...)**: Returns the product of all numeric arguments
/// 
/// # Adding New Functions
/// 
/// To add a new built-in function:
/// 
/// 1. Implement the function (with span-aware version recommended)
/// 2. Add it to this registry with appropriate parameter specification
/// 3. Update the dispatch logic in `FunctionSignature::call_impl`
/// 4. Add comprehensive tests
/// 
/// # Examples
/// 
/// ```rust
/// let functions = get_builtin_functions();
/// let max_sig = functions.get("max").unwrap();
/// let sum_sig = functions.get("sum").unwrap();
/// ```
pub fn get_builtin_functions() -> HashMap<String, FunctionSignature> {
    let mut functions = HashMap::new();
    
    // Fixed parameter functions
    functions.insert(
        "max".to_string(), 
        FunctionSignature::new_fixed("max", vec!["a", "b"], builtin_max_impl)
    );
    
    functions.insert(
        "min".to_string(), 
        FunctionSignature::new_fixed("min", vec!["a", "b"], builtin_min_impl)
    );
    
    // Variadic functions
    functions.insert(
        "sum".to_string(),
        FunctionSignature::new_variadic("sum", vec![], "values", builtin_sum_impl)
    );
    
    functions.insert(
        "product".to_string(),
        FunctionSignature::new_variadic("product", vec![], "values", builtin_product_impl)
    );
    
    functions
}