use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphicsError {
    #[error("No suitable graphics device found")]
    NoSuitableDevice,

    #[error("No suitable graphics queue found")]
    NoSuitableGraphicsQueue,

    #[error("No suitable transfer queue found")]
    NoSuitableTransferQueue,

    #[error("No suitable surface format found")]
    NoSuitableSurfaceFormat,
}
