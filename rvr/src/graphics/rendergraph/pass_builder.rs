use crate::{
    Error,
    graphics::rendergraph::*,
    util::IdGenerator,
};

pub struct PassBuilder<'a> {
    id: u32,
    name: String,
    sample_images: Vec<ImageHandle>,
    id_generator: &'a mut IdGenerator,
}

impl<'a> PassBuilder<'a> {
    pub fn new(id: u32, name: String, id_generator: &'a mut IdGenerator) -> Self {
        Self {
            id,
            name,
            sample_images: Vec::new(),
            id_generator,
        }
    }

    pub fn color_attachment(&mut self, image: ImageHandle) -> ImageHandle {
        ImageHandle::new(image.id, image.version + 1)
    }

    pub fn depth_stencil_attachment(&mut self, image: ImageHandle) -> ImageHandle {
        ImageHandle::new(image.id, image.version + 1)
    }

    pub(crate) fn build(self, description: RenderTargetDescription) -> Pass {

        Pass::new()
    }
}
