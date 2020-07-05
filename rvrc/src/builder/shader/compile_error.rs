use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Could not initialize shaderc")]
    Initialization,
}
