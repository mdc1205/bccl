//! # Evaluator Module
//!
//! The evaluator is the core execution engine of the BCCL interpreter. It takes parsed
//! Abstract Syntax Trees (ASTs) and executes them, managing variables, function calls,
//! and all runtime operations.
//!
//! ## Architecture Overview
//!
//! The evaluator consists of several key components:
//! - **Evaluator**: Main struct that orchestrates evaluation
//! - **Environment**: Variable storage and scoping
//! - **Value**: Runtime value representation
//! - **Builtins**: Built-in function implementations
//!
//! ## Key Features
//!
//! - **Type-safe evaluation**: All operations include comprehensive type checking
//! - **Rich error reporting**: Precise span information for all errors
//! - **Function calls**: Support for both positional and keyword arguments
//! - **Collections**: Lists and dictionaries with indexing and membership testing
//! - **Compound operations**: +=, -=, *=, /= operators
//! - **Logical operations**: and, or, not, in, not in with short-circuit evaluation

use std::collections::HashMap;
use crate::parser::{Expr, Stmt, Program, BinaryOp, UnaryOp, CompoundOp};
use crate::error::{BcclError, BcclResult, Span};

mod value;
mod environment;
mod builtins;

#[cfg(test)]
mod tests;

pub use value::Value;
pub use environment::Environment;
pub use builtins::{builtin_max, builtin_min, FunctionSignature, get_builtin_functions};

/// The main evaluator struct that executes BCCL programs.
/// 
/// The evaluator maintains:
/// - An environment for variable storage
/// - A registry of built-in functions
/// 
/// # Example
/// 
/// ```rust
/// let mut evaluator = Evaluator::new();
/// let result = evaluator.evaluate_program(&program)?;
/// ```
pub struct Evaluator {
    /// Variable storage and scoping environment
    environment: Environment,
    /// Registry of built-in functions with their signatures
    functions: HashMap<String, FunctionSignature>,
}

