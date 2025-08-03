# Adding Built-in Functions to BCCL

This guide explains how to add new built-in functions to the BCCL interpreter. Built-in functions are implemented in Rust and made available to BCCL programs through the function call syntax.

## Overview

Built-in functions in BCCL follow a specific pattern:
1. **Function Implementation**: Rust function that operates on `Value` types
2. **Function Signature**: Metadata describing parameters and validation
3. **Registry**: Registration of the function for lookup during evaluation
4. **Testing**: Comprehensive test coverage for the new function

## Step-by-Step Guide

### Step 1: Understand the Function System

BCCL uses a `FunctionSignature` system that provides:
- **Parameter validation**: Ensures correct argument count and types
- **Keyword argument support**: Functions can be called with `func(a=1, b=2)` syntax
- **Rich error reporting**: Precise error messages with source locations

### Step 2: Implement the Function Logic

Add your function implementation to `src/evaluator/builtins.rs`. Follow this pattern:

```rust
/// Implementation of the `your_function(param1, param2)` built-in function.
/// 
/// Brief description of what the function does.
/// 
/// # Arguments
/// 
/// * `args` - Function arguments in parameter order
/// * `spans` - Source spans for each argument (for error reporting)
/// 
/// # Returns
/// 
/// * `Ok(Value::Type(result))` - The function result
/// * `Err(error)` - If arguments are invalid or execution fails
/// 
/// # Examples
/// 
/// - `your_function(arg1, arg2)` → `expected_result`
/// 
/// # Type Requirements
/// 
/// Describe what types the function accepts and any coercion rules.
fn builtin_your_function_with_spans(args: &[Value], spans: &[Span]) -> BcclResult<Value> {
    // Validate argument count (handled by FunctionSignature, but good to document)
    assert_eq!(args.len(), 2, "Function signature ensures correct argument count");
    
    // Extract and validate first argument
    let param1 = match &args[0] {
        Value::Number(n) => *n,
        Value::Integer(i) => *i as f64,
        _ => return Err(BcclError::function_argument_type_error_with_span(
            "your_function", 1, "number", args[0].type_name(), &args[0].display(), spans[0]
        )),
    };
    
    // Extract and validate second argument  
    let param2 = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(BcclError::function_argument_type_error_with_span(
            "your_function", 2, "string", args[1].type_name(), &args[1].display(), spans[1]
        )),
    };
    
    // Implement your function logic here
    let result = your_actual_logic(param1, &param2);
    
    // Return the appropriate Value type
    Ok(Value::Number(result))
}

// Legacy implementation for backward compatibility (if needed)
fn builtin_your_function_impl(args: &[Value]) -> BcclResult<Value> {
    // Simple validation without spans
    let param1 = args[0].as_number()
        .ok_or_else(|| BcclError::function_argument_type_error("your_function", 1, "number", args[0].type_name()))?;
    
    let param2 = args[1].as_string()
        .ok_or_else(|| BcclError::function_argument_type_error("your_function", 2, "string", args[1].type_name()))?;
    
    let result = your_actual_logic(param1, param2);
    Ok(Value::Number(result))
}

// Your actual implementation logic
fn your_actual_logic(param1: f64, param2: &str) -> f64 {
    // Implement the core logic here
    param1 * param2.len() as f64
}
```

### Step 3: Add Function to Registry

Update the `get_builtin_functions()` function in `src/evaluator/builtins.rs`:

```rust
pub fn get_builtin_functions() -> HashMap<String, FunctionSignature> {
    let mut functions = HashMap::new();
    
    // Existing functions
    functions.insert(
        "max".to_string(), 
        FunctionSignature::new("max", vec!["a", "b"], builtin_max_impl)
    );
    
    functions.insert(
        "min".to_string(), 
        FunctionSignature::new("min", vec!["a", "b"], builtin_min_impl)
    );
    
    // Add your new function
    functions.insert(
        "your_function".to_string(),
        FunctionSignature::new("your_function", vec!["param1", "param2"], builtin_your_function_impl)
    );
    
    functions
}
```

### Step 4: Add Span-Aware Dispatch

Update the `call_impl` method in `FunctionSignature` to use your span-aware implementation:

