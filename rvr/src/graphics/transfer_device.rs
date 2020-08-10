use crate::{Error, graphics::vulkan::Device};
use std::sync::Arc;
use rvr_assets::shader::ShaderAsset;

pub struct TransferDevice {
    device: Arc<Device>,
}

impl TransferDevice {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,
        }
    }

    pub fn create_pipeline(&mut self, shader_asset: &ShaderAsset) -> Result<(), Error> {
        Ok(())
    }
}
