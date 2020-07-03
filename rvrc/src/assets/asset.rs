
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
    pub vertex_shader: Path,
    pub vertex_shader_entry_point: String,
    pub fragment_shader: Path,
    pub fragment_shader_entry_point: String,
}

impl ShaderAsset {
    pub fn get_dependencies(&self, file_path: &Path) -> Result<Vec<Path>, Error> {
        Ok(vec![
            self.vertex_shader.relative_to(file_path),
            self.fragment_shader.relative_to(file_path),
        ])
    }
}
