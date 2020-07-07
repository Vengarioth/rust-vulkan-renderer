pub use anyhow::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("PlatformError: {0}")]
    PlatformError(String),
}
