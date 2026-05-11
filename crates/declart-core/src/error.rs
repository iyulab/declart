use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeclartError {
    #[error("parse error: {0}")]
    Parse(String),
}
