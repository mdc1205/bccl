# BCCL Interpreter Testing Guide

This document explains the testing strategy and processes for the BCCL interpreter project.

## Overview

The BCCL interpreter uses a comprehensive testing approach that covers:
- **Unit Tests**: Individual component testing
- **Integration Tests**: Feature combination testing
- **Error Scenario Tests**: Comprehensive error handling verification
- **Performance Tests**: Basic performance validation

## Test Organization

### Test Binaries

The project includes several test binaries in `src/bin/` for different components:

1. **`test_comprehensive.rs`** - Main comprehensive test suite
2. **`test_basic_types.rs`** - Basic data types and operations
3. **`test_functions.rs`** - Custom function testing
4. **`test_lists_dicts.rs`** - Lists and dictionaries with indexing
5. **`test_compound_assignment.rs`** - Compound assignment operators
6. **`test_logical_operators.rs`** - Logical operations and comparisons
7. **`test_errors.rs`** - Error handling and diagnostics
8. **`test_integration.rs`** - Cross-feature integration tests

### Running Tests

```bash
# Run all individual test binaries
cargo run --bin test_comprehensive     # Main test suite
cargo run --bin test_basic_types       # Basic types
cargo run --bin test_functions         # Functions
cargo run --bin test_lists_dicts       # Collections
cargo run --bin test_compound_assignment # Compound ops
cargo run --bin test_logical_operators  # Logic ops
cargo run --bin test_errors            # Error handling
cargo run --bin test_integration       # Integration

# Run the main REPL for manual testing
cargo run --bin bccl

# Build project (checks compilation)
cargo build

# Run with release optimizations
cargo build --release
cargo run --release --bin test_comprehensive
```

## Test Categories

### 1. Basic Data Types (`test_basic_types.rs`)

Tests fundamental data types and operations:
- **Numbers**: Integers and floats, arithmetic operations
- **Booleans**: True/false values, logical operations
- **Strings**: String literals, escape sequences
- **Variables**: Assignment, retrieval, scoping

**Example Tests:**
```rust
42 → 42                    // Integer literals
3.14 → 3.14                // Float literals
true → true                // Boolean literals
"hello" → "hello"          // String literals
x = 10; x → 10             // Variable assignment
```

### 2. Custom Functions (`test_functions.rs`)

Tests built-in function system:
- **Function calls**: Argument parsing, return values
- **Built-in functions**: `max()`, `min()`
- **Error handling**: Wrong argument count, type errors

**Example Tests:**
```rust
max(5, 10) → 10            // Function calls
max(3.14, 2) → 3.14        // Mixed types
min(1, 2) → 1              // Multiple functions
max(1) → Error             // Wrong argument count
```

### 3. Lists and Dictionaries (`test_lists_dicts.rs`)

Tests collection data types:
- **List literals**: `[1, 2, 3]`, empty lists
- **Dictionary literals**: `{"key": "value"}`, nested structures
- **Indexing operations**: `list[0]`, `dict["key"]`
- **Error cases**: Index out of bounds, key not found

**Example Tests:**
```rust
[1, 2, 3] → [1, 2, 3]      // List creation
{"name": "John"} → {"name": "John"} // Dictionary creation
[1, 2, 3][1] → 2           // List indexing
{"a": 1}["a"] → 1          // Dictionary indexing
[1, 2][5] → Error          // Index out of bounds
```

### 4. Compound Assignment (`test_compound_assignment.rs`)

Tests compound assignment operators:
- **Arithmetic compounds**: `+=`, `-=`, `*=`, `/=`
- **Expression evaluation**: Right-hand side expressions
- **Error handling**: Undefined variables, division by zero

**Example Tests:**
```rust
x = 10; x += 5 → 15        // Addition assignment
y = 20; y -= 8 → 12        // Subtraction assignment
z = 3; z *= 4 → 12         // Multiplication assignment
w = 15; w /= 3 → 5         // Division assignment
unknown += 5 → Error       // Undefined variable
```

### 5. Logical Operators (`test_logical_operators.rs`)

Tests logical and comparison operations:
- **Logical operators**: `and`, `or`, `not`
- **Comparison operators**: `<`, `>`, `<=`, `>=`
- **Membership operators**: `in`, `not in`
- **Short-circuit evaluation**: Proper evaluation order
- **Truthiness**: All data types

**Example Tests:**
```rust
true and false → false     // Logical AND
true or false → true       // Logical OR
not true → false           // Logical NOT
5 < 10 → true              // Comparison
1 in [1, 2, 3] → true      // Membership
"key" in {"key": "val"} → true // Dictionary membership
```

