use crate::graphics::*;

#[derive(Debug)]
pub struct ImageDescription {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub image_type: ImageType,
    pub sample_count: SampleCount,
    pub initial_layout: ImageLayout,
}

impl ImageDescription {
    pub fn new(width: u32, height: u32, format: ImageFormat, image_type: ImageType, sample_count: SampleCount, initial_layout: ImageLayout) -> Self {
        Self {
            width,
            height,
            format,
            image_type,
            sample_count,
            initial_layout,
        }
    }
}
