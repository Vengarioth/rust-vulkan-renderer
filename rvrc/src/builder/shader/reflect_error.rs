use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReflectError {
    #[error("Could not initialize spirv-reflect: {0}")]
    Initialization(String),

    #[error("Could not enumerate descriptor sets: {0}")]
    EnumerateDescriptorSets(String),

    #[error("Could not enumerate input variables: {0}")]
    EnumerateInputVariables(String),

    #[error("Could not enumerate output variables: {0}")]
    EnumerateOutputVariables(String),

    #[error("Could not enumerate push constants: {0}")]
    EnumeratePushConstants(String),
}