### 6. Error Handling (`test_errors.rs`)

Tests comprehensive error diagnostics:
- **Lexical errors**: Invalid characters, malformed numbers
- **Parse errors**: Syntax errors, unexpected tokens
- **Runtime errors**: Type errors, undefined variables
- **Error formatting**: Rich diagnostics with `^` pointers

**Example Tests:**
```rust
@invalid → LexError        // Invalid character
1.2.3 → MalformedNumber    // Bad number format
x + → UnexpectedEof        // Incomplete expression
undefined_var → UndefinedVariable // Variable not found
"text" + 5 → TypeError     // Type mismatch
```

### 7. Integration Tests (`test_integration.rs`)

Tests feature combinations and complex scenarios:
- **Multi-feature expressions**: Combining all language features
- **Complex data structures**: Nested lists and dictionaries
- **Advanced logic**: Complex boolean expressions
- **Real-world scenarios**: Practical use cases

**Example Tests:**
```rust
// Complex expression combining multiple features
nums = [1, 2, 3, 4, 5]
result = max(nums[0], nums[4]) + 10
result *= 2
is_large = result > 20 and result < 50

// Nested data structures
data = {
  "users": [
    {"name": "Alice", "active": true},
    {"name": "Bob", "active": false}
  ]
}
active_users = data["users"][0]["active"]
```

## Test Execution Strategy

### 1. Automated Testing
- Each test binary runs automatically and reports pass/fail status
- Tests use expected vs actual result comparison
- Error cases verify proper error types and messages

### 2. Manual Testing
- REPL interface for interactive testing
- Real-time error display with rich diagnostics
- Variable state inspection

### 3. Regression Testing
- All existing tests must pass after new feature additions
- Test cases cover edge cases and boundary conditions
- Error scenarios ensure proper failure modes

## Error Testing Strategy

### Error Categories Tested

1. **Lexical Errors**
   - Invalid characters in source code
   - Malformed number literals
   - Unterminated strings

2. **Parse Errors**
   - Syntax errors and malformed expressions
   - Unexpected tokens
   - Missing required tokens

3. **Runtime Errors**
   - Type mismatches in operations
   - Undefined variables and functions
   - Index out of bounds
   - Division by zero
   - Key not found in dictionaries

### Error Quality Verification

Tests verify that errors include:
- **Clear error messages**: Human-readable descriptions
- **Source location**: Exact position of the error
- **Helpful suggestions**: Hints for fixing the error
- **Rich formatting**: Visual `^` pointers and context

## Performance Considerations

### Performance Tests Include

1. **Large Data Structures**
   - Lists with hundreds of elements
   - Dictionaries with many key-value pairs
   - Nested collection performance

2. **Complex Expressions**
   - Deeply nested arithmetic
   - Long chains of logical operations
   - Function call overhead

3. **Memory Usage**
   - Variable storage efficiency
   - Collection memory management
   - Garbage collection behavior

## Test Maintenance

### Adding New Tests

When adding new features:
1. Create specific test cases in relevant test binary
2. Add integration tests showing feature combinations
3. Include error cases for the new feature
4. Update this documentation

### Test Organization Principles

- **One test per feature**: Each major feature has dedicated tests
- **Comprehensive coverage**: Test both success and failure paths
- **Clear test names**: Descriptive test case descriptions
- **Expected results**: All tests specify expected outcomes
- **Error verification**: Error tests check specific error types

## Continuous Integration

### Build Verification
- All test binaries must compile successfully
- No compilation warnings for core functionality
- Release builds must succeed

### Test Execution
- All individual test suites must pass
- Integration tests must demonstrate feature interaction
- Error tests must verify proper diagnostic quality

### Quality Gates
- New features require corresponding tests
- All existing tests must continue passing
- Error handling must be comprehensive

## Example Test Session

```bash
# Complete testing workflow
cargo build                           # Verify compilation
cargo run --bin test_comprehensive    # Run main test suite
cargo run --bin test_integration      # Run integration tests
cargo run --bin bccl                  # Manual REPL testing

# Example REPL session
> x = [1, 2, 3]
[1, 2, 3]
> x[1] + max(5, 10)
17
> person = {"name": "Alice", "age": 30}
{"name": "Alice", "age": 30}
> person["name"] in ["Alice", "Bob"]
true
```

This comprehensive testing approach ensures the BCCL interpreter is robust, reliable, and provides excellent error diagnostics for users.