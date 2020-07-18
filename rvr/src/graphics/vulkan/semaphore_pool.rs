use ash::{
    vk,
    version::*,
};
use crate::Error;
use std::sync::Arc;

pub struct SemaphorePool {
    device: Arc<ash::Device>,
}

impl SemaphorePool {
    pub fn new(device: Arc<ash::Device>) -> Self {
        Self {
            device,
        }
    }

    pub fn get_semaphore(&mut self) -> Result<vk::Semaphore, Error> {
        let create_info = vk::SemaphoreCreateInfo::default();
        let semaphore = unsafe { self.device.create_semaphore(&create_info, None)? };
        Ok(semaphore)
    }

    pub fn return_semaphore(&mut self, semaphore: vk::Semaphore) {
        unsafe {
            self.device.destroy_semaphore(semaphore, None);
        }
    }
}
