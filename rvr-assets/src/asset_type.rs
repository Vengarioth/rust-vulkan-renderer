use serde_derive::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub enum AssetType {
    Shader,
}
