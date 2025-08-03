use super::span::Span;
use super::types::BcclError;

impl BcclError {
    /// Create a lexer error for invalid characters
    pub fn invalid_character(ch: char, span: Span) -> Self {
        Self::LexError {
            message: format!("Invalid character '{}'", ch),
            span: span.into(),
        }
    }

    /// Create a lexer error for malformed numbers
    pub fn malformed_number(text: &str, span: Span) -> Self {
        Self::MalformedNumber {
            message: format!("Malformed number '{}'", text),
            span: span.into(),
        }
    }
}