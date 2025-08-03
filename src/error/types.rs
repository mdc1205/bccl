use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Comprehensive error type for the BCCL interpreter
#[derive(Error, Diagnostic, Debug)]
pub enum BcclError {
    #[error("Lexical error: {message}")]
    #[diagnostic(
        code(bccl::lexer::invalid_character),
        help("Check for unsupported characters or malformed tokens")
    )]
    LexError {
        message: String,
        #[label("invalid character here")]
        span: SourceSpan,
    },

    #[error("Lexical error: {message}")]
    #[diagnostic(
        code(bccl::lexer::malformed_number),
        help("Numbers should be in the format: 123 or 123.456")
    )]
    MalformedNumber {
        message: String,
        #[label("malformed number")]
        span: SourceSpan,
    },

    #[error("Syntax error: {message}")]
    #[diagnostic(
        code(bccl::parser::unexpected_token),
        help("Check the syntax near this location")
    )]
    UnexpectedToken {
        message: String,
        #[label("unexpected token")]
        span: SourceSpan,
        expected: Vec<String>,
    },

    #[error("Syntax error: unexpected end of input")]
    #[diagnostic(code(bccl::parser::unexpected_eof))]
    UnexpectedEof {
        #[label("expected more input here")]
        span: SourceSpan,
        expected: Vec<String>,
        #[help]
        help_message: String,
    },

    #[error("Syntax error: {message}")]
    #[diagnostic(
        code(bccl::parser::missing_token),
        help("Add the missing token to complete the expression")
    )]
    MissingToken {
        message: String,
        #[label("missing token should be here")]
        span: SourceSpan,
        expected: String,
    },

    #[error("Runtime error: undefined variable '{name}'")]
    #[diagnostic(code(bccl::runtime::undefined_variable))]
    UndefinedVariable {
        name: String,
        #[label("undefined variable")]
        span: SourceSpan,
        #[help]
        suggestion: Option<String>,
    },

    #[error("Runtime error: division by zero")]
    #[diagnostic(
        code(bccl::runtime::division_by_zero),
        help("Cannot divide by zero. Check the divisor value.")
    )]
    DivisionByZero {
        #[label("division by zero occurs here")]
        span: SourceSpan,
        #[label("divisor evaluates to zero")]
        divisor_span: SourceSpan,
    },

    #[error("Type error: {message}")]
    #[diagnostic(
        code(bccl::runtime::type_error),
        help("Check that all operands have compatible types")
    )]
    TypeError {
        message: String,
        #[label("type error")]
        span: SourceSpan,
        expected_type: String,
        actual_type: String,
    },

    #[error("Runtime error: {message}")]
    #[diagnostic(
        code(bccl::runtime::evaluation_error)
    )]
    EvaluationError {
        message: String,
        #[label("error occurs here")]
        span: SourceSpan,
        #[help]
        suggestion: Option<String>,
    },

    #[error("Assignment error: {message}")]
    #[diagnostic(
        code(bccl::runtime::assignment_error),
        help("Check the assignment syntax and variable name")
    )]
    AssignmentError {
        message: String,
        #[label("assignment error")]
        span: SourceSpan,
        variable_name: String,
    },

    #[error("Runtime error: undefined function '{name}'")]
    #[diagnostic(code(bccl::runtime::undefined_function))]
    UndefinedFunction {
        name: String,
        #[label("undefined function")]
        span: SourceSpan,
        #[help]
        suggestion: Option<String>,
    },

    #[error("Function error: {function_name} expects {expected} arguments, got {actual}")]
    #[diagnostic(
        code(bccl::runtime::wrong_argument_count),
        help("Check the function call arguments")
    )]
    WrongArgumentCount {
        function_name: String,
        expected: usize,
        actual: usize,
        #[label("function call")]
        span: SourceSpan,
    },

    #[error("Function error: {function_name} argument {arg_number} must be {expected_type}, got {actual_type}")]
    #[diagnostic(
        code(bccl::runtime::function_argument_type_error),
        help("Check the argument types")
    )]
    FunctionArgumentTypeError {
        function_name: String,
        arg_number: usize,
        expected_type: String,
        actual_type: String,
        #[label("wrong argument type")]
        span: SourceSpan,
    },

    #[error("Index error: {collection_type} index {index} is out of bounds (length: {length})")]
    #[diagnostic(
        code(bccl::runtime::index_out_of_bounds),
        help("Check that the index is within the valid range")
    )]
    IndexOutOfBounds {
        collection_type: String,
        index: usize,
        length: usize,
        #[label("index out of bounds")]
        span: SourceSpan,
    },

    #[error("Key error: key '{key}' not found")]
    #[diagnostic(
        code(bccl::runtime::key_not_found)
    )]
    KeyNotFound {
        key: String,
        #[label("key not found")]
        span: SourceSpan,
        available_keys: Vec<String>,
        #[help]
        suggestion: String,
    },

    #[error("Compound assignment error: {message}")]
    #[diagnostic(
        code(bccl::runtime::compound_assignment_error)
    )]
    CompoundAssignmentError {
        message: String,
        variable_name: String,
        operator: String,
        #[label("{operator} compound assignment")]
        span: SourceSpan,
        #[help]
        suggestion: String,
    },

    #[error("Logical operation error: {message}")]
    #[diagnostic(
        code(bccl::runtime::logical_operation_error)
    )]
    LogicalOperationError {
        message: String,
        operator: String,
        #[label("logical {operator} operation")]
        span: SourceSpan,
        #[help]
        suggestion: String,
    },

    #[error("Collection operation error: {message}")]
    #[diagnostic(
        code(bccl::runtime::collection_operation_error)
    )]
    CollectionOperationError {
        message: String,
        operation: String,
        #[label("{operation} operation")]
        span: SourceSpan,
        #[help]
        suggestion: String,
    },
}

pub type BcclResult<T> = Result<T, BcclError>;