```rust
fn call_impl(&self, args: &[Value], spans: &[Span]) -> BcclResult<Value> {
    match self.name.as_str() {
        "max" => builtin_max_with_spans(args, spans),
        "min" => builtin_min_with_spans(args, spans),
        "your_function" => builtin_your_function_with_spans(args, spans),
        _ => (self.function)(args), // Fallback to legacy function call
    }
}
```

### Step 5: Add Comprehensive Tests

Add tests to the appropriate test module. For built-in functions, add tests to the integration test suite in `tests/integration_tests.rs`:

```rust
#[cfg(test)]
mod your_function_tests {
    use super::*;

    #[test]
    fn test_your_function_basic() {
        eval_number("your_function(5.0, \"hello\")", 25.0);
        eval_number("your_function(3.14, \"test\")", 12.56);
    }

    #[test]
    fn test_your_function_kwargs() {
        eval_number("your_function(param1=5.0, param2=\"hello\")", 25.0);
        eval_number("your_function(5.0, param2=\"hello\")", 25.0);
    }

    #[test]
    fn test_your_function_with_variables() {
        eval_number("x = 5.0; text = \"hello\"; your_function(x, text)", 25.0);
    }

    #[test]
    fn test_your_function_errors() {
        // Wrong argument count
        eval_error("your_function(5.0)");
        eval_error("your_function(5.0, \"hello\", \"extra\")");
        
        // Wrong argument types
        eval_error("your_function(\"not_a_number\", \"hello\")");
        eval_error("your_function(5.0, 42)");
        
        // Invalid kwargs
        eval_error("your_function(x=5.0, y=\"hello\")");  // Unknown parameters
    }

    #[test]
    fn test_your_function_edge_cases() {
        // Test edge cases specific to your function
        eval_number("your_function(0, \"\")", 0.0);
        eval_number("your_function(-5.0, \"test\")", -20.0);
    }
}
```

## Common Patterns

### Type Validation Patterns

#### Numbers (accepts both integers and floats)
```rust
let num = match &args[0] {
    Value::Number(n) => *n,
    Value::Integer(i) => *i as f64,
    _ => return Err(BcclError::function_argument_type_error_with_span(
        function_name, 1, "number", args[0].type_name(), &args[0].display(), spans[0]
    )),
};
```

#### Integers only
```rust
let int_val = match &args[0] {
    Value::Integer(i) => *i,
    Value::Number(n) if n.fract() == 0.0 => *n as i64,
    _ => return Err(BcclError::function_argument_type_error_with_span(
        function_name, 1, "integer", args[0].type_name(), &args[0].display(), spans[0]
    )),
};
```

#### Booleans
```rust
let bool_val = match &args[0] {
    Value::Boolean(b) => *b,
    _ => return Err(BcclError::function_argument_type_error_with_span(
        function_name, 1, "boolean", args[0].type_name(), &args[0].display(), spans[0]
    )),
};
```

#### Strings
```rust
let string_val = match &args[0] {
    Value::String(s) => s.clone(),
    _ => return Err(BcclError::function_argument_type_error_with_span(
        function_name, 1, "string", args[0].type_name(), &args[0].display(), spans[0]
    )),
};
```

#### Lists
```rust
let list_val = match &args[0] {
    Value::List(items) => items.clone(),
    _ => return Err(BcclError::function_argument_type_error_with_span(
        function_name, 1, "list", args[0].type_name(), &args[0].display(), spans[0]
    )),
};
```

#### Dictionaries
```rust
let dict_val = match &args[0] {
    Value::Dictionary(dict) => dict.clone(),
    _ => return Err(BcclError::function_argument_type_error_with_span(
        function_name, 1, "dictionary", args[0].type_name(), &args[0].display(), spans[0]
    )),
};
```

### Return Value Patterns

Choose the appropriate return type for your function:

```rust
// Numbers (most common for mathematical functions)
Ok(Value::Number(result))

// Integers (for counting, indexing, etc.)
Ok(Value::Integer(result))

// Booleans (for predicates, comparisons)
Ok(Value::Boolean(result))

// Strings (for text processing)
Ok(Value::String(result))

// Lists (for collection operations)
Ok(Value::List(result))

// Dictionaries (for structured data)
Ok(Value::Dictionary(result))
```

## Example: Adding a `len()` Function

Let's walk through adding a `len()` function that returns the length of strings, lists, and dictionaries:

### 1. Implementation

