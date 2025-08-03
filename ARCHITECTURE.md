# BCCL Interpreter Architecture

## Overview

The BCCL (Basic Calculator and Control Language) interpreter is a tree-walking interpreter implemented in Rust. It follows the classic three-stage interpretation pipeline: **Lexical Analysis** → **Parsing** → **Evaluation**. This document provides a comprehensive technical guide for engineers working on the codebase.

## System Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Source Code   │    │     Tokens      │    │  Abstract       │
│   "x = 42"      │───▶│ [Identifier(x), │───▶│  Syntax Tree    │
│                 │    │  Assign, Int(42)]│    │  (AST)          │
└─────────────────┘    └─────────────────┘    └─────────────────┘
        ▲                        ▲                        ▲
        │                        │                        │
     Input                   Lexer                    Parser
        │                   Stage                     Stage
        ▼                        ▼                        ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│      REPL       │    │   Error         │    │   Evaluator     │
│   Interface     │◀───│   Handling      │◀───│   Execution     │
│                 │    │   (miette)      │    │   Engine        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Core Components

### 1. Lexer (`src/lexer/`)

The lexer transforms raw source code into a stream of tokens.

#### Key Responsibilities:
- **Character Processing**: Reads UTF-8 input character by character
- **Token Recognition**: Identifies keywords, operators, literals, identifiers
- **Whitespace Handling**: Skips whitespace but preserves source positions
- **Error Detection**: Reports invalid characters and malformed tokens

#### Token Types:
```rust
pub enum TokenType {
    // Literals
    Integer(i64),        // 42, -17
    Number(f64),         // 3.14, -2.7
    String(String),      // "hello world"
    Boolean(bool),       // true, false
    
    // Identifiers & Keywords
    Identifier(String),  // variable_name, function_name
    
    // Operators
    Plus, Minus, Star, Slash,           // +, -, *, /
    Equal, NotEqual,                    // ==, !=
    Less, Greater, LessEqual, GreaterEqual, // <, >, <=, >=
    And, Or, Not,                       // and, or, not
    In, NotIn,                          // in, not in
    
    // Assignment
    Assign,              // =
    PlusAssign, MinusAssign, StarAssign, SlashAssign, // +=, -=, *=, /=
    
    // Delimiters
    LeftParen, RightParen,    // (, )
    LeftBracket, RightBracket, // [, ]
    LeftBrace, RightBrace,     // {, }
    Comma, Colon,             // ,, :
    
    // Control
    EOF,                 // End of input
}
```

#### Implementation Details:
- **Modular Design**: Split into focused modules (token types, readers, core logic)
- **Span Tracking**: Every token includes source position for error reporting
- **Lookahead**: Single-character lookahead for multi-character operators
- **Error Recovery**: Continues parsing after encountering invalid characters

### 2. Parser (`src/parser/`)

The parser builds an Abstract Syntax Tree (AST) from the token stream using recursive descent parsing.

#### Grammar (Simplified EBNF):
```ebnf
program         = statement*
statement       = assignment | compound_assignment | expression
assignment      = IDENTIFIER "=" expression
compound_assign = IDENTIFIER ("+="|"-="|"*="|"/=") expression
expression      = logical_or
logical_or      = logical_and ("or" logical_and)*
logical_and     = equality ("and" equality)*
equality        = comparison (("=="|"!=") comparison)*
comparison      = term (("<"|">"|"<="|">="|"in"|"not in") term)*
term            = factor (("+"|"-") factor)*
factor          = unary (("*"|"/") unary)*
unary           = ("not"|"-"|"+") unary | primary
primary         = NUMBER | INTEGER | BOOLEAN | STRING | IDENTIFIER
                | function_call | list | dictionary | index
                | "(" expression ")"
function_call   = IDENTIFIER "(" argument_list? ")"
argument_list   = expression ("," expression)* 
                | (expression ("," expression)*)? kwarg_list
kwarg_list      = IDENTIFIER "=" expression ("," IDENTIFIER "=" expression)*
list            = "[" (expression ("," expression)*)? "]"
dictionary      = "{" (dict_pair ("," dict_pair)*)? "}"
dict_pair       = STRING ":" expression
index           = primary "[" expression "]"
```

