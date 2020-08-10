use thiserror::*;
use rvr_assets::AssetType;

#[derive(Debug, Error)]
pub enum AssetError {

    #[error("Shader \"{0}\" not found")]
    ShaderNotFound(String),

    #[error("Asset has wrong type, expected {0}, found {1}")]
    AssetHasWrongType(AssetType, AssetType),
    
}