impl Evaluator {
    /// Creates a new evaluator with an empty environment and built-in functions.
    /// 
    /// The evaluator starts with:
    /// - Empty variable environment
    /// - Pre-loaded built-in functions (max, min, etc.)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let evaluator = Evaluator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            functions: get_builtin_functions(),
        }
    }
    
    /// Evaluates a complete BCCL program.
    /// 
    /// A program consists of multiple statements. Each statement is evaluated in sequence,
    /// and the result of the last expression statement is returned. Assignment statements
    /// also return their assigned value.
    /// 
    /// # Arguments
    /// 
    /// * `program` - The parsed program to evaluate
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(value))` - The value of the last expression
    /// * `Ok(None)` - If the program contains no expressions
    /// * `Err(error)` - If any statement fails to evaluate
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let program = parser.parse()?;
    /// match evaluator.evaluate_program(&program)? {
    ///     Some(value) => println!("{}", value.display()),
    ///     None => {} // No output needed
    /// }
    /// ```
    pub fn evaluate_program(&mut self, program: &Program) -> BcclResult<Option<Value>> {
        let mut last_value = None;
        
        for stmt in &program.statements {
            last_value = self.evaluate_statement(stmt)?;
        }
        
        Ok(last_value)
    }
    
    /// Evaluates a single statement.
    /// 
    /// Statements in BCCL include:
    /// - Expression statements: `42`, `x + y`
    /// - Assignment statements: `x = 42`
    /// - Compound assignment: `x += 5`
    /// 
    /// # Arguments
    /// 
    /// * `stmt` - The statement to evaluate
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(value))` - The resulting value of the statement
    /// * `Ok(None)` - For statements that don't produce values
    /// * `Err(error)` - If the statement fails to evaluate
    fn evaluate_statement(&mut self, stmt: &Stmt) -> BcclResult<Option<Value>> {
        match stmt {
            Stmt::Expression { expr, span: _ } => {
                let value = self.evaluate_expression(expr)?;
                Ok(Some(value))
            }
            Stmt::Assignment { name, value, span: _ } => {
                let evaluated_value = self.evaluate_expression(value)?;
                self.environment.define(name.clone(), evaluated_value.clone());
                Ok(Some(evaluated_value))
            }
            Stmt::CompoundAssignment { name, operator, value, span } => {
                // Get the current value of the variable
                let current_value = self.environment
                    .get(name)
                    .cloned()
                    .ok_or_else(|| {
                        let available_vars = self.environment.variable_names();
                        BcclError::undefined_variable(name, *span, &available_vars)
                    })?;
                
                // Evaluate the right-hand side
                let rhs_value = self.evaluate_expression(value)?;
                
                // Perform the compound operation
                let result = self.perform_compound_operation(&current_value, operator, &rhs_value, name, *span)?;
                
                // Store the result
                self.environment.define(name.clone(), result.clone());
                Ok(Some(result))
            }
        }
    }
    
    /// Evaluates an expression and returns its value.
    /// 
    /// This is the core evaluation method that handles all expression types:
    /// - Literals: numbers, booleans, strings
    /// - Variables: identifier lookups
    /// - Binary operations: arithmetic, comparison, logical
    /// - Unary operations: negation, logical not
    /// - Function calls: built-in function invocation
    /// - Collections: lists and dictionaries
    /// - Indexing: list[index] and dict[key]
    /// 
    /// # Arguments
    /// 
    /// * `expr` - The expression to evaluate
    /// 
    /// # Returns
    /// 
    /// * `Ok(value)` - The resulting value
    /// * `Err(error)` - If evaluation fails (type errors, undefined variables, etc.)
    fn evaluate_expression(&self, expr: &Expr) -> BcclResult<Value> {
        match expr {
            Expr::Number { value, span: _ } => Ok(Value::Number(*value)),
            Expr::Integer { value, span: _ } => Ok(Value::Integer(*value)),
            Expr::Boolean { value, span: _ } => Ok(Value::Boolean(*value)),
            Expr::String { value, span: _ } => Ok(Value::String(value.clone())),
            Expr::Identifier { name, span } => {
                self.environment
                    .get(name)
                    .cloned()
                    .ok_or_else(|| {
                        let available_vars = self.environment.variable_names();
                        BcclError::undefined_variable(name, *span, &available_vars)
                    })
            }
            Expr::Binary { left, operator, right, span } => {
                self.evaluate_binary_expression(left, operator, right, *span)
            }
            Expr::Unary { operator, operand, span: _ } => {
                let operand_val = self.evaluate_expression(operand)?;
                
                match operator {
                    UnaryOp::Minus | UnaryOp::Plus => {
                        let operand_num = operand_val.as_number()
                            .ok_or_else(|| BcclError::type_error(
                                "Operand must be a number", 
                                operand.span(), 
                                "number", 
                                operand_val.type_name()
                            ))?;
                        
                        let result = match operator {
                            UnaryOp::Minus => -operand_num,
                            UnaryOp::Plus => operand_num,
                            _ => unreachable!(),
                        };
                        
                        Ok(Value::Number(result))
                    }
                    UnaryOp::Not => {
                        let result = !operand_val.is_truthy();
                        Ok(Value::Boolean(result))
                    }
                }
            }
            Expr::FunctionCall { name, args, kwargs, span } => {
                // Evaluate all positional arguments with their spans
                let mut arg_values_with_spans = Vec::new();
                for arg in args {
                    let value = self.evaluate_expression(arg)?;
                    arg_values_with_spans.push((value, arg.span()));
                }
                
                // Evaluate all keyword arguments with their spans
                let mut kwarg_values_with_spans = Vec::new();
                for (param_name, value_expr) in kwargs {
                    let value = self.evaluate_expression(value_expr)?;
                    kwarg_values_with_spans.push((param_name.clone(), value, value_expr.span()));
                }
                
                // Look up the function signature and call with proper parameter validation
                if let Some(func_sig) = self.functions.get(name) {
                    func_sig.call_with_spans(&arg_values_with_spans, &kwarg_values_with_spans, *span)
                } else {
                    let available_functions: Vec<String> = self.functions.keys().cloned().collect();
                    Err(BcclError::undefined_function(name, *span, &available_functions))
                }
            }
            Expr::List { elements, span: _ } => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.evaluate_expression(element)?);
                }
                Ok(Value::List(values))
            }
            Expr::Dictionary { pairs, span: _ } => {
                let mut dict = HashMap::new();
                for (key, value_expr) in pairs {
                    let value = self.evaluate_expression(value_expr)?;
                    dict.insert(key.clone(), value);
                }
                Ok(Value::Dictionary(dict))
            }
            Expr::Index { object, index, span } => {
                self.evaluate_index_expression(object, index, *span)
            }
        }
    }
    
    /// Evaluates binary expressions (e.g., `a + b`, `x == y`).
    /// 
    /// Handles all binary operations including:
    /// - Arithmetic: +, -, *, /
    /// - Comparison: ==, !=, <, >, <=, >=
    /// - Logical: and, or
    /// - Membership: in, not in
    /// 
    /// # Arguments
    /// 
    /// * `left` - Left operand expression
    /// * `operator` - Binary operator
    /// * `right` - Right operand expression
    /// * `span` - Source location for error reporting
    /// 
    /// # Returns
    /// 
    /// * `Ok(value)` - Result of the binary operation
    /// * `Err(error)` - Type errors, division by zero, etc.
    fn evaluate_binary_expression(&self, left: &Expr, operator: &BinaryOp, right: &Expr, span: Span) -> BcclResult<Value> {
        let left_val = self.evaluate_expression(left)?;
        let right_val = self.evaluate_expression(right)?;
        
        match operator {
            // Arithmetic operations - require numbers
            BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => {
                let left_num = left_val.as_number()
                    .ok_or_else(|| BcclError::type_error(
                        "Left operand must be a number", 
                        left.span(), 
                        "number", 
                        left_val.type_name()
                    ))?;
                    
                let right_num = right_val.as_number()
                    .ok_or_else(|| BcclError::type_error(
                        "Right operand must be a number", 
                        right.span(), 
                        "number", 
                        right_val.type_name()
                    ))?;
                
                let result = match operator {
                    BinaryOp::Add => left_num + right_num,
                    BinaryOp::Subtract => left_num - right_num,
                    BinaryOp::Multiply => left_num * right_num,
                    BinaryOp::Divide => {
                        if right_num == 0.0 {
                            return Err(BcclError::division_by_zero(span, right.span()));
                        }
                        left_num / right_num
                    }
                    _ => unreachable!(),
                };
                
                Ok(Value::Number(result))
            }
            
            // Equality operations - work with any types
            BinaryOp::Equal => {
                let result = self.values_equal(&left_val, &right_val);
                Ok(Value::Boolean(result))
            }
            BinaryOp::NotEqual => {
                let result = !self.values_equal(&left_val, &right_val);
                Ok(Value::Boolean(result))
            }
            
            // Comparison operations - require numbers
            BinaryOp::Less | BinaryOp::Greater | BinaryOp::LessEqual | BinaryOp::GreaterEqual => {
                let op_str = match operator {
                    BinaryOp::Less => "<",
                    BinaryOp::Greater => ">", 
                    BinaryOp::LessEqual => "<=",
                    BinaryOp::GreaterEqual => ">=",
                    _ => unreachable!(),
                };
                
                let left_num = left_val.as_number()
                    .ok_or_else(|| BcclError::logical_operation_error(
                        op_str,
                        &format!("Cannot compare {} with numbers", left_val.type_name()),
                        left.span()
                    ))?;
                    
                let right_num = right_val.as_number()
                    .ok_or_else(|| BcclError::logical_operation_error(
                        op_str,
                        &format!("Cannot compare numbers with {}", right_val.type_name()),
                        right.span()
                    ))?;
                
                let result = match operator {
                    BinaryOp::Less => left_num < right_num,
                    BinaryOp::Greater => left_num > right_num,
                    BinaryOp::LessEqual => left_num <= right_num,
                    BinaryOp::GreaterEqual => left_num >= right_num,
                    _ => unreachable!(),
                };
                
                Ok(Value::Boolean(result))
            }
            
            // Logical operations - use truthiness
            BinaryOp::And => {
                // Short-circuit evaluation: if left is falsy, return left
                if !left_val.is_truthy() {
                    Ok(left_val)
                } else {
                    Ok(right_val)
                }
            }
            BinaryOp::Or => {
                // Short-circuit evaluation: if left is truthy, return left
                if left_val.is_truthy() {
                    Ok(left_val)
                } else {
                    Ok(right_val)
                }
            }
            
            // Membership operations
            BinaryOp::In => {
                self.evaluate_membership(&left_val, &right_val, left, right, false)
            }
            BinaryOp::NotIn => {
                self.evaluate_membership(&left_val, &right_val, left, right, true)
            }
        }
    }
    
    /// Evaluates indexing expressions (e.g., `list[0]`, `dict["key"]`).
    /// 
    /// Supports:
    /// - List indexing with integers: `[1,2,3][0]` → `1`
    /// - Dictionary lookup with strings: `{"a": 1}["a"]` → `1`
    /// 
    /// # Arguments
    /// 
    /// * `object` - The collection being indexed
    /// * `index` - The index or key expression
    /// * `span` - Source location for error reporting
    /// 
    /// # Returns
    /// 
    /// * `Ok(value)` - The value at the specified index/key
    /// * `Err(error)` - Index out of bounds, key not found, type errors
    fn evaluate_index_expression(&self, object: &Expr, index: &Expr, span: Span) -> BcclResult<Value> {
        let obj_value = self.evaluate_expression(object)?;
        let index_value = self.evaluate_expression(index)?;
        
        match (&obj_value, &index_value) {
            (Value::List(items), Value::Integer(i)) => {
                let idx = *i as usize;
                if idx < items.len() {
                    Ok(items[idx].clone())
                } else {
                    Err(BcclError::index_out_of_bounds("list", idx, items.len(), span))
                }
            }
            (Value::Dictionary(dict), Value::String(key)) => {
                dict.get(key)
                    .cloned()
                    .ok_or_else(|| BcclError::key_not_found(key, span, &dict.keys().cloned().collect::<Vec<_>>()))
            }
            (Value::List(_), _) => {
                Err(BcclError::collection_operation_error(
                    "index",
                    &format!("List indices must be integers, not {}", index_value.type_name()),
                    index.span()
                ))
            }
            (Value::Dictionary(_), _) => {
                Err(BcclError::collection_operation_error(
                    "index",
                    &format!("Dictionary keys must be strings, not {}", index_value.type_name()),
                    index.span()
                ))
            }
            (_, _) => {
                Err(BcclError::collection_operation_error(
                    "index",
                    &format!("Cannot index {} values - only lists and dictionaries support indexing", obj_value.type_name()),
                    object.span()
                ))
            }
        }
    }
    
    /// Evaluates membership expressions (`in` and `not in`).
    /// 
    /// Tests whether a value is contained within a collection:
    /// - `5 in [1, 2, 5]` → `true`
    /// - `"key" in {"key": "value"}` → `true`
    /// 
    /// # Arguments
    /// 
    /// * `left_val` - The value to search for
    /// * `right_val` - The collection to search in
    /// * `left` - Left expression (for error reporting)
    /// * `right` - Right expression (for error reporting)
    /// * `negate` - Whether this is `not in` (true) or `in` (false)
    /// 
    /// # Returns
    /// 
    /// * `Ok(Boolean(found))` - Whether the value was found
    /// * `Err(error)` - If the right operand is not a collection
    fn evaluate_membership(&self, left_val: &Value, right_val: &Value, left: &Expr, right: &Expr, negate: bool) -> BcclResult<Value> {
        let found = match right_val {
            Value::List(items) => {
                items.iter().any(|item| self.values_equal(left_val, item))
            }
            Value::Dictionary(dict) => {
                if let Value::String(key) = left_val {
                    dict.contains_key(key)
                } else {
                    return Err(BcclError::collection_operation_error(
                        "membership",
                        &format!("Dictionary keys must be strings, not {}", left_val.type_name()),
                        left.span()
                    ));
                }
            }
            _ => {
                let op_name = if negate { "not in" } else { "in" };
                return Err(BcclError::collection_operation_error(
                    "membership",
                    &format!("Cannot use '{}' with {} - only lists and dictionaries support membership testing", op_name, right_val.type_name()),
                    right.span()
                ));
            }
        };
        
        Ok(Value::Boolean(if negate { !found } else { found }))
    }
    
    /// Gets the value of a variable by name.
    /// 
    /// Used primarily by the REPL to display variable values.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The variable name to look up
    /// 
    /// # Returns
    /// 
    /// * `Some(value)` - The variable's current value
    /// * `None` - If the variable is not defined
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.environment.get(name)
    }
    
    /// Gets a list of all defined variable names.
    /// 
    /// Used by the REPL's `:vars` command to show all variables.
    /// 
    /// # Returns
    /// 
    /// A vector of all currently defined variable names.
    pub fn get_variable_names(&self) -> Vec<String> {
        self.environment.variable_names()
    }
    
    /// Performs compound assignment operations (+=, -=, *=, /=).
    /// 
    /// Compound operations modify a variable by applying an operation between
    /// its current value and a new value.
    /// 
    /// # Arguments
    /// 
    /// * `current` - The variable's current value
    /// * `operator` - The compound operator to apply
    /// * `rhs` - The right-hand side value
    /// * `variable_name` - Name of the variable (for error messages)
    /// * `span` - Source location (for error reporting)
    /// 
    /// # Returns
    /// 
    /// * `Ok(value)` - The result of the compound operation
    /// * `Err(error)` - If the operation fails (type mismatch, division by zero)
    fn perform_compound_operation(&self, current: &Value, operator: &CompoundOp, rhs: &Value, variable_name: &str, span: Span) -> BcclResult<Value> {
        let operator_str = match operator {
            CompoundOp::Add => "+=",
            CompoundOp::Subtract => "-=",
            CompoundOp::Multiply => "*=",
            CompoundOp::Divide => "/=",
        };
        
        // For compound assignment, both operands must be numbers
        let current_num = current.as_number()
            .ok_or_else(|| BcclError::compound_assignment_error(
                variable_name,
                operator_str,
                &format!("Variable '{}' contains {} but {} requires a number", variable_name, current.type_name(), operator_str),
                span
            ))?;
            
        let rhs_num = rhs.as_number()
            .ok_or_else(|| BcclError::compound_assignment_error(
                variable_name,
                operator_str,
                &format!("Cannot use {} with {} value", operator_str, rhs.type_name()),
                span
            ))?;
        
        let result = match operator {
            CompoundOp::Add => current_num + rhs_num,
            CompoundOp::Subtract => current_num - rhs_num,
            CompoundOp::Multiply => current_num * rhs_num,
            CompoundOp::Divide => {
                if rhs_num == 0.0 {
                    return Err(BcclError::division_by_zero(span, span));
                }
                current_num / rhs_num
            }
        };
        
        Ok(Value::Number(result))
    }
    
    /// Compares two values for equality.
    /// 
    /// This method handles type coercion between integers and floats,
    /// and provides proper equality semantics for all BCCL types.
    /// 
    /// # Arguments
    /// 
    /// * `left` - The left value to compare
    /// * `right` - The right value to compare
    /// 
    /// # Returns
    /// 
    /// `true` if the values are considered equal, `false` otherwise.
    /// 
    /// # Type Coercion
    /// 
    /// - Numbers and integers can be compared (42 == 42.0)
    /// - Different types are never equal
    /// - Floating point comparison uses epsilon for precision
    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            
            // Allow comparison between integers and numbers
            (Value::Number(a), Value::Integer(b)) => (*a - *b as f64).abs() < f64::EPSILON,
            (Value::Integer(a), Value::Number(b)) => (*a as f64 - *b).abs() < f64::EPSILON,
            
            // Different types are not equal
            _ => false,
        }
    }
}