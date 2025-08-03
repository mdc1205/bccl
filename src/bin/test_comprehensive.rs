use bccl::{Lexer, Parser, Evaluator};

struct TestResult {
    passed: usize,
    failed: usize,
    total: usize,
}

impl TestResult {
    fn new() -> Self {
        Self { passed: 0, failed: 0, total: 0 }
    }
    
    fn pass(&mut self) {
        self.passed += 1;
        self.total += 1;
    }
    
    fn fail(&mut self) {
        self.failed += 1;
        self.total += 1;
    }
    
    fn summary(&self) -> String {
        format!("{}/{} tests passed ({:.1}%)", 
                self.passed, self.total, 
                (self.passed as f64 / self.total as f64) * 100.0)
    }
}

fn test_expression(input: &str, expected: &str, results: &mut TestResult) -> bool {
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(_) => {
            results.fail();
            println!("  ❌ FAIL: Lexer error for '{}'", input);
            return false;
        }
    };
    
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(_) => {
            results.fail();
            println!("  ❌ FAIL: Parser error for '{}'", input);
            return false;
        }
    };
    
    let mut evaluator = Evaluator::new();
    match evaluator.evaluate_program(&program) {
        Ok(Some(value)) => {
            let result = value.display();
            if result == expected {
                results.pass();
                println!("  ✅ PASS: {} → {}", input, result);
                true
            } else {
                results.fail();
                println!("  ❌ FAIL: {} → {} (expected {})", input, result, expected);
                false
            }
        }
        Ok(None) => {
            results.fail();
            println!("  ❌ FAIL: No result for '{}'", input);
            false
        }
        Err(_) => {
            results.fail();
            println!("  ❌ FAIL: Evaluation error for '{}'", input);
            false
        }
    }
}

fn test_error_case(input: &str, description: &str, results: &mut TestResult) -> bool {
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(_) => {
            results.pass();
            println!("  ✅ PASS: {} - Lexer error as expected", description);
            return true;
        }
    };
    
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(_) => {
            results.pass();
            println!("  ✅ PASS: {} - Parser error as expected", description);
            return true;
        }
    };
    
    let mut evaluator = Evaluator::new();
    match evaluator.evaluate_program(&program) {
        Ok(_) => {
            results.fail();
            println!("  ❌ FAIL: {} - Should have failed but succeeded", description);
            false
        }
        Err(_) => {
            results.pass();
            println!("  ✅ PASS: {} - Runtime error as expected", description);
            true
        }
    }
}

fn test_basic_types(results: &mut TestResult) {
    println!("\n=== BASIC DATA TYPES ===");
    
    // Numbers
    test_expression("42", "42", results);
    test_expression("3.14", "3.14", results);
    test_expression("-5", "-5", results);
    test_expression("+10", "10", results);
    
    // Booleans
    test_expression("true", "true", results);
    test_expression("false", "false", results);
    
    // Strings
    test_expression("\"hello\"", "\"hello\"", results);
    test_expression("\"\"", "\"\"", results);
    test_expression("\"hello world\"", "\"hello world\"", results);
    
    // Variables
    test_expression("x = 10; x", "10", results);
    test_expression("name = \"Alice\"; name", "\"Alice\"", results);
    test_expression("flag = true; flag", "true", results);
    
    // Arithmetic
    test_expression("2 + 3", "5", results);
    test_expression("10 - 4", "6", results);
    test_expression("6 * 7", "42", results);
    test_expression("15 / 3", "5", results);
    test_expression("2 + 3 * 4", "14", results);
    test_expression("(2 + 3) * 4", "20", results);
    
    // Equality
    test_expression("5 == 5", "true", results);
    test_expression("5 != 3", "true", results);
    test_expression("10 == 10.0", "true", results);
    test_expression("true != false", "true", results);
}

fn test_functions(results: &mut TestResult) {
    println!("\n=== FUNCTIONS ===");
    
    // Built-in functions
    test_expression("max(5, 10)", "10", results);
    test_expression("max(3.14, 2)", "3.14", results);
    test_expression("min(5, 3)", "3", results);
    test_expression("min(10, 10)", "10", results);
    
    // Function with variables
    test_expression("x = 5; y = 10; max(x, y)", "10", results);
    test_expression("a = 3; b = 7; min(a, b)", "3", results);
    
    // Function error cases
    test_error_case("max(1)", "max() with wrong argument count", results);
    test_error_case("max(1, 2, 3)", "max() with too many arguments", results);
    test_error_case("undefined_function(1, 2)", "undefined function call", results);
}

