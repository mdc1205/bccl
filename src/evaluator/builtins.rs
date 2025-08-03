use crate::error::{BcclError, BcclResult};
use super::value::Value;

// Built-in functions
pub fn builtin_max(args: &[Value]) -> BcclResult<Value> {
    if args.len() != 2 {
        return Err(BcclError::wrong_argument_count("max", 2, args.len()));
    }
    
    let a = args[0].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error("max", 1, "number", args[0].type_name()))?;
    let b = args[1].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error("max", 2, "number", args[1].type_name()))?;
    
    Ok(Value::Number(a.max(b)))
}

pub fn builtin_min(args: &[Value]) -> BcclResult<Value> {
    if args.len() != 2 {
        return Err(BcclError::wrong_argument_count("min", 2, args.len()));
    }
    
    let a = args[0].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error("min", 1, "number", args[0].type_name()))?;
    let b = args[1].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error("min", 2, "number", args[1].type_name()))?;
    
    Ok(Value::Number(a.min(b)))
}