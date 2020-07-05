#[derive(Debug)]
pub struct ShaderStage {
    pub entry_point: String,
    pub spirv: Vec<u32>,
}

impl ShaderStage {
    pub fn new(entry_point: String, spirv: Vec<u32>) -> Self {
        Self {
            entry_point,
            spirv,
        }
    }
}

#[derive(Debug)]
pub struct ShaderAsset {
    pub vertex_shader: ShaderStage,
    pub fragment_shader: ShaderStage,
}

impl ShaderAsset {
    pub fn new(vertex_shader: ShaderStage, fragment_shader: ShaderStage) -> Self {
        Self {
            vertex_shader,
            fragment_shader,
        }
    }
}
