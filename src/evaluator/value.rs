//! # Value Types
//!
//! This module defines the `Value` enum, which represents all possible runtime values
//! in the BCCL interpreter. Values are the result of evaluating expressions and
//! are stored in variables.
//!
//! ## Type System
//!
//! BCCL supports six core value types:
//! - **Number**: 64-bit floating point numbers (3.14, 42.0)
//! - **Integer**: 64-bit signed integers (42, -17)
//! - **Boolean**: true/false values
//! - **String**: UTF-8 text ("hello world")
//! - **List**: Ordered collections ([1, 2, 3])
//! - **Dictionary**: Key-value mappings ({"key": "value"})
//!
//! ## Type Coercion
//!
//! The system includes smart type coercion:
//! - Integers can be used as numbers (42 → 42.0)
//! - Numbers can be integers if they have no fractional part
//! - All types have truthiness semantics for logical operations

use std::collections::HashMap;

/// Represents all possible runtime values in BCCL.
/// 
/// This enum is the core of BCCL's type system. Every expression evaluates to a `Value`,
/// and every variable stores a `Value`. The type supports deep cloning and debug output.
/// 
/// # Examples
/// 
/// ```rust
/// let num = Value::Number(3.14);
/// let list = Value::List(vec![Value::Integer(1), Value::Integer(2)]);
/// let dict = Value::Dictionary(HashMap::from([("key".to_string(), Value::String("value".to_string()))]));
/// ```
/// 
/// # Type Hierarchy
/// 
/// - Primitive types: Number, Integer, Boolean, String
/// - Collection types: List, Dictionary
/// - All types support equality comparison and truthiness testing
#[derive(Debug, Clone)]
pub enum Value {
    /// 64-bit floating point number (IEEE 754)
    Number(f64),
    /// 64-bit signed integer
    Integer(i64),
    /// Boolean true/false value
    Boolean(bool),
    /// UTF-8 string
    String(String),
    /// Ordered list of values (can contain mixed types)
    List(Vec<Value>),
    /// String-keyed dictionary/map of values
    Dictionary(HashMap<String, Value>),
}

impl Value {
    /// Attempts to convert this value to a floating-point number.
    /// 
    /// This method provides type coercion from integers to numbers,
    /// which is essential for arithmetic operations that accept both types.
    /// 
    /// # Returns
    /// 
    /// * `Some(f64)` - If this is a Number or Integer
    /// * `None` - If this is any other type
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert_eq!(Value::Number(3.14).as_number(), Some(3.14));
    /// assert_eq!(Value::Integer(42).as_number(), Some(42.0));
    /// assert_eq!(Value::Boolean(true).as_number(), None);
    /// ```
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
    
