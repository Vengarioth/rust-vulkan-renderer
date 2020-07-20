use crate::{
    Error,
    builder::BuildContext,
    assets::ShaderAsset,
};

mod attributes;
mod compile;
mod compile_error;
mod descriptors;
mod reflect;
mod reflect_error;

use attributes::*;
use compile::*;
use descriptors::*;
pub use compile_error::*;

use reflect::*;
pub use reflect_error::*;

pub fn build_shader_asset(asset: &ShaderAsset, context: &mut BuildContext) -> Result<(), Error> {

    let mut descriptor_sets = DescriptorSets::new();
    let mut vertex_attributes = None;

    let mut stages = Vec::new();
    for stage in &asset.stages {
        let source = context.load(&stage.source)?;
        let source = std::str::from_utf8(&source)?;

        let binary = compile(
            &stage.entry_point,
            source,
            &context.get_full_path(&stage.source).to_platform_string(),
            stage.stage_type.into(),
        )?;

        let attributes = reflect(&binary, &stage.entry_point, &mut descriptor_sets, stage.stage_type.into())?;

        let shader_stage = rvr_assets::shader::ShaderStage::new(
            stage.entry_point.clone(),
            binary,
            stage.stage_type.into(),
        );

        if stage.stage_type == crate::assets::ShaderStageType::Vertex {
            vertex_attributes = Some(attributes);
        }
    }

    let descriptor_set_layouts = descriptor_sets.build_layouts();

    let shader_asset = rvr_assets::shader::ShaderAsset::new(
        stages,
        vertex_attributes.unwrap(), // TODO
        descriptor_set_layouts,
        Vec::new(),
    );

    Ok(())
}
