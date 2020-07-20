use crate::Error;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use tinypath::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "asset_type")]
pub enum Asset {
    Shader(ShaderAsset),
}

impl Asset {
    pub fn from_contents(contents: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice::<Self>(contents)?)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ShaderStageType {
    Vertex,
    Fragment,
}

impl Into<shaderc::ShaderKind> for ShaderStageType {
    fn into(self) -> shaderc::ShaderKind {
        match self {
            ShaderStageType::Vertex => shaderc::ShaderKind::Vertex,
            ShaderStageType::Fragment => shaderc::ShaderKind::Fragment,
        }
    }
}

impl Into<rvr_assets::shader::ShaderStageFlags> for ShaderStageType {
    fn into(self) -> rvr_assets::shader::ShaderStageFlags {
        match self {
            ShaderStageType::Vertex => rvr_assets::shader::ShaderStageFlags::VERTEX,
            ShaderStageType::Fragment => rvr_assets::shader::ShaderStageFlags::FRAGMENT,
        }
    }
}

impl Into<rvr_assets::shader::ShaderStageType> for ShaderStageType {
    fn into(self) -> rvr_assets::shader::ShaderStageType {
        match self {
            ShaderStageType::Vertex => rvr_assets::shader::ShaderStageType::Vertex,
            ShaderStageType::Fragment => rvr_assets::shader::ShaderStageType::Fragment,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderStage {
    pub stage_type: ShaderStageType,
    pub entry_point: String,
    pub source: Path,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShaderAsset {
    pub stages: Vec<ShaderStage>,
}

impl ShaderAsset {
    pub fn get_dependencies(&self, file_path: &Path) -> Result<Vec<Path>, Error> {
        Ok(self
            .stages
            .iter()
            .map(|stage| stage.source.relative_to(file_path))
            .collect())
    }
}
