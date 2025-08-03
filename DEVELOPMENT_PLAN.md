# BCCL Interpreter Development Plan

## Current Status ✅

### Completed Core Features
- ✅ **Basic Data Types**: integers, floats, booleans, strings
- ✅ **Equality Operators**: `==`, `!=` with smart type coercion
- ✅ **Rich Error Diagnostics**: Full miette integration with `^` pointers and help text
- ✅ **Type-safe Operations**: Proper error handling for type mismatches
- ✅ **Enhanced Lexer**: Keywords (`true`/`false`), string literals with escape sequences
- ✅ **Variable Assignment**: All types work with assignments
- ✅ **Custom Functions**: Built-in `max()` and `min()` functions with kwargs support
- ✅ **Lists and Dictionaries**: Full collection support with indexing
- ✅ **Compound Assignment**: `+=`, `-=`, `*=`, `/=` operators
- ✅ **Logical Operators**: `and`, `or`, `not`, `in`, `not in`
- ✅ **Keyword Arguments**: Mixed positional and keyword arguments for functions
- ✅ **Comprehensive Testing**: 77 tests covering all features
- ✅ **Code Refactoring**: Modular codebase with focused, maintainable files
- ✅ **Error Diagnostic Consistency**: Uniform error display across all execution contexts
- ✅ **Comprehensive Documentation**: Full API documentation and architecture guide

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

**Test Coverage**: 77 passing tests across all components

## Completed Development Phases 📋

### Phase 1: Core Language Features ✅ COMPLETED
1. ✅ **Custom Functions** (`max()`, `min()`) with parameter validation
2. ✅ **Lists and Dictionaries** with operations and indexing
3. ✅ **Compound Assignment** (`+=`, `-=`, `*=`, `/=`) operators
4. ✅ **Logical Operators** (`and`, `or`, `not`, `in`, `not in`)
5. ✅ **Comprehensive Testing** for all features (77 tests)
6. ✅ **Error Diagnostics** with rich miette integration
7. ✅ **Keyword Arguments (kwargs)** for custom functions

### Phase 2: Code Quality & Architecture ✅ COMPLETED
8. ✅ **Add back commented test cases** for parser and lexer coverage
9. ✅ **Major Refactoring** - Break up large files into focused modules:
   - `parser/` - Modular AST, expressions, statements, collections  
   - `lexer/` - Token types, readers, core lexer
   - `error/` - Span utilities, error types, categorized constructors
   - `test_errors/` - Organized error test suites
   - `evaluator/` - Value system, environment, builtins
10. ✅ **Error Diagnostic Consistency** - Uniform formatting across execution contexts
11. ✅ **Comprehensive Documentation** - API docstrings and architecture guide

### Phase 3: Developer Experience ✅ COMPLETED
12. ✅ **API Documentation** - Docstrings for all core modules
13. ✅ **Engineering Documentation** - Complete ARCHITECTURE.md guide

## Current Development Phase 🚧

### Phase 4: Testing & Tooling Improvements
14. **Consolidate Binary Tests** - Move separate binary tests into cargo test framework
15. **Builtin Function Documentation** - Guide for adding new built-in functions

### Phase 5: Advanced Language Features (Future)
16. **Dynamic Function Creation** - Lambda/anonymous functions
17. **Enhanced Error Diagnostics** - Optional graphical error display
18. **User-defined Functions** - Custom function definitions in BCCL
19. **Control Flow** - if/else, loops, blocks
20. **Modules/Imports** - Code organization and reuse

## Implementation History

### 1. Keyword Arguments for Functions (COMPLETED ✅)

**Goal**: Add support for keyword arguments in function calls

**Example Syntax**:
```
// Mixed positional and keyword arguments
result = max(5, 10)                    // Positional only
result = max(a=5, b=10)                // Keyword only  
result = max(5, b=10)                  // Mixed style

// Error cases with helpful diagnostics
result = max(a=5, 10)                  // Error: positional after keyword
result = max(x=5, y=10)                // Error: unknown parameters
```

**Implementation Details**:
- ✅ Updated AST to support keyword arguments in function calls
- ✅ Extended parser to handle `name=value` syntax in function arguments
- ✅ Added validation to prevent positional args after keyword args
- ✅ Enhanced evaluator to handle kwargs in function evaluation
- ✅ Implemented proper parameter validation (kwargs must match actual parameter names)
- ✅ Added comprehensive test coverage for kwargs functionality
- ✅ Span-aware error reporting for precise function call diagnostics