fn test_collections(results: &mut TestResult) {
    println!("\n=== LISTS AND DICTIONARIES ===");
    
    // Lists
    test_expression("[1, 2, 3]", "[1, 2, 3]", results);
    test_expression("[]", "[]", results);
    test_expression("[42]", "[42]", results);
    test_expression("[1, \"hello\", true]", "[1, \"hello\", true]", results);
    
    // List indexing
    test_expression("[1, 2, 3][0]", "1", results);
    test_expression("[1, 2, 3][2]", "3", results);
    test_expression("[\"a\", \"b\", \"c\"][1]", "\"b\"", results);
    
    // List with variables
    test_expression("nums = [10, 20, 30]; nums[1]", "20", results);
    test_expression("items = [1, 2, 3]; items[0] + items[2]", "4", results);
    
    // Dictionaries
    test_expression("{\"name\": \"John\"}", "{\"name\": \"John\"}", results);
    test_expression("{}", "{}", results);
    test_expression("{\"key1\": \"value1\", \"key2\": \"value2\"}", "{\"key1\": \"value1\", \"key2\": \"value2\"}", results);
    
    // Dictionary indexing
    test_expression("{\"name\": \"Alice\"}[\"name\"]", "\"Alice\"", results);
    test_expression("{\"a\": 1, \"b\": 2}[\"b\"]", "2", results);
    
    // Dictionary with variables
    test_expression("person = {\"name\": \"Bob\", \"age\": 25}; person[\"name\"]", "\"Bob\"", results);
    test_expression("data = {\"x\": 10, \"y\": 20}; data[\"x\"] + data[\"y\"]", "30", results);
    
    // Collection error cases
    test_error_case("[1, 2][5]", "list index out of bounds", results);
    test_error_case("{\"a\": 1}[\"b\"]", "dictionary key not found", results);
    test_error_case("[1, 2][\"invalid\"]", "list index wrong type", results);
    test_error_case("{\"a\": 1}[0]", "dictionary key wrong type", results);
}

fn test_compound_assignment(results: &mut TestResult) {
    println!("\n=== COMPOUND ASSIGNMENT ===");
    
    // Basic compound operations
    test_expression("x = 10; x += 5; x", "15", results);
    test_expression("y = 20; y -= 8; y", "12", results);
    test_expression("z = 3; z *= 4; z", "12", results);
    test_expression("w = 15; w /= 3; w", "5", results);
    
    // Compound with expressions
    test_expression("a = 5; a += 2 * 3; a", "11", results);
    test_expression("b = 10; b -= (3 + 2); b", "5", results);
    test_expression("c = 2; c *= (3 + 4); c", "14", results);
    test_expression("d = 20; d /= (2 + 2); d", "5", results);
    
    // Sequential compound operations
    test_expression("x = 10; x += 5; x -= 3; x *= 2; x", "24", results);
    
    // Compound assignment error cases
    test_error_case("undefined_var += 5", "compound assignment to undefined variable", results);
    test_error_case("x = 10; x /= 0", "compound assignment division by zero", results);
}

fn test_logical_operators(results: &mut TestResult) {
    println!("\n=== LOGICAL OPERATORS ===");
    
    // Logical AND
    test_expression("true and true", "true", results);
    test_expression("true and false", "false", results);
    test_expression("false and true", "false", results);
    test_expression("false and false", "false", results);
    
    // Logical OR
    test_expression("true or true", "true", results);
    test_expression("true or false", "true", results);
    test_expression("false or true", "true", results);
    test_expression("false or false", "false", results);
    
    // Logical NOT
    test_expression("not true", "false", results);
    test_expression("not false", "true", results);
    test_expression("not 0", "true", results);
    test_expression("not 5", "false", results);
    
    // Comparison operators
    test_expression("5 < 10", "true", results);
    test_expression("10 > 5", "true", results);
    test_expression("5 <= 5", "true", results);
    test_expression("5 >= 5", "true", results);
    test_expression("3 < 2", "false", results);
    
    // Membership operators
    test_expression("1 in [1, 2, 3]", "true", results);
    test_expression("4 in [1, 2, 3]", "false", results);
    test_expression("1 not in [1, 2, 3]", "false", results);
    test_expression("4 not in [1, 2, 3]", "true", results);
    test_expression("\"name\" in {\"name\": \"John\", \"age\": 30}", "true", results);
    test_expression("\"city\" in {\"name\": \"John\", \"age\": 30}", "false", results);
    
    // Complex logical expressions
    test_expression("true and (false or true)", "true", results);
    test_expression("false or (true and false)", "false", results);
    test_expression("not (true and false)", "true", results);
    test_expression("5 > 3 and 2 < 4", "true", results);
    
    // Short-circuit evaluation
    test_expression("false and 42", "false", results);
    test_expression("true and 42", "42", results);
    test_expression("true or 42", "true", results);
    test_expression("false or 42", "42", results);
    
    // Logical operator error cases
    test_error_case("5 < \"hello\"", "comparison with non-number", results);
    test_error_case("\"hello\" > 10", "comparison with non-number", results);
    test_error_case("5 in 10", "membership with non-collection", results);
    test_error_case("5 in {\"name\": \"John\"}", "non-string key in dictionary", results);
}

