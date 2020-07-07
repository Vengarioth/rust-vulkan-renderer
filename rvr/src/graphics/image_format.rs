#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum ImageFormat {
    B8G8R8A8_SRGB,
    R8G8B8A8_SRGB,
    R32G32B32A32_SFLOAT,
    R32G32B32_SFLOAT,
    R32G32_SFLOAT,
    D32_SFLOAT,
}

use sdl2::pixels::PixelFormatEnum;

impl Into<ImageFormat> for PixelFormatEnum {
    fn into(self) -> ImageFormat {
        match self {
            PixelFormatEnum::RGB888 => ImageFormat::R8G8B8A8_SRGB,
            _ => unimplemented!()
        }
    }
    
}