#### AST Node Types:
```rust
pub enum Expr {
    Number { value: f64, span: Span },
    Integer { value: i64, span: Span },
    Boolean { value: bool, span: Span },
    String { value: String, span: Span },
    Identifier { name: String, span: Span },
    Binary { left: Box<Expr>, operator: BinaryOp, right: Box<Expr>, span: Span },
    Unary { operator: UnaryOp, operand: Box<Expr>, span: Span },
    FunctionCall { name: String, args: Vec<Expr>, kwargs: Vec<(String, Expr)>, span: Span },
    List { elements: Vec<Expr>, span: Span },
    Dictionary { pairs: Vec<(String, Expr)>, span: Span },
    Index { object: Box<Expr>, index: Box<Expr>, span: Span },
}

pub enum Stmt {
    Expression { expr: Expr, span: Span },
    Assignment { name: String, value: Expr, span: Span },
    CompoundAssignment { name: String, operator: CompoundOp, value: Expr, span: Span },
}
```

#### Parsing Strategies:
- **Precedence Climbing**: Handles operator precedence correctly
- **Left Recursion Elimination**: Converts left-recursive rules to iterative parsing
- **Error Recovery**: Synchronizes on statement boundaries after errors
- **Span Preservation**: Every AST node includes source location information

### 3. Evaluator (`src/evaluator/`)

The evaluator executes the AST using the visitor pattern with environment-based variable storage.

#### Value System:
```rust
pub enum Value {
    Number(f64),                    // IEEE 754 double precision
    Integer(i64),                   // 64-bit signed integer
    Boolean(bool),                  // true/false
    String(String),                 // UTF-8 text
    List(Vec<Value>),              // Heterogeneous ordered collection
    Dictionary(HashMap<String, Value>), // String-keyed map
}
```

#### Type Coercion Rules:
- **Integer ↔ Number**: Seamless conversion (42 can be used as 42.0)
- **Truthiness**: All values have boolean interpretation
  - `false`, `0`, `0.0`, `""`, `[]`, `{}` are falsy
  - Everything else is truthy
- **Equality**: Smart comparison with type coercion
  - `42 == 42.0` → `true`
  - Different types are never equal

#### Environment Management:
```rust
pub struct Environment {
    variables: HashMap<String, Value>,
}
```
- **Single Scope**: Currently global scope only
- **Dynamic Typing**: Variables can hold any value type
- **Mutable by Default**: All variables can be reassigned
- **O(1) Lookup**: HashMap-based for efficiency

#### Function System:
```rust
pub struct FunctionSignature {
    name: String,
    parameters: Vec<String>,
    function: fn(&[Value]) -> BcclResult<Value>,
}
```

**Built-in Functions:**
- `max(a, b)`: Returns larger of two numbers
- `min(a, b)`: Returns smaller of two numbers

**Function Call Process:**
1. **Argument Evaluation**: All arguments evaluated left-to-right
2. **Parameter Matching**: Positional and keyword arguments resolved
3. **Validation**: Check argument count, types, parameter names
4. **Execution**: Call implementation with validated arguments
5. **Error Handling**: Rich error messages with source spans

### 4. Error System (`src/error/`)

BCCL provides compiler-quality error messages using the miette crate.

#### Error Categories:
```rust
pub enum BcclError {
    // Lexical errors
    InvalidCharacter { char: char, span: SourceSpan },
    MalformedNumber { input: String, span: SourceSpan },
    
    // Parse errors  
    UnexpectedToken { expected: String, found: String, span: SourceSpan },
    MissingClosingParen { opening_span: SourceSpan },
    
    // Runtime errors
    UndefinedVariable { name: String, span: SourceSpan, suggestions: Vec<String> },
    TypeMismatch { expected: String, found: String, span: SourceSpan },
    DivisionByZero { dividend_span: SourceSpan, divisor_span: SourceSpan },
    
    // Function errors
    UndefinedFunction { name: String, span: SourceSpan, available: Vec<String> },
    WrongArgumentCount { function: String, expected: usize, found: usize },
    FunctionArgumentTypeError { function_name: String, arg_number: usize, expected_type: String, actual_type: String, span: SourceSpan },
    
    // Collection errors
    IndexOutOfBounds { collection_type: String, index: usize, length: usize, span: SourceSpan },
    KeyNotFound { key: String, span: SourceSpan, available_keys: Vec<String> },
}
```

