use crate::error::Span;

#[derive(Debug, Clone)]
pub enum Expr {
    Number { value: f64, span: Span },
    Integer { value: i64, span: Span },
    Boolean { value: bool, span: Span },
    String { value: String, span: Span },
    Identifier { name: String, span: Span },
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
        span: Span,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
        kwargs: Vec<(String, Expr)>,
        span: Span,
    },
    List {
        elements: Vec<Expr>,
        span: Span,
    },
    Dictionary {
        pairs: Vec<(String, Expr)>,
        span: Span,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Number { span, .. } => *span,
            Expr::Integer { span, .. } => *span,
            Expr::Boolean { span, .. } => *span,
            Expr::String { span, .. } => *span,
            Expr::Identifier { span, .. } => *span,
            Expr::Binary { span, .. } => *span,
            Expr::Unary { span, .. } => *span,
            Expr::FunctionCall { span, .. } => *span,
            Expr::List { span, .. } => *span,
            Expr::Dictionary { span, .. } => *span,
            Expr::Index { span, .. } => *span,
        }
    }
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Expression { span, .. } => *span,
            Stmt::Assignment { span, .. } => *span,
            Stmt::CompoundAssignment { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    
    // Basic comparison for now
    Equal,
    NotEqual,
    
    // Comparison
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    
    // Logical
    And,
    Or,
    In,
    NotIn,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
    Plus,
    Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expr: Expr, span: Span },
    Assignment { name: String, value: Expr, span: Span },
    CompoundAssignment { 
        name: String, 
        operator: CompoundOp, 
        value: Expr, 
        span: Span 
    },
}

#[derive(Debug, Clone)]
pub enum CompoundOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}