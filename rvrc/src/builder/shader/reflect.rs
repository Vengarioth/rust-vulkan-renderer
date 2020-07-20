use crate::{
    Error,
    builder::shader::*,
};
use rvr_assets::{
    Format,
    shader::{DescriptorType, ShaderStageFlags, VertexAttribute},
};
use spirv_reflect::{
    ShaderModule,
    types::{
        ReflectDescriptorType,
        ReflectFormat,
    },
};

fn reflect_descriptor_type_to_descriptor_type(reflect_descriptor_type: &ReflectDescriptorType) -> Result<DescriptorType, Error> {
    match reflect_descriptor_type {
        ReflectDescriptorType::Sampler => Ok(DescriptorType::Sampler),
        ReflectDescriptorType::CombinedImageSampler => Ok(DescriptorType::CombinedImageSampler),
        ReflectDescriptorType::SampledImage => Ok(DescriptorType::SampledImage),
        ReflectDescriptorType::StorageImage => Ok(DescriptorType::StorageImage),
        ReflectDescriptorType::UniformTexelBuffer => Ok(DescriptorType::UniformTexelBuffer),
        ReflectDescriptorType::StorageTexelBuffer => Ok(DescriptorType::StorageTexelBuffer),
        ReflectDescriptorType::UniformBuffer => Ok(DescriptorType::UniformBuffer),
        ReflectDescriptorType::StorageBuffer => Ok(DescriptorType::StorageBuffer),
        ReflectDescriptorType::UniformBufferDynamic => Ok(DescriptorType::UniformBufferDynamic),
        ReflectDescriptorType::StorageBufferDynamic => Ok(DescriptorType::StorageBufferDynamic),
        ReflectDescriptorType::InputAttachment => Ok(DescriptorType::InputAttachment),
        ReflectDescriptorType::AccelerationStructureNV => Ok(DescriptorType::AccelerationStructureNV),
        ReflectDescriptorType::Undefined => Err(ReflectError::UnknownDescriptorType.into()),
    }
}

fn reflect_format_to_asseter_format(reflect_format: ReflectFormat) -> Result<Format, Error> {
    match reflect_format {
        ReflectFormat::R32_UINT => Ok(Format::R32_UINT),
        ReflectFormat::R32_SINT => Ok(Format::R32_SINT),
        ReflectFormat::R32_SFLOAT => Ok(Format::R32_SFLOAT),
        ReflectFormat::R32G32_UINT => Ok(Format::R32G32_UINT),
        ReflectFormat::R32G32_SINT => Ok(Format::R32G32_SINT),
        ReflectFormat::R32G32_SFLOAT => Ok(Format::R32G32_SFLOAT),
        ReflectFormat::R32G32B32_UINT => Ok(Format::R32G32B32_UINT),
        ReflectFormat::R32G32B32_SINT => Ok(Format::R32G32B32_SINT),
        ReflectFormat::R32G32B32_SFLOAT => Ok(Format::R32G32B32_SFLOAT),
        ReflectFormat::R32G32B32A32_UINT => Ok(Format::R32G32B32A32_UINT),
        ReflectFormat::R32G32B32A32_SINT => Ok(Format::R32G32B32A32_SINT),
        ReflectFormat::R32G32B32A32_SFLOAT => Ok(Format::R32G32B32A32_SFLOAT),
        ReflectFormat::Undefined => Err(ReflectError::UnknownFormat.into()),
    }
}

fn reflect_format_to_stride(reflect_format: ReflectFormat) -> Result<u32, Error> {
    match reflect_format {
        ReflectFormat::R32_UINT => Ok(4),
        ReflectFormat::R32_SINT => Ok(4),
        ReflectFormat::R32_SFLOAT => Ok(4),
        ReflectFormat::R32G32_UINT => Ok(8),
        ReflectFormat::R32G32_SINT => Ok(8),
        ReflectFormat::R32G32_SFLOAT => Ok(8),
        ReflectFormat::R32G32B32_UINT => Ok(12),
        ReflectFormat::R32G32B32_SINT => Ok(12),
        ReflectFormat::R32G32B32_SFLOAT => Ok(12),
        ReflectFormat::R32G32B32A32_UINT => Ok(16),
        ReflectFormat::R32G32B32A32_SINT => Ok(16),
        ReflectFormat::R32G32B32A32_SFLOAT => Ok(16),
        ReflectFormat::Undefined => Err(ReflectError::UnknownFormat.into()),
    }
}

pub fn reflect(spirv: &[u32], entry_point: &str, descriptor_sets: &mut DescriptorSets, stage_flags: ShaderStageFlags) -> Result<Vec<VertexAttribute>, Error> {
    let module = ShaderModule::load_u32_data(spirv)
        .map_err(|error| ReflectError::Initialization(error.to_string()))?;

    let reflected_descriptor_sets = module.enumerate_descriptor_sets(Some(entry_point))
        .map_err(|error| ReflectError::EnumerateDescriptorSets(error.to_string()))?;
    
    let reflected_input_variables = module.enumerate_input_variables(Some(entry_point))
        .map_err(|error| ReflectError::EnumerateInputVariables(error.to_string()))?;
    // let reflected_output_variables = module.enumerate_output_variables(Some(entry_point))
    //     .map_err(|error| ReflectError::EnumerateOutputVariables(error.to_string()))?;
    // dbg!(module.enumerate_push_constant_blocks(Some(entry_point)).map_err(|error| ReflectError::EnumeratePushConstants(error.to_string()))?);

    for reflected_descriptor_set in reflected_descriptor_sets {
        let mut descriptor_set = DescriptorSet::new(reflected_descriptor_set.set);

        for reflected_binding in &reflected_descriptor_set.bindings {
            let binding = DescriptorBinding::new(
                reflected_binding.binding,
                reflect_descriptor_type_to_descriptor_type(&reflected_binding.descriptor_type)?,
                reflected_binding.count,
                stage_flags,
            );

            descriptor_set.insert_binding(binding)?;
        }

        descriptor_sets.insert_set(descriptor_set)?;
    }

    let mut input = Vec::new();
    for variable in reflected_input_variables.iter() {
        let builtin = if let Some(type_description) = &variable.type_description {
            BuiltinAttribute::from_str(&type_description.type_name)
        } else {
            None
        };

        input.push(Attribute::new(
            variable.location,
            reflect_format_to_asseter_format(variable.format)?,
            reflect_format_to_stride(variable.format)?,
            builtin,
        ));
    }
    let input_signature = StageSignature::new(input);
    let attributes = input_signature.extract_vertex_attributes();

    Ok(attributes)
}