#### Error Reporting Features:
- **Source Code Display**: Shows the problematic line with context
- **Precise Pointers**: `^` indicators point to exact error locations
- **Help Messages**: Constructive suggestions for common mistakes
- **Variable Suggestions**: "Did you mean X?" for typos
- **Color Coding**: Different colors for different error types
- **Span Information**: Rich source location metadata

#### Example Error Output:
```
bccl::runtime::undefined_variable

  × Undefined variable 'ys'
   ╭─[input:1:1]
 1 │ x + ys
   ·     ─┬
   ·      ╰── undefined variable
   ╰────
  help: Did you mean 'xs'? Available variables: xs, count, result
```

## Key Design Patterns

### 1. Span-Aware Programming
Every token, AST node, and error includes source span information:
```rust
#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
```
This enables precise error reporting and could support IDE features in the future.

### 2. Error Propagation
Rust's `Result` type is used throughout for error handling:
```rust
pub type BcclResult<T> = Result<T, BcclError>;
```
Errors bubble up through the call stack with context preservation.

### 3. Visitor Pattern (Evaluator)
The evaluator uses the visitor pattern to traverse and execute AST nodes:
```rust
impl Evaluator {
    fn evaluate_expression(&self, expr: &Expr) -> BcclResult<Value> {
        match expr {
            Expr::Number { value, .. } => Ok(Value::Number(*value)),
            Expr::Binary { left, operator, right, span } => {
                self.evaluate_binary_expression(left, operator, right, *span)
            }
            // ... other expressions
        }
    }
}
```

### 4. Builder Pattern (Error Construction)
Errors are constructed using builder-like static methods:
```rust
impl BcclError {
    pub fn undefined_variable(name: &str, span: Span, suggestions: &[String]) -> Self {
        Self::UndefinedVariable {
            name: name.to_string(),
            span: span.into(),
            suggestions: suggestions.to_vec(),
        }
    }
}
```

## Module Organization

```
src/
├── lib.rs                      # Public API and re-exports
├── main.rs                     # REPL implementation
├── lexer/                      # Tokenization
│   ├── mod.rs                 # Core lexer logic
│   ├── token.rs               # Token type definitions
│   ├── readers.rs             # Specialized reading methods
│   └── tests.rs               # Lexer test suite
├── parser/                     # AST construction
│   ├── mod.rs                 # Core parser logic
│   ├── ast.rs                 # AST type definitions
│   ├── expressions.rs         # Expression parsing
│   ├── statements.rs          # Statement parsing
│   └── collections.rs         # List/dict parsing
├── evaluator/                  # Execution engine
│   ├── mod.rs                 # Core evaluation logic
│   ├── value.rs               # Value type system
│   ├── environment.rs         # Variable storage
│   ├── builtins.rs           # Built-in functions
│   └── tests.rs              # Evaluator test suite
├── error/                      # Error handling
│   ├── mod.rs                # Error organization
│   ├── span.rs               # Source span utilities
│   ├── types.rs              # Error type definitions
│   ├── lexer_errors.rs       # Lexer error constructors
│   ├── parser_errors.rs      # Parser error constructors
│   ├── runtime_errors.rs     # Runtime error constructors
│   └── context.rs            # Error context management
├── test_errors/               # Error testing
│   ├── mod.rs                # Basic error tests
│   ├── lexer_errors.rs       # Lexer error tests
│   ├── parser_errors.rs      # Parser error tests
│   ├── evaluator_errors.rs   # Evaluator error tests
│   ├── integration_errors.rs # Integration tests
│   └── error_recovery.rs     # Error recovery tests
├── verification_tests.rs      # End-to-end verification
└── demo_errors.rs            # Error demonstration
```

## Performance Characteristics

### Lexer
- **Time Complexity**: O(n) where n is input length
- **Space Complexity**: O(n) for token storage
- **Optimization**: Single-character lookahead, no backtracking

### Parser
- **Time Complexity**: O(n) for valid input, O(n²) worst case with error recovery
- **Space Complexity**: O(d) where d is maximum expression depth
- **Optimization**: Recursive descent without backtracking

