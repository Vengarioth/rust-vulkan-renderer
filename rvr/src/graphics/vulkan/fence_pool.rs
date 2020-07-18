use ash::{
    Device,
    version::DeviceV1_0,
    vk,
};
use std::sync::Arc;
use crate::Error;

pub struct FencePool {
    device: Arc<Device>,
}

impl FencePool {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,
        }
    }

    pub fn get_fence(&mut self) -> Result<vk::Fence, Error> {
        let create_info = vk::FenceCreateInfo::default();
        let fence = unsafe { self.device.create_fence(&create_info, None)? };
        Ok(fence)
    }

    pub fn return_fence(&mut self, fence: vk::Fence) {
        unsafe {
            self.device.destroy_fence(fence, None);
        }
    }
}
