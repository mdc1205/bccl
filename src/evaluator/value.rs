use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
    List(Vec<Value>),
    Dictionary(HashMap<String, Value>),
}

impl Value {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
    
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            Value::Number(n) if n.fract() == 0.0 => Some(*n as i64),
            _ => None,
        }
    }
    
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    
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
    
    /// Check if this value is truthy (for logical operations)
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