```rust
/// Implementation of the `len(collection)` built-in function.
/// 
/// Returns the length/size of a collection or string.
/// 
/// # Arguments
/// 
/// * `args` - Function arguments [collection]
/// * `spans` - Source spans for error reporting
/// 
/// # Returns
/// 
/// * `Ok(Value::Integer(length))` - The length of the collection
/// * `Err(error)` - If argument is not a valid collection type
/// 
/// # Examples
/// 
/// - `len("hello")` → `5`
/// - `len([1, 2, 3])` → `3`
/// - `len({"a": 1, "b": 2})` → `2`
/// 
/// # Supported Types
/// 
/// - Strings: Returns character count
/// - Lists: Returns element count
/// - Dictionaries: Returns key count
fn builtin_len_with_spans(args: &[Value], spans: &[Span]) -> BcclResult<Value> {
    let length = match &args[0] {
        Value::String(s) => s.len() as i64,
        Value::List(items) => items.len() as i64,
        Value::Dictionary(dict) => dict.len() as i64,
        _ => return Err(BcclError::function_argument_type_error_with_span(
            "len", 1, "string, list, or dictionary", 
            args[0].type_name(), &args[0].display(), spans[0]
        )),
    };
    
    Ok(Value::Integer(length))
}

fn builtin_len_impl(args: &[Value]) -> BcclResult<Value> {
    let length = match &args[0] {
        Value::String(s) => s.len() as i64,
        Value::List(items) => items.len() as i64,
        Value::Dictionary(dict) => dict.len() as i64,
        _ => return Err(BcclError::function_argument_type_error(
            "len", 1, "string, list, or dictionary", args[0].type_name()
        )),
    };
    
    Ok(Value::Integer(length))
}
```

### 2. Registry

```rust
functions.insert(
    "len".to_string(),
    FunctionSignature::new("len", vec!["collection"], builtin_len_impl)
);
```

### 3. Dispatch

```rust
"len" => builtin_len_with_spans(args, spans),
```

### 4. Tests

```rust
#[cfg(test)]
mod len_function_tests {
    use super::*;

    #[test]
    fn test_len_strings() {
        eval_number("len(\"hello\")", 5.0);
        eval_number("len(\"\")", 0.0);
        eval_number("len(\"Hello, 世界!\")", 9.0);
    }

    #[test]
    fn test_len_lists() {
        eval_number("len([1, 2, 3])", 3.0);
        eval_number("len([])", 0.0);
        eval_number("len([\"a\", \"b\", \"c\", \"d\"])", 4.0);
    }

    #[test]
    fn test_len_dictionaries() {
        eval_number("len({\"a\": 1, \"b\": 2})", 2.0);
        eval_number("len({})", 0.0);
    }

    #[test]
    fn test_len_with_variables() {
        eval_number("text = \"hello\"; len(text)", 5.0);
        eval_number("nums = [1, 2, 3]; len(nums)", 3.0);
    }

    #[test]
    fn test_len_kwargs() {
        eval_number("len(collection=\"hello\")", 5.0);
    }

    #[test]
    fn test_len_errors() {
        eval_error("len()");            // No arguments
        eval_error("len(\"a\", \"b\")");  // Too many arguments
        eval_error("len(42)");          // Invalid type
        eval_error("len(true)");        // Invalid type
    }
}
```

## Best Practices

### Error Handling
- Use span-aware error reporting for better user experience
- Provide clear error messages indicating expected vs. actual types
- Include the actual value in error messages when helpful

### Function Naming
- Use clear, descriptive names (`len`, `abs`, `sqrt`)
- Follow common conventions from other languages when appropriate
- Avoid abbreviations unless they're very common (`max`, `min` are okay)

### Parameter Names
- Use descriptive parameter names that appear in error messages
- Keep names short but clear (`value`, `list`, `dict`, `start`, `end`)
- Consider how they'll look in kwargs: `substring(text="hello", start=1, end=3)`

### Documentation
- Include comprehensive docstrings with examples
- Document type requirements and coercion rules
- Explain any special behaviors or edge cases

### Testing
- Test all supported input types
- Test error conditions (wrong types, wrong argument count)
- Test kwargs functionality
- Test edge cases (empty collections, boundary values)
- Test integration with variables and expressions

### Performance
- For simple functions, avoid unnecessary allocations
- Clone values only when necessary
- Consider if the function could benefit from lazy evaluation

## Advanced Topics

