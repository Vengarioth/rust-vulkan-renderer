use crate::{
    Error,
    Context,
    assets::Shader,
};

pub fn load_shader(context: &mut Context, name: &str) -> Result<Shader, Error> {
    let shader = context.assets.load_shader(name)?;
    Ok(shader)
}

pub fn is_shader_resident(context: &Context, shader: Shader) -> Result<bool, Error> {
    context.assets.is_shader_resident(shader)
}