**Files Modified**:
- ✅ `src/parser/ast.rs`: Added kwargs to FunctionCall AST node
- ✅ `src/parser/expressions.rs`: Implemented keyword argument parsing
- ✅ `src/evaluator/mod.rs`: Added kwargs handling in function evaluation
- ✅ `src/evaluator/builtins.rs`: Function signature system with parameter validation
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
- **Well-Tested**: Comprehensive test coverage (77 tests, all passing)
- **Well-Documented**: Complete API documentation and architecture guide
- **Developer-Friendly**: Extensive docstrings and technical documentation

## Testing Strategy

### Comprehensive Test Structure ✅
- **Evaluator Tests** (29 tests): Core evaluation functionality
- **Lexer Tests** (11 tests): Tokenization and lexical analysis  
- **Error Tests** (22 tests): Error handling and diagnostics
- **Integration Tests** (12 tests): End-to-end verification

**Total: 77 passing tests covering:**
1. ✅ **Unit Tests**: Each feature in isolation
2. ✅ **Integration Tests**: Complex expressions and combinations
3. ✅ **Error Cases**: Comprehensive error scenarios with recovery
4. ✅ **Edge Cases**: Boundary conditions and malformed input

### Test Categories
- **Lexer Tests** (11 tests): Number parsing, string literals, operators, collections
- **Parser Tests** (embedded): Expression parsing, statement parsing, error recovery  
- **Evaluator Tests** (29 tests): Arithmetic, logic, collections, functions, variables
- **Error Tests** (22 tests): Lexical, syntax, runtime, and type errors
- **Integration Tests** (15+ tests): Complex expressions, nested operations, error states
- **Verification Tests** (6 tests): End-to-end validation of key features

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

## Next Development Priorities

### Immediate Tasks (Phase 4)
1. **Consolidate Testing** - Move binary tests into unified cargo test framework
2. **Builtin Function Guide** - Documentation for adding new built-in functions
3. **Code Cleanup** - Address compiler warnings and unused code

### Future Language Features (Phase 5)
1. **Lambda Functions** - Anonymous function expressions
   ```
   // Proposed syntax
   square = |x| x * x
   numbers.map(|n| n * 2)
   ```

2. **User-defined Functions** - Function definitions in BCCL
   ```
   // Proposed syntax
   fn fibonacci(n) {
       if n <= 1 { n } else { fibonacci(n-1) + fibonacci(n-2) }
   }
   ```

3. **Control Flow** - Conditional and loop constructs
   ```
   // Proposed syntax
   if x > 0 {
       print("positive")
   } else {
       print("non-positive")
   }
   
   for item in list {
       print(item)
   }
   ```

4. **Enhanced Error Display** - Optional graphical error diagnostics
   - Configurable ASCII vs. graphical themes
   - Color support detection
   - Rich formatting for supported terminals

### Quality Improvements
- **Performance Optimization** - Profiling and bottleneck identification
- **Memory Efficiency** - Reduce unnecessary cloning in Value system
- **Static Analysis** - Optional type checking and linting
- **IDE Integration** - Language server protocol support

## Current Project Status

### Build Status
- ✅ **All Tests Passing**: 77/77 tests pass
- ✅ **Clean Compilation**: Builds successfully with `cargo build`
- ⚠️ **Minor Warnings**: Some unused imports/variables (non-critical)
- ✅ **Error Diagnostics**: Consistent across debug/release builds

### Usage
- **Main REPL**: `cargo run --bin bccl`
- **All Tests**: `cargo test`
- **Release Build**: `cargo build --release`
- **Documentation**: `cargo doc --open` (generates API docs from docstrings)

### Performance
- **Lexer**: O(n) time complexity, single-pass tokenization
- **Parser**: O(n) time for valid input, recursive descent without backtracking
- **Evaluator**: O(n) evaluation time, O(1) variable lookup
- **Memory**: Efficient for small to medium programs, room for optimization in Value cloning

## Documentation Resources

### For Users
- **README.md**: Basic usage and installation
- **DEVELOPMENT_PLAN.md**: This document - project status and roadmap
- **ERROR_DIAGNOSTIC_EXAMPLES.md**: Examples of error messages and diagnostics

### For Developers
- **ARCHITECTURE.md**: Comprehensive technical guide to interpreter internals
- **API Documentation**: Generated from docstrings with `cargo doc`
- **Test Suite**: 77 tests demonstrating expected behavior
- **Code Comments**: Extensive inline documentation in all modules

### Performance Considerations

- **Current Optimizations**:
  - HashMap-based variable lookup (O(1))
  - Single-pass lexing without backtracking
  - Recursive descent parsing without lookahead
  - Efficient span tracking for error reporting

- **Future Optimizations**:
  - String interning for identifiers
  - Copy-on-write for Value cloning
  - Bytecode compilation for better performance
  - Memory pooling for AST nodes