use crate::graphics::render_graph::*;

#[derive(Debug, Clone)]
pub struct RenderTargetHandle {
    color_attachments: Vec<ImageHandle>,
    depth_stencil_attachment: Option<ImageHandle>,
}

impl RenderTargetHandle {
    pub fn new(color_attachments: Vec<ImageHandle>, depth_stencil_attachment: Option<ImageHandle>) -> Self {
        Self {
            color_attachments,
            depth_stencil_attachment,
        }
    }

    pub fn get_color_attachment(&self, index: usize) -> ImageHandle {
        self.color_attachments[index].clone()
    }

    pub fn get_depth_stencil_attachment(&self) -> ImageHandle {
        self.depth_stencil_attachment.clone().unwrap()
    }
}
