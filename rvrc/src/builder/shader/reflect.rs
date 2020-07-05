use crate::{
    Error,
    builder::shader::ReflectError,
};
use spirv_reflect::{
    ShaderModule,
};

pub fn reflect(spirv: &[u32], entry_point: &str) -> Result<(), Error> {
    let module = ShaderModule::load_u32_data(spirv)
        .map_err(|error| ReflectError::Initialization(error.to_string()))?;

    dbg!(module.enumerate_descriptor_sets(Some(entry_point)).map_err(|error| ReflectError::EnumerateDescriptorSets(error.to_string()))?);
    dbg!(module.enumerate_input_variables(Some(entry_point)).map_err(|error| ReflectError::EnumerateInputVariables(error.to_string()))?);
    dbg!(module.enumerate_output_variables(Some(entry_point)).map_err(|error| ReflectError::EnumerateOutputVariables(error.to_string()))?);
    dbg!(module.enumerate_push_constant_blocks(Some(entry_point)).map_err(|error| ReflectError::EnumeratePushConstants(error.to_string()))?);

    Ok(())
}
