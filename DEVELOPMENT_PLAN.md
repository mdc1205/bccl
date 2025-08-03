# BCCL Interpreter Development Plan

## Current Status ✅

### Completed Features
- ✅ **Basic Data Types**: integers, floats, booleans, strings
- ✅ **Equality Operators**: `==`, `!=` with smart type coercion
- ✅ **Rich Error Diagnostics**: Full miette integration with `^` pointers and help text
- ✅ **Type-safe Operations**: Proper error handling for type mismatches
- ✅ **Enhanced Lexer**: Keywords (`true`/`false`), string literals with escape sequences
- ✅ **Variable Assignment**: All types work with assignments
- ✅ **Custom Functions**: Built-in `max()` and `min()` functions
- ✅ **Lists and Dictionaries**: Full collection support with indexing
- ✅ **Compound Assignment**: `+=`, `-=`, `*=`, `/=` operators
- ✅ **Logical Operators**: `and`, `or`, `not`, `in`, `not in`
- ✅ **Comprehensive Testing**: 74 tests covering all features
- ✅ **Code Refactoring**: Modular codebase with focused, maintainable files

### Test Results
All features working perfectly with comprehensive test coverage:
```
Input: 42 → Result: 42
Input: true → Result: true  
Input: "hello" → Result: "hello"
Input: 10 == 10.0 → Result: true
Input: true != false → Result: true
Input: max(5, 10) → Result: 10
Input: [1, 2, 3][0] → Result: 1
Input: {"name": "John"}["name"] → Result: "John"
Input: x = 5; x += 3 → Result: 8
Input: true and false → Result: false
Input: 5 in [1, 2, 5] → Result: true
Input: max(a=5, b=10) → Result: 10
Input: max(5, b=10) → Result: 10
```

**Test Coverage**: 76 passing tests across all components

## Remaining Tasks 📋

### High Priority - ALL COMPLETED ✅
1. ✅ **Custom Functions** (`max()`, `min()`) - COMPLETED
2. ✅ **Lists and Dictionaries** with operations - COMPLETED
3. ✅ **Compound Assignment** (`+=`, `-=`, `*=`, `/=`) - COMPLETED
4. ✅ **Logical Operators** (`and`, `or`, `not`, `in`, `not in`) - COMPLETED
5. ✅ **Comprehensive Testing** for all features - COMPLETED
6. ✅ **Error Diagnostics** for new features - COMPLETED
7. ✅ **Keyword Arguments (kwargs)** for custom functions - COMPLETED

### Medium Priority - MAJOR REFACTORING COMPLETED ✅
7. ✅ **Add back commented test cases** for parser and lexer coverage - COMPLETED
8. ✅ **Refactor to smaller files** (break up 450+ line files) - COMPLETED
   - `parser/` - Modular AST, expressions, statements, collections  
   - `lexer/` - Token types, readers, core lexer
   - `error/` - Span utilities, error types, categorized constructors
   - `test_errors/` - Organized error test suites
9. **Add docstrings** to all code for better understanding

### Low Priority
10. **Create high-level documentation** explaining interpreter components
11. **Example Documentation**

## Implementation Plan

### 1. Keyword Arguments for Functions (COMPLETED ✅)

**Goal**: Add support for keyword arguments in function calls

**Example Syntax**:
```
// Mixed positional and keyword arguments
result = max(5, 10)                    // Current: positional only
result = max(a=5, b=10)                // New: keyword only  
result = max(5, b=10)                  // New: mixed style

// Error cases with helpful diagnostics
result = max(a=5, 10)                  // Error: positional after keyword
```

**Implementation Details**:
- ✅ Updated AST to support keyword arguments in function calls
- ✅ Extended parser to handle `name=value` syntax in function arguments
- ✅ Added validation to prevent positional args after keyword args
- ✅ Enhanced evaluator to handle kwargs in function evaluation
- ✅ Added comprehensive test coverage for kwargs functionality

