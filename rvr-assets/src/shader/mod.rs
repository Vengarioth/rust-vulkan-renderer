use serde_derive::*;
use bitflags::*;
use crate::Format;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct ShaderStageFlags: u32 {
        const VERTEX = 0x00000001;
        const TESSELLATION_CONTROL = 0x00000002;
        const TESSELLATION_EVALUATION = 0x00000004;
        const GEOMETRY = 0x00000008;
        const FRAGMENT = 0x00000010;
        const COMPUTE = 0x00000020;
        const RAYGEN_KHR = 0x00000100;
        const ANY_HIT_KHR = 0x00000200;
        const CLOSEST_HIT_KHR = 0x00000400;
        const MISS_KHR = 0x00000800;
        const INTERSECTION_KHR = 0x00001000;
        const CALLABLE_KHR = 0x00002000;
        const TASK_NV = 0x00000040;
        const MESH_NV = 0x00000080;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ShaderStageType {
    Vertex,
    Geometry,
    TessellationControl,
    TessellationEvaluation,
    Fragment,
    Compute,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub enum DescriptorType {
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
    AccelerationStructureNV,
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DescriptorSetLayoutBinding {
    binding: u32,
    descriptor_type: DescriptorType,
    count: u32,
    stage_flags: ShaderStageFlags,
}

impl DescriptorSetLayoutBinding {
    pub fn new(binding: u32, descriptor_type: DescriptorType, count: u32, stage_flags: ShaderStageFlags) -> Self {
        Self {
            binding,
            descriptor_type,
            count,
            stage_flags,
        }
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DescriptorSetLayout {
    pub set: u32,
    pub bindings: Vec<DescriptorSetLayoutBinding>,
}

impl DescriptorSetLayout {
    pub fn new(set: u32, bindings: Vec<DescriptorSetLayoutBinding>) -> Self {
        Self {
            set,
            bindings,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PushConstant {
    offset: u32,
    size: u32,
    stage_flags: ShaderStageFlags,
}

impl PushConstant {
    pub fn new(offset: u32, size: u32, stage_flags: ShaderStageFlags) -> Self {
        Self {
            offset,
            size,
            stage_flags,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct VertexAttribute {
    location: u32,
    stride: u32,
    format: Format,
}

impl VertexAttribute {
    pub fn new(location: u32, stride: u32, format: Format) -> Self {
        Self {
            location,
            stride,
            format,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ShaderStage {
    pub entry_point: String,
    pub spirv: Vec<u32>,
    pub stage_type: ShaderStageType,
}

impl ShaderStage {
    pub fn new(entry_point: String, spirv: Vec<u32>, stage_type: ShaderStageType) -> Self {
        Self {
            entry_point,
            spirv,
            stage_type,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ShaderAsset {
    pub stages: Vec<ShaderStage>,
    pub attributes: Vec<VertexAttribute>,
    pub descriptor_set_layouts: Vec<DescriptorSetLayout>,
    pub push_constants: Vec<PushConstant>,
}

impl ShaderAsset {
    pub fn new(stages: Vec<ShaderStage>, attributes: Vec<VertexAttribute>, descriptor_set_layouts: Vec<DescriptorSetLayout>, push_constants: Vec<PushConstant>) -> Self {
        Self {
            stages,
            attributes,
            descriptor_set_layouts,
            push_constants,
        }
    }
}