### Evaluator
- **Time Complexity**: O(n) where n is number of AST nodes
- **Space Complexity**: O(v + d) where v is variables, d is call depth
- **Optimization**: HashMap-based variable lookup (O(1))

### Memory Management
- **Zero-Copy Lexing**: String slices where possible
- **AST Ownership**: Box<> for recursive structures
- **Value Cloning**: Deep cloning for variable assignment (could be optimized)

## Testing Strategy

### Test Coverage
- **Unit Tests**: Each module has comprehensive unit tests
- **Integration Tests**: Full pipeline testing with real input
- **Error Tests**: Comprehensive error condition coverage
- **Edge Cases**: Boundary conditions and malformed input

### Test Categories
1. **Lexer Tests** (11 tests): Token recognition, error handling
2. **Parser Tests** (embedded): AST construction, syntax errors
3. **Evaluator Tests** (29 tests): Expression evaluation, function calls
4. **Error Tests** (22 tests): Error message quality, span accuracy
5. **Integration Tests** (12+ tests): End-to-end scenarios

### Example Test Structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_call_with_kwargs() {
        let input = "max(a=5, b=10)";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        let mut evaluator = Evaluator::new();
        let result = evaluator.evaluate_program(&program).unwrap();
        assert_eq!(result, Some(Value::Number(10.0)));
    }
}
```

## Extension Points

### Adding New Language Features

#### 1. New Operators
1. Add token type to `lexer/token.rs`
2. Update lexer recognition in `lexer/mod.rs`
3. Add to `BinaryOp` or `UnaryOp` enum in `parser/ast.rs`
4. Implement parsing in `parser/expressions.rs`
5. Add evaluation logic in `evaluator/mod.rs`
6. Write comprehensive tests

#### 2. New Built-in Functions
1. Implement function in `evaluator/builtins.rs`
2. Add to function registry in `get_builtin_functions()`
3. Update dispatch logic in `FunctionSignature::call_impl`
4. Add tests for the new function

#### 3. New Value Types
1. Add variant to `Value` enum in `evaluator/value.rs`
2. Implement display, type checking, and conversion methods
3. Update equality comparison in `values_equal()`
4. Add literal parsing if needed (lexer + parser)
5. Update error messages to handle new type

### Future Architecture Improvements

#### 1. Scoped Environments
Current global scope could be extended to support:
- Function local scopes
- Block scoping for control structures
- Module-level isolation

#### 2. Bytecode Compilation
The tree-walking interpreter could be replaced/supplemented with:
- Bytecode generation phase
- Virtual machine execution
- Better performance characteristics

#### 3. Static Analysis
Could add optional static analysis phase:
- Type checking (optional static typing)
- Dead code detection
- Variable usage analysis

## Common Development Tasks

### Debugging Parser Issues
1. **Add Print Debugging**: Insert `println!` in parsing methods
2. **Token Inspection**: Check token stream with lexer tests
3. **AST Visualization**: Implement `Debug` display for nodes
4. **Span Verification**: Ensure spans are correct for error reporting

### Adding Error Messages
1. **Identify Error Condition**: Where should error be detected?
2. **Add Error Variant**: Update `BcclError` enum
3. **Implement Constructor**: Add static method in appropriate error module
4. **Update Error Site**: Replace generic error with specific one
5. **Test Error Message**: Verify error output quality

### Performance Profiling
1. **Benchmark Suite**: Create representative test inputs
2. **Profiling Tools**: Use `cargo flamegraph` or similar
3. **Memory Analysis**: Track allocations and cloning
4. **Optimization**: Focus on hot paths identified by profiling

## Conclusion

The BCCL interpreter demonstrates clean architecture with clear separation of concerns. The modular design makes it easy to understand, modify, and extend. The comprehensive error handling provides excellent developer experience, and the test suite ensures reliability.

Key strengths:
- **Clean Architecture**: Well-separated, focused modules
- **Rich Error Messages**: Compiler-quality diagnostics
- **Comprehensive Testing**: High test coverage across all components
- **Extensible Design**: Easy to add new features
- **Type Safety**: Rust's type system prevents many common interpreter bugs

This architecture provides a solid foundation for future language features and optimizations.