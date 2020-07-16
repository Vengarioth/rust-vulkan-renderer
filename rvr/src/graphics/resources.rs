use ash::vk;
use crate::{
    Error,
    graphics::vulkan::SemaphorePool,
};

pub struct Resources {
    semaphores: SemaphorePool,
}

impl Resources {
    pub fn new(semaphores: SemaphorePool) -> Self {
        Self {
            semaphores,
        }
    }

    pub fn get_semaphore(&mut self) -> Result<vk::Semaphore, Error> {
        self.semaphores.get_semaphore()
    }

    pub fn return_semaphore(&mut self, semaphore: vk::Semaphore) {
        self.semaphores.return_semaphore(semaphore);
    }
}
