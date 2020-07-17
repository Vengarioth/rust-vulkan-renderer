use crate::{
    Error,
    graphics::rendergraph::*,
    util::IdGenerator,
};

pub struct PassBuilder<'a> {
    id: u32,
    name: String,
    sample_images: Vec<ImageHandle>,
    color_attachments: Vec<ImageHandle>,
    depth_stencil_attachment: Option<ImageHandle>,
    id_generator: &'a mut IdGenerator,
}

impl<'a> PassBuilder<'a> {
    pub fn new(id: u32, name: String, id_generator: &'a mut IdGenerator) -> Self {
        Self {
            id,
            name,
            sample_images: Vec::new(),
            color_attachments: Vec::new(),
            depth_stencil_attachment: None,
            id_generator,
        }
    }

    pub fn sample_image(&mut self, image: ImageHandle) {
        self.sample_images.push(image);
    }

    pub fn color_attachment(&mut self, image: ImageHandle) -> Result<ImageHandle, Error> {
        let next_image = ImageHandle::new(image.id, image.version + 1);

        self.color_attachments.push(next_image.clone());

        Ok(next_image)
    }

    pub fn depth_stencil_attachment(&mut self, image: ImageHandle) -> Result<ImageHandle, Error> {
        if self.depth_stencil_attachment.is_some() {
            // TODO make proper error
            panic!("{}", "TODO depth stencil attachment already defined");
        }

        let next_image = ImageHandle::new(image.id, image.version + 1);

        self.depth_stencil_attachment = Some(next_image.clone());

        Ok(next_image)
    }

    pub(crate) fn build(self, executor: Box<dyn Executor>) -> Pass {

        let render_target_description = RenderTargetDescription::new(
            self.color_attachments,
            self.depth_stencil_attachment,
        );

        Pass::new(
            self.id,
            self.name,
            render_target_description,
            self.sample_images,
            executor,
        )
    }
}
