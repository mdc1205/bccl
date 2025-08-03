//! # Environment Module
//!
//! The environment manages variable storage and scoping for the BCCL interpreter.
//! Currently implements a single global scope, but the design allows for future
//! extension to nested scopes for functions and blocks.
//!
//! ## Variable Management
//!
//! - Variables are stored in a HashMap for O(1) lookup
//! - Variable names are case-sensitive
//! - All variables are mutable (can be reassigned)
//! - No variable declaration required (dynamic typing)
//!
//! ## Future Extensions
//!
//! The current design can be extended to support:
//! - Nested scopes for function calls
//! - Block scoping for control structures
//! - Immutable variable declarations
//! - Module-level variable isolation

use std::collections::HashMap;
use super::value::Value;

/// Manages variable storage and lookup for the BCCL interpreter.
/// 
/// The environment acts as a symbol table, storing variable names and their
/// associated values. Currently implements a single global scope where all
/// variables are accessible throughout the program execution.
/// 
/// # Design
/// 
/// - Uses HashMap for efficient O(1) variable lookup
/// - Variables can store any Value type
/// - Supports variable redefinition (assignments overwrite)
/// - Thread-safe operations (no interior mutability)
/// 
/// # Examples
/// 
/// ```rust
/// let mut env = Environment::new();
/// env.define("x".to_string(), Value::Integer(42));
/// assert_eq!(env.get("x"), Some(&Value::Integer(42)));
/// ```
pub struct Environment {
    /// Storage for variable name -> value mappings
    variables: HashMap<String, Value>,
}

impl Environment {
    /// Creates a new empty environment.
    /// 
    /// The environment starts with no variables defined. Variables are added
    /// through assignment statements or explicit `define()` calls.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let env = Environment::new();
    /// assert_eq!(env.variable_names().len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    /// Defines or updates a variable with the given name and value.
    /// 
    /// If the variable already exists, it will be overwritten with the new value.
    /// This is the primary method for variable assignment in BCCL.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The variable name (case-sensitive)
    /// * `value` - The value to store
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let mut env = Environment::new();
    /// env.define("x".to_string(), Value::Integer(42));
    /// env.define("x".to_string(), Value::String("hello".to_string())); // Overwrites
    /// ```
    /// 
    /// # Variable Naming Rules
    /// 
    /// - Names are case-sensitive ("X" and "x" are different)
    /// - Any valid UTF-8 string can be a variable name
    /// - No reserved words enforced at this level (handled by parser)
    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    /// Retrieves the value of a variable by name.
    /// 
    /// Returns a reference to the stored value if the variable exists,
    /// or None if the variable is not defined.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The variable name to look up
    /// 
    /// # Returns
    /// 
    /// * `Some(&Value)` - Reference to the variable's value
    /// * `None` - Variable not found
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let mut env = Environment::new();
    /// env.define("x".to_string(), Value::Integer(42));
    /// 
    /// assert_eq!(env.get("x"), Some(&Value::Integer(42)));
    /// assert_eq!(env.get("y"), None);
    /// ```
    /// 
    /// # Usage in Error Handling
    /// 
    /// When a variable lookup fails, the evaluator uses this None result
    /// to generate helpful "undefined variable" errors with suggestions.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    /// Returns a list of all currently defined variable names.
    /// 
    /// Used primarily for:
    /// - REPL `:vars` command to show all variables
    /// - Error messages with variable suggestions
    /// - Debugging and introspection
    /// 
    /// # Returns
    /// 
    /// A vector containing all variable names in arbitrary order.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let mut env = Environment::new();
    /// env.define("x".to_string(), Value::Integer(42));
    /// env.define("name".to_string(), Value::String("Alice".to_string()));
    /// 
    /// let names = env.variable_names();
    /// assert_eq!(names.len(), 2);
    /// assert!(names.contains(&"x".to_string()));
    /// assert!(names.contains(&"name".to_string()));
    /// ```
    /// 
    /// # Performance
    /// 
    /// This method clones all variable names, so it's O(n) in the number of variables.
    /// Use sparingly in performance-critical code.
    pub fn variable_names(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }
}