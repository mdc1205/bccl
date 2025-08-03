use std::collections::HashMap;
use super::value::Value;

pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    pub fn variable_names(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }
}