### Variable Argument Count

For functions that accept variable numbers of arguments, you can validate manually:

```rust
fn builtin_sum_with_spans(args: &[Value], spans: &[Span]) -> BcclResult<Value> {
    if args.is_empty() {
        return Err(BcclError::wrong_argument_count("sum", 1, 0));
    }
    
    let mut total = 0.0;
    for (i, arg) in args.iter().enumerate() {
        let num = match arg {
            Value::Number(n) => *n,
            Value::Integer(i) => *i as f64,
            _ => return Err(BcclError::function_argument_type_error_with_span(
                "sum", i + 1, "number", arg.type_name(), &arg.display(), spans[i]
            )),
        };
        total += num;
    }
    
    Ok(Value::Number(total))
}
```

However, you'll need to handle parameter validation manually in this case.

### Side Effects

Built-in functions should generally be pure (no side effects). If you need side effects:
- Document them clearly
- Consider if they belong in the evaluator instead
- Be careful about thread safety if added in the future

### Complex Return Types

For functions that might return different types based on input:

```rust
fn builtin_type_of_with_spans(args: &[Value], _spans: &[Span]) -> BcclResult<Value> {
    let type_name = args[0].type_name();
    Ok(Value::String(type_name.to_string()))
}
```

## Summary

Adding built-in functions to BCCL involves:
1. Implementing the function logic with proper error handling
2. Registering the function with appropriate parameter names
3. Adding span-aware dispatch for better error reporting
4. Writing comprehensive tests

The key is following the established patterns for consistency and maintainability. The function system is designed to be extensible, so adding new functions should be straightforward once you understand the patterns.

## Variadic Functions - Advanced Feature

BCCL now supports **variadic functions** - functions that accept a variable number of arguments! This is perfect for functions like `sum()`, `product()`, `concat()`, etc.

### Creating Variadic Functions

Use `FunctionSignature::new_variadic()` instead of `new_fixed()`:

```rust
// Pure variadic: sum(values...)
FunctionSignature::new_variadic("sum", vec![], "values", builtin_sum_impl)

// Mixed: clamp(value, bounds...) - 1 required + variadic
FunctionSignature::new_variadic("clamp", vec!["value"], "bounds", builtin_clamp_impl)
```

### Variadic Function Implementation

```rust
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
```

### Usage Examples

```rust
// All valid calls:
sum(5)                          // → 5
sum(1, 2, 3)                   // → 6  
sum(1, 2, 3, 4, 5, 6, 7, 8, 9, 10) // → 55

// With keyword arguments:
sum(values=1, values=2, values=3)   // → 6
sum(1, values=2, values=3)          // → 6

// Mixed function example:
clamp(value=5, bounds=1, bounds=10) // → 5 (clamped between 1 and 10)
clamp(5, 1, 10)                     // → 5 (same result)
```

### Benefits of Variadic Functions

1. **Flexible API**: Users can call with any number of arguments
2. **Natural syntax**: `sum(1, 2, 3, 4, 5)` feels natural
3. **Keyword support**: `sum(values=1, values=2)` works automatically
4. **Rich errors**: Precise error messages for each argument
5. **Performance**: No need to create intermediate collections

### When to Use Variadic vs Fixed

**Use Variadic for:**
- Mathematical operations: `sum()`, `product()`, `average()`, `gcd()`
- Collection operations: `concat()`, `merge()`, `union()`
- Output functions: `print()`, `format()`
- Statistical functions: `mean()`, `median()`, `mode()`

**Use Fixed for:**
- Binary operations: `max(a, b)`, `min(a, b)`, `pow(base, exp)`
- Transformations: `substring(text, start, end)`, `round(number, decimals)`
- Type operations: `isinstance(value, type)`, `convert(value, target_type)`

### Current Built-in Functions

**Fixed Parameter Functions:**
- `max(a, b)` - Returns the larger of two numbers
- `min(a, b)` - Returns the smaller of two numbers

**Variadic Functions:**
- `sum(values...)` - Returns the sum of all numeric arguments
- `product(values...)` - Returns the product of all numeric arguments

The enhanced function system makes BCCL much more powerful and flexible. You can now easily implement functions like `one_hot()` that take varying numbers of arguments based on your specific needs!

Remember to always test your functions thoroughly with different argument counts and provide good error messages - this greatly improves the user experience of the BCCL interpreter!