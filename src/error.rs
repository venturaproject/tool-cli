use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolerError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
