use miette::SourceSpan;

/// Source span information for tracking locations in the input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    
    pub fn single(pos: usize) -> Self {
        Self { start: pos, end: pos + 1 }
    }
    
    pub fn zero_width(pos: usize) -> Self {
        Self { start: pos, end: pos }
    }
    
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    
    pub fn combine(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl From<Span> for SourceSpan {
    fn from(span: Span) -> Self {
        SourceSpan::new(span.start.into(), span.len())
    }
}