**Files Modified**:
- ✅ `src/parser/ast.rs`: Added kwargs to FunctionCall AST node
- ✅ `src/parser/expressions.rs`: Implemented keyword argument parsing
- ✅ `src/evaluator/mod.rs`: Added kwargs handling in function evaluation
- ✅ `src/evaluator/tests.rs`: Added kwargs test cases
- ✅ `src/test_errors/parser_errors.rs`: Added kwargs error handling tests

### 2. Custom Functions (COMPLETED ✅)

**Goal**: Implement `max(a, b)` function as example

**Changes Needed**:
- Add `FunctionCall` expr type (already stubbed in parser.rs:35-39)
- Implement function call parsing in `parse_primary()`
- Add built-in function registry to evaluator
- Handle function arguments and return values

**Files to Modify**:
- `src/parser.rs`: Uncomment and implement FunctionCall parsing
- `src/evaluator.rs`: Add function registry and evaluation
- `src/lexer.rs`: May need function call syntax support

**Example Syntax**:
```
max(5, 10)     → 10
max(3.14, 2)   → 3.14
result = max(x, y)
```

### 2. Lists and Dictionaries

**Goal**: Support `[1, 2, 3]` and `{"key": "value"}` syntax

**Changes Needed**:
- Uncomment List/Dictionary expr types in parser.rs
- Implement list/dict parsing with `[`, `]`, `{`, `}`, `,`, `:`
- Add indexing support: `list[0]`, `dict["key"]`
- Update evaluator Value enum and methods
- Add list/dict operations (append, length, etc.)

**Example Syntax**:
```
nums = [1, 2, 3]
person = {"name": "John", "age": 30}
first = nums[0]
name = person["name"]
```

### 3. Compound Assignment Operators

**Goal**: Support `+=`, `-=`, `*=`, `/=`

**Changes Needed**:
- Uncomment CompoundAssignment in parser.rs:93-101
- Add compound assignment parsing logic
- Update evaluator to handle compound operations
- Ensure proper error handling

**Example Syntax**:
```
x = 10
x += 5     → x becomes 15
x *= 2     → x becomes 30
```

### 4. Logical Operators

**Goal**: Support `and`, `or`, `not`, `in`, `not in`

**Changes Needed**:
- Uncomment logical operators in BinaryOp enum
- Add precedence parsing for logical operations
- Implement short-circuit evaluation
- Add `not` as UnaryOp
- Implement `in`/`not in` for collections

**Example Syntax**:
```
true and false    → false
true or false     → true
not true          → false
5 in [1, 2, 5]    → true
```

## Architecture Notes

### Current Modular Structure ✅
```
src/
├── lib.rs                    # Module declarations and re-exports
├── main.rs                   # REPL with rich error display
├── lexer/                    # Tokenization (584 → 4 focused files)
│   ├── mod.rs               # Core Lexer struct and tokenization
│   ├── token.rs             # Token types and definitions
│   ├── readers.rs           # Specialized reading methods
│   └── tests.rs             # Comprehensive lexer tests
├── parser/                   # Parsing (826 → 5 focused files)
│   ├── mod.rs               # Core Parser struct and utilities
│   ├── ast.rs               # AST type definitions
│   ├── expressions.rs       # Expression parsing methods
│   ├── statements.rs        # Statement parsing methods
│   └── collections.rs       # List/dictionary parsing
├── evaluator/                # Evaluation (881 → 5 focused files)
│   ├── mod.rs               # Core Evaluator and main logic
│   ├── value.rs             # Value type and methods
│   ├── environment.rs       # Variable environment
│   ├── builtins.rs          # Built-in functions
│   └── tests.rs             # Comprehensive evaluator tests
├── error/                    # Error handling (526 → 6 focused files)
│   ├── mod.rs               # Module exports and organization
│   ├── span.rs              # Source span utilities
│   ├── types.rs             # Error type definitions
│   ├── lexer_errors.rs      # Lexer error constructors
│   ├── parser_errors.rs     # Parser error constructors
│   ├── runtime_errors.rs    # Runtime error constructors
│   └── context.rs           # Error context and source tracking
├── test_errors/              # Error testing (606 → 6 focused files)
│   ├── mod.rs               # Basic error tests
│   ├── lexer_errors.rs      # Lexer-specific error tests
│   ├── parser_errors.rs     # Parser-specific error tests
│   ├── evaluator_errors.rs  # Evaluator-specific error tests
│   ├── integration_errors.rs # Complex integration tests
│   └── error_recovery.rs    # Error recovery tests
├── verification_tests.rs     # End-to-end verification
└── demo_errors.rs           # Error demonstration examples
```

### Key Design Decisions
- **Modular Architecture**: Each component focused on single responsibility
- **Type Safety**: Proper error handling for all operations
- **Rich Diagnostics**: Rust-compiler-level error quality with helpful suggestions
- **Extensible**: Easy to add new functions, operators, and features
- **Maintainable**: Clean separation of concerns, no files over 450 lines
- **Well-Tested**: Comprehensive test coverage (74 tests, all passing)

## Testing Strategy

### Comprehensive Test Structure ✅
- **Evaluator Tests** (29 tests): Core evaluation functionality
- **Lexer Tests** (11 tests): Tokenization and lexical analysis  
- **Error Tests** (22 tests): Error handling and diagnostics
- **Integration Tests** (12 tests): End-to-end verification

**Total: 74 passing tests covering:**
1. ✅ **Unit Tests**: Each feature in isolation
2. ✅ **Integration Tests**: Complex expressions and combinations
3. ✅ **Error Cases**: Comprehensive error scenarios with recovery
4. ✅ **Edge Cases**: Boundary conditions and malformed input

### Test Categories
- **Lexer**: Number parsing, string literals, operators, collections
- **Parser**: Expression parsing, statement parsing, error recovery
- **Evaluator**: Arithmetic, logic, collections, functions, variables
- **Error Handling**: Lexical, syntax, runtime, and type errors
- **Integration**: Complex expressions, nested operations, error states

## Code Examples to Implement

### Functions
```rust
// Built-in functions
max(10, 20)           → 20
min(5, 3)             → 3
len([1, 2, 3])        → 3
len("hello")          → 5

// User-defined functions (future)
fn add(a, b) { a + b }
```

### Collections
```rust
// Lists
numbers = [1, 2, 3, 4]
numbers[0]            → 1
numbers += [5]        → [1, 2, 3, 4, 5]

// Dictionaries  
person = {"name": "Alice", "age": 25}
person["name"]        → "Alice"
person["city"] = "NYC"
```

### Logical Operations
```rust
// Boolean logic
result = (x > 5) and (y < 10)
found = item in collection
valid = not is_empty(data)
```

## Error Handling Requirements

### Maintain Current Quality
- Visual `^` pointers for all error types
- Helpful suggestions and context
- Source code display with line numbers
- Color-coded error categories

### New Error Types Needed
- Function call errors (wrong arguments, undefined functions)
- Index out of bounds (lists/dictionaries)
- Type errors for logical operations
- Collection operation errors

## Files with TODO Comments

### parser.rs
- Lines 22-40: Commented List, Dictionary, Index, FunctionCall
- Lines 73-80: Commented additional BinaryOps
- Lines 87-88: Commented UnaryOp::Not
- Lines 95-111: Commented CompoundAssignment

### evaluator.rs  
- Lines 11-13: Commented List/Dictionary Value types
- Lines 53-54, 70-79, 90-91: Commented display/type methods

## Next Session Prompt

When ready to continue:

```
I'm continuing development of the BCCL interpreter. Previously completed:
- Basic data types (integers, floats, booleans, strings) ✅
- Equality operators (==, !=) with type coercion ✅  
- Rich error diagnostics with ^ pointers ✅

Next priority: Implement custom functions with max() as example.

Current status: All basic types working. Need to uncomment and implement FunctionCall in parser.rs:35-39, add function registry to evaluator, and test max(a,b) function.

Please continue from the TODO list in DEVELOPMENT_PLAN.md, starting with custom functions.
```

## Compilation Notes

- Current warnings are expected (unused variables/imports)
- Build succeeds with `cargo build`
- Test with `cargo run --bin test_errors`
- Main REPL: `cargo run --bin bccl`

## Performance Considerations

- String interning for identifiers (future optimization)
- Efficient collection operations
- Memory management for large data structures
- Function call overhead minimization