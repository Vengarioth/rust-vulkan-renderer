use crate::Error;
use serde_derive::{Serialize, Deserialize};
use serde_json;
use super::AssetType;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    pub asset_type: AssetType,
}

impl AssetInfo {
    pub fn from_contents(contents: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice::<Self>(contents)?)
    }
}
