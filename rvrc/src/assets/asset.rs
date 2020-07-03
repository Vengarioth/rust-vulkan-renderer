
use serde_derive::{Serialize, Deserialize};
use serde_json;
use crate::Error;
use tinypath::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "asset_type")]
pub enum Asset {
    Shader(ShaderAsset)
}

impl Asset {
    pub fn from_contents(contents: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice::<Self>(contents)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderAsset {
    pub vertex_shader: String,
    pub vertex_shader_entry_point: String,
    pub fragment_shader: String,
    pub fragment_shader_entry_point: String,
}

impl ShaderAsset {
    pub fn get_dependencies(&self, file_path: &Path) -> Result<Vec<Path>, Error> {
        Ok(vec![
            Path::from_str(&self.vertex_shader).unwrap().relative_to(file_path),
            Path::from_str(&self.fragment_shader).unwrap().relative_to(file_path),
        ])
    }
}
