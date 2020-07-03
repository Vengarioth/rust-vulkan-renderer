use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum AssetType {
    Shader,
}