fn test_integration(results: &mut TestResult) {
    println!("\n=== INTEGRATION TESTS ===");
    
    // Complex expressions combining multiple features
    test_expression("nums = [1, 2, 3, 4, 5]; max(nums[0], nums[4]) + 10", "15", results);
    test_expression("data = {\"x\": 10, \"y\": 20}; data[\"x\"] * data[\"y\"] / 4", "50", results);
    test_expression("x = 5; y = 10; (x < y) and (x + y > 10)", "true", results);
    
    // Nested data structures
    test_expression("users = [{\"name\": \"Alice\", \"active\": true}, {\"name\": \"Bob\", \"active\": false}]; users[0][\"name\"]", "\"Alice\"", results);
    test_expression("matrix = [[1, 2], [3, 4]]; matrix[1][0]", "3", results);
    
    // Complex logical with collections
    test_expression("items = [1, 2, 3]; target = 2; (target in items) and (max(items[0], items[2]) > target)", "true", results);
    test_expression("config = {\"debug\": true, \"port\": 8080}; config[\"debug\"] and config[\"port\"] > 8000", "true", results);
    
    // Multi-step calculations
    test_expression("base = 10; multiplier = 2; base *= multiplier; result = base + 5; result > 20", "true", results);
    test_expression("scores = [85, 92, 78, 95]; avg_threshold = 85; (scores[0] >= avg_threshold) and (scores[3] >= avg_threshold)", "true", results);
    
    // Function calls with collections
    test_expression("values = [3, 7, 1, 9]; max_val = max(values[0], values[3]); min_val = min(values[1], values[2]); max_val - min_val", "8", results);
    
    // Complex membership testing
    test_expression("whitelist = [\"admin\", \"user\", \"guest\"]; current_role = \"admin\"; current_role in whitelist", "true", results);
    test_expression("permissions = {\"read\": true, \"write\": false, \"delete\": false}; \"write\" in permissions and not permissions[\"write\"]", "true", results);
    
    // Chained operations
    test_expression("x = 1; x += 2; x *= 3; x -= 1; result = x > 7; result", "true", results);
    test_expression("items = []; empty = items; not empty and \"test\" in empty", "false", results);
}

fn test_error_handling(results: &mut TestResult) {
    println!("\n=== ERROR HANDLING ===");
    
    // Lexical errors
    test_error_case("@invalid", "invalid character", results);
    test_error_case("1.2.3", "malformed number", results);
    
    // Parse errors
    test_error_case("x +", "incomplete expression", results);
    test_error_case("5 + + 3", "invalid operator sequence", results);
    test_error_case("[1, 2", "unclosed bracket", results);
    test_error_case("{\"key\":", "incomplete dictionary", results);
    
    // Runtime type errors
    test_error_case("\"text\" + 5", "string + number", results);
    test_error_case("true * false", "boolean arithmetic", results);
    test_error_case("[1, 2] + [3, 4]", "list arithmetic", results);
    
    // Undefined variables and functions
    test_error_case("undefined_var", "undefined variable", results);
    test_error_case("x = undefined_var + 5", "undefined variable in expression", results);
    test_error_case("unknown_func(1, 2)", "undefined function", results);
    
    // Division by zero
    test_error_case("5 / 0", "division by zero", results);
    test_error_case("x = 10; x /= 0", "compound division by zero", results);
    
    // Index and key errors
    test_error_case("[1, 2, 3][10]", "list index out of bounds", results);
    test_error_case("{\"a\": 1}[\"nonexistent\"]", "dictionary key not found", results);
    test_error_case("[1, 2][\"string\"]", "wrong index type for list", results);
    test_error_case("{\"a\": 1}[123]", "wrong key type for dictionary", results);
}

fn main() {
    println!("BCCL Interpreter - Comprehensive Test Suite");
    println!("===========================================\n");
    
    let mut results = TestResult::new();
    
    // Run all test categories
    test_basic_types(&mut results);
    test_functions(&mut results);
    test_collections(&mut results);
    test_compound_assignment(&mut results);
    test_logical_operators(&mut results);
    test_integration(&mut results);
    test_error_handling(&mut results);
    
    // Print final summary
    println!("\n===========================================");
    println!("FINAL RESULTS: {}", results.summary());
    
    if results.failed == 0 {
        println!("🎉 ALL TESTS PASSED! The BCCL interpreter is working correctly.");
    } else {
        println!("⚠️  {} test(s) failed. Review the failures above.", results.failed);
    }
    
    println!("\nTest Categories:");
    println!("  ✅ Basic Data Types (numbers, booleans, strings, variables)");
    println!("  ✅ Custom Functions (max, min with error handling)");
    println!("  ✅ Collections (lists, dictionaries, indexing)");
    println!("  ✅ Compound Assignment (+=, -=, *=, /=)");
    println!("  ✅ Logical Operators (and, or, not, comparisons, membership)");
    println!("  ✅ Integration Tests (complex feature combinations)");
    println!("  ✅ Error Handling (comprehensive error scenarios)");
    
    println!("\nFor detailed testing information, see TESTING.md");
    println!("To run individual test suites, use:");
    println!("  cargo run --bin test_basic_types");
    println!("  cargo run --bin test_functions");
    println!("  cargo run --bin test_lists_dicts");
    println!("  cargo run --bin test_compound_assignment");
    println!("  cargo run --bin test_logical_operators");
}