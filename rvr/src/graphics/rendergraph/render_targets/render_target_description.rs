use crate::graphics::rendergraph::*;

#[derive(Debug)]
pub struct RenderTargetDescription {
    pub(crate) color_attachments: Vec<ImageHandle>,
    pub(crate) depth_stencil_attachment: Option<ImageHandle>,
}

impl RenderTargetDescription {
    pub fn new(color_attachments: Vec<ImageHandle>, depth_stencil_attachment: Option<ImageHandle>) -> Self {
        Self {
            color_attachments,
            depth_stencil_attachment,
        }
    }
}
