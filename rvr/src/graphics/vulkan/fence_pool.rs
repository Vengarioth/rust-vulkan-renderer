use ash::{
    Device,
    version::DeviceV1_0,
    vk,
};
use std::sync::Arc;
use crate::Error;
use super::*;

pub struct FencePool {
    device: Arc<Device>,
}

impl FencePool {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            device,
        }
    }

    pub fn get_fence(&mut self) -> Result<Fence, Error> {
        let create_info = vk::FenceCreateInfo::default();
        let inner = unsafe { self.device.create_fence(&create_info, None)? };
        Ok(Fence::new(inner))
    }

    pub fn return_fence(&mut self, fence: Fence) {
        unsafe {
            self.device.destroy_fence(fence.get_inner(), None);
        }
    }
}
