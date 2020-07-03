use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Could not initialize shaderc")]
    Initialization,

    #[error("Could not include a file")]
    Include,

    #[error("Could not compile")]
    Compile,
}
