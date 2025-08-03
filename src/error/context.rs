/// Context for error reporting that tracks the original source
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub source: String,
    pub filename: Option<String>,
}

impl ErrorContext {
    pub fn new(source: String) -> Self {
        Self {
            source,
            filename: None,
        }
    }

    pub fn with_filename(mut self, filename: String) -> Self {
        self.filename = Some(filename);
        self
    }
}