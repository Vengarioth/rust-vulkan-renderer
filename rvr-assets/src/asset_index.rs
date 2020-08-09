use crate::AssetType;
use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetLocation {
    name: String,
    asset_type: AssetType,
    file_index: usize,
    offset: usize,
    length: usize,
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
    files: Vec<String>,
    locations: Vec<AssetLocation>,
}

impl AssetIndex {
    pub fn new(files: Vec<String>, locations: Vec<AssetLocation>) -> Self {
        Self { files, locations }
    }
}
