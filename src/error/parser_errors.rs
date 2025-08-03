use super::span::Span;
use super::types::BcclError;

impl BcclError {
    /// Create a parser error for unexpected tokens
    pub fn unexpected_token(found: &str, span: Span, expected: Vec<String>) -> Self {
        Self::UnexpectedToken {
            message: format!("Expected one of [{}], but found '{}'", expected.join(", "), found),
            span: span.into(),
            expected,
        }
    }

    /// Create a parser error for unexpected EOF
    pub fn unexpected_eof(span: Span, expected: Vec<String>) -> Self {
        let help_message = if expected.len() == 1 {
            format!("Expected: {}", expected[0])
        } else {
            format!("Expected one of: {}", expected.join(", "))
        };
        
        Self::UnexpectedEof {
            span: span.into(),
            expected,
            help_message,
        }
    }

    /// Create a parser error for missing tokens
    pub fn missing_token(expected: &str, span: Span) -> Self {
        Self::MissingToken {
            message: format!("Expected '{}'", expected),
            span: span.into(),
            expected: expected.to_string(),
        }
    }
}