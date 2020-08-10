use crate::AssetType;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetLocation {
    pub name: String,
    pub asset_type: AssetType,
    pub file_index: usize,
    pub offset: usize,
    pub length: usize,
}

impl AssetLocation {
    pub fn new(name: String, asset_type: AssetType, file_index: usize, offset: usize, length: usize) -> Self {
        Self {
            name,
            asset_type,
            file_index,
            offset,
            length,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetIndex {
    pub files: Vec<String>,
    pub locations: Vec<AssetLocation>,
}

impl AssetIndex {
    pub fn new(files: Vec<String>, locations: Vec<AssetLocation>) -> Self {
        Self { files, locations }
    }
}
