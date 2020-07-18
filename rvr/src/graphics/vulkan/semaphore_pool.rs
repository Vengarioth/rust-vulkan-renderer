use ash::{
    vk,
    version::*,
};
use crate::Error;
use super::*;
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

    pub fn get_semaphore(&mut self) -> Result<Semaphore, Error> {
        let create_info = vk::SemaphoreCreateInfo::default();
        let inner = unsafe { self.device.create_semaphore(&create_info, None)? };
        Ok(Semaphore::new(inner))
    }

    pub fn return_semaphore(&mut self, semaphore: Semaphore) {
        unsafe {
            self.device.destroy_semaphore(semaphore.get_inner(), None);
        }
    }
}
