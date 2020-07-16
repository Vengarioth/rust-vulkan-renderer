use super::*;

pub struct Pass {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) render_target: RenderTargetDescription,
    pub(crate) sample_images: Vec<ImageHandle>,
    pub(crate) executor: Box<dyn Executor>,
}

impl Pass {
    pub fn new(id: u32, name: String, render_target: RenderTargetDescription, sample_images: Vec<ImageHandle>, executor: Box<dyn Executor>) -> Self {
        Self {
            id,
            name,
            render_target,
            sample_images,
            executor,
        }
    }
}
