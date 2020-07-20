use rvr_assets::{
    Format,
    shader::VertexAttribute,
};

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum BuiltinAttribute {
    gl_VertexID,
    gl_InstanceID,
    gl_DrawID,
    gl_BaseVertex,
    gl_BaseInstance,
    gl_Position,
    gl_PointSize,
    gl_ClipDistance,
    gl_PatchVerticesIn,
    gl_PrimitiveID,
    gl_InvocationID,
    gl_TessCoord,
    gl_PrimitiveIDIn,
    gl_FragCoord,
    gl_FrontFacing,
    gl_PointCoord,
    gl_SampleID,
    gl_SamplePosition,
    gl_SampleMaskIn,
    gl_Layer,
    gl_ViewportIndex,
    gl_FragDepth,
    gl_SampleMask,
    gl_PerVertex,
}

impl BuiltinAttribute {
    pub fn from_str(value: &str) -> Option<BuiltinAttribute> {
        match value {
            "gl_VertexID" => Some(Self::gl_VertexID),
            "gl_InstanceID" => Some(Self::gl_InstanceID),
            "gl_DrawID" => Some(Self::gl_DrawID),
            "gl_BaseVertex" => Some(Self::gl_BaseVertex),
            "gl_BaseInstance" => Some(Self::gl_BaseInstance),
            "gl_Position" => Some(Self::gl_Position),
            "gl_PointSize" => Some(Self::gl_PointSize),
            "gl_ClipDistance" => Some(Self::gl_ClipDistance),
            "gl_PatchVerticesIn" => Some(Self::gl_PatchVerticesIn),
            "gl_PrimitiveID" => Some(Self::gl_PrimitiveID),
            "gl_InvocationID" => Some(Self::gl_InvocationID),
            "gl_TessCoord" => Some(Self::gl_TessCoord),
            "gl_PrimitiveIDIn" => Some(Self::gl_PrimitiveIDIn),
            "gl_FragCoord" => Some(Self::gl_FragCoord),
            "gl_FrontFacing" => Some(Self::gl_FrontFacing),
            "gl_PointCoord" => Some(Self::gl_PointCoord),
            "gl_SampleID" => Some(Self::gl_SampleID),
            "gl_SamplePosition" => Some(Self::gl_SamplePosition),
            "gl_SampleMaskIn" => Some(Self::gl_SampleMaskIn),
            "gl_Layer" => Some(Self::gl_Layer),
            "gl_ViewportIndex" => Some(Self::gl_ViewportIndex),
            "gl_FragDepth" => Some(Self::gl_FragDepth),
            "gl_SampleMask" => Some(Self::gl_SampleMask),
            "gl_PerVertex" => Some(Self::gl_PerVertex),
            _ => None,
        }
    } 
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Attribute {
    pub location: u32,
    pub format: Format,
    pub stride: u32,
    pub builtin: Option<BuiltinAttribute>,
}

impl Attribute {
    pub fn new(
        location: u32,
        format: Format,
        stride: u32,
        builtin: Option<BuiltinAttribute>,
    ) -> Self {
        Self {
            location,
            format,
            stride,
            builtin,
        }
    }
}

#[derive(Debug)]
pub struct StageSignature {
    attributes: Vec<Attribute>,
}

impl StageSignature {
    pub fn new(attributes: Vec<Attribute>) -> Self {
        Self {
            attributes,
        }
    }

    pub fn extract_vertex_attributes(&self) -> Vec<VertexAttribute> {
        let mut attributes = Vec::new();
        for attribute in &self.attributes {
            if attribute.builtin.is_some() {
                continue;
            }

            attributes.push(VertexAttribute::new(attribute.location, attribute.stride, attribute.format));
        }

        attributes
    }

    pub fn is_compatible_with(&self, target_stage: &Self) -> bool {
        use std::collections::HashMap;

        let mut target = HashMap::new();

        for attribute in &target_stage.attributes {
            if attribute.builtin.is_some() {
                continue;
            }
            
            target.insert(attribute.location, attribute);
        }

        for attribute in &self.attributes {
            if attribute.builtin.is_some() {
                continue;
            }

            if let Some(other) = target.get(&attribute.location) {
                if attribute != *other {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
