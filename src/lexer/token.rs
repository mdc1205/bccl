use crate::error::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
    Identifier(String),
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    
    // Comparison
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    
    // Logical
    And,
    Or,
    Not,
    In,
    
    // Assignment
    Assign,
    PlusAssign,   // +=
    MinusAssign,  // -=
    MultiplyAssign, // *=
    DivideAssign, // /=
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,
    Colon,
    Semicolon,
    
    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }
}