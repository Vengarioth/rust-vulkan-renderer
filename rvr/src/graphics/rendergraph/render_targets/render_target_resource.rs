use crate::graphics::render_graph::*;

#[derive(Debug)]
pub struct RenderTargetResource {
    pub(crate) id: u32,
    pub(crate) description: RenderTargetDescription,
    pub(crate) input_images: Vec<ImageHandle>,
    pub(crate) output_images: Vec<ImageHandle>,
}

impl RenderTargetResource {
    pub fn new(id: u32, description: RenderTargetDescription, input_images: Vec<ImageHandle>, output_images: Vec<ImageHandle>) -> Self {
        Self {
            id,
            description,
            input_images,
            output_images,
        }
    }

    pub fn print_graphviz(&self, visualizer: &mut Visualizer) {
        for input_image in &self.input_images {
            visualizer.println(&format!("Image_{}_{} -> RenderTarget_{} [label=\"write\"];", input_image.id, input_image.version, self.id));
        }

        for output_image in &self.output_images {
            visualizer.println(&format!("RenderTarget_{} -> Image_{}_{};", self.id, output_image.id, output_image.version));
        }
    }
}
