#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ImageLayout {
    Unknown,
    ColorAttachment,
    DepthStencilAttachment,
    ShaderSample,
    Present,
}