    /// Attempts to convert this value to an integer.
    /// 
    /// Numbers can be converted to integers if they have no fractional part.
    /// This enables seamless interop between integers and whole numbers.
    /// 
    /// # Returns
    /// 
    /// * `Some(i64)` - If this is an Integer or a whole Number
    /// * `None` - If this is a fractional Number or any other type
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert_eq!(Value::Integer(42).as_integer(), Some(42));
    /// assert_eq!(Value::Number(42.0).as_integer(), Some(42));
    /// assert_eq!(Value::Number(3.14).as_integer(), None);
    /// ```
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            Value::Number(n) if n.fract() == 0.0 => Some(*n as i64),
            _ => None,
        }
    }
    
    /// Attempts to convert this value to a boolean.
    /// 
    /// Only returns `Some` for actual Boolean values. For truthiness testing
    /// (which all types support), use `is_truthy()` instead.
    /// 
    /// # Returns
    /// 
    /// * `Some(bool)` - If this is a Boolean value
    /// * `None` - If this is any other type
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert_eq!(Value::Boolean(true).as_boolean(), Some(true));
    /// assert_eq!(Value::Integer(1).as_boolean(), None); // Use is_truthy() instead
    /// ```
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    /// Attempts to get a string reference from this value.
    /// 
    /// Only returns `Some` for actual String values. For string representation
    /// of any value, use `display()` instead.
    /// 
    /// # Returns
    /// 
    /// * `Some(&str)` - If this is a String value
    /// * `None` - If this is any other type
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert_eq!(Value::String("hello".to_string()).as_string(), Some("hello"));
    /// assert_eq!(Value::Integer(42).as_string(), None); // Use display() instead
    /// ```
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    
    /// Returns the type name of this value as a string.
    /// 
    /// Used primarily for error messages to clearly indicate what type
    /// was found when a different type was expected.
    /// 
    /// # Returns
    /// 
    /// A static string representing the type name:
    /// - "number" for floating-point numbers
    /// - "integer" for whole numbers
    /// - "boolean" for true/false values
    /// - "string" for text values
    /// - "list" for ordered collections
    /// - "dictionary" for key-value maps
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert_eq!(Value::Number(3.14).type_name(), "number");
    /// assert_eq!(Value::List(vec![]).type_name(), "list");
    /// ```
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Integer(_) => "integer", 
            Value::Boolean(_) => "boolean",
            Value::String(_) => "string",
            Value::List(_) => "list",
            Value::Dictionary(_) => "dictionary",
        }
    }
    
    /// Returns a string representation of this value for display.
    /// 
    /// This method provides user-friendly string representations suitable
    /// for REPL output and debugging. Collections are displayed recursively.
    /// 
    /// # Format Rules
    /// 
    /// - Numbers: Display as integers if whole (42.0 → "42"), otherwise with decimals
    /// - Integers: Plain numeric display (42 → "42")
    /// - Booleans: "true" or "false"
    /// - Strings: Quoted ("hello" → "\"hello\"")
    /// - Lists: Bracketed with comma separation ([1, 2, 3])
    /// - Dictionaries: Braced with key-value pairs ({"a": 1, "b": 2})
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert_eq!(Value::Number(42.0).display(), "42");
    /// assert_eq!(Value::String("hello".to_string()).display(), "\"hello\"");
    /// ```
    pub fn display(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    format!("{}", n)
                }
            }
            Value::Integer(i) => format!("{}", i),
            Value::Boolean(b) => format!("{}", b),
            Value::String(s) => format!("\"{}\"", s),
            Value::List(items) => {
                let items_str: Vec<String> = items.iter().map(|v| v.display()).collect();
                format!("[{}]", items_str.join(", "))
            }
            Value::Dictionary(dict) => {
                let pairs: Vec<String> = dict.iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.display()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }
    
    /// Tests whether this value is "truthy" for logical operations.
    /// 
    /// BCCL follows Python-like truthiness rules where most values are truthy
    /// except for explicit "empty" or "zero" values.
    /// 
    /// # Truthiness Rules
    /// 
    /// - **Boolean**: `true` is truthy, `false` is falsy
    /// - **Number**: Non-zero numbers are truthy, `0.0` is falsy
    /// - **Integer**: Non-zero integers are truthy, `0` is falsy
    /// - **String**: Non-empty strings are truthy, `""` is falsy
    /// - **List**: Non-empty lists are truthy, `[]` is falsy
    /// - **Dictionary**: Non-empty dictionaries are truthy, `{}` is falsy
    /// 
    /// # Returns
    /// 
    /// `true` if the value should be considered truthy, `false` otherwise.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert_eq!(Value::Boolean(true).is_truthy(), true);
    /// assert_eq!(Value::Integer(0).is_truthy(), false);
    /// assert_eq!(Value::String("".to_string()).is_truthy(), false);
    /// assert_eq!(Value::List(vec![Value::Integer(1)]).is_truthy(), true);
    /// ```
    /// 
    /// # Usage in Logical Operations
    /// 
    /// This method is used by `and`, `or`, and `not` operators:
    /// - `true and "hello"` → `"hello"` (both are truthy)
    /// - `0 or 42` → `42` (0 is falsy, 42 is truthy)
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Integer(i) => *i != 0,
            Value::String(s) => !s.is_empty(),
            Value::List(items) => !items.is_empty(),
            Value::Dictionary(dict) => !dict.is_empty(),
        }
    }
}