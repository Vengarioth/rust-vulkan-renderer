use crate::{
    Error,
    builder::BuildContext,
    assets::ShaderAsset,
};

mod compile;
mod compile_error;
mod reflect;
mod reflect_error;

use compile::*;
pub use compile_error::*;

use reflect::*;
pub use reflect_error::*;

pub fn build_shader_asset(asset: &ShaderAsset, context: &mut BuildContext) -> Result<(), Error> {

    let vertex_shader_source = context.load(&asset.vertex_shader)?;
    let fragment_shader_source = context.load(&asset.fragment_shader)?;

    let vertex_shader_source = std::str::from_utf8(&vertex_shader_source)?;
    let fragment_shader_source = std::str::from_utf8(&fragment_shader_source)?;

    let vertex_shader_binary = compile(
        &asset.vertex_shader_entry_point,
        vertex_shader_source,
        &context.get_full_path(&asset.vertex_shader).to_platform_string(),
        shaderc::ShaderKind::Vertex,
    )?;

    let fragment_shader_binary = compile(
        &asset.fragment_shader_entry_point,
        fragment_shader_source,
        &context.get_full_path(&asset.fragment_shader).to_platform_string(),
        shaderc::ShaderKind::Fragment,
    )?;

    reflect(&vertex_shader_binary, &asset.vertex_shader_entry_point)?;
    reflect(&fragment_shader_binary, &asset.fragment_shader_entry_point)?;

    dbg!(vertex_shader_source);
    dbg!(fragment_shader_source);

    dbg!(vertex_shader_binary);
    dbg!(fragment_shader_binary);

    Ok(())
}
