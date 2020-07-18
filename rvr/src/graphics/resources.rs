use ash::vk;
use crate::{
    Error,
    graphics::vulkan::*,
};

pub struct Resources {
    semaphores: SemaphorePool,
    fences: FencePool,
}

impl Resources {
    pub fn new(semaphores: SemaphorePool, fences: FencePool) -> Self {
        Self {
            semaphores,
            fences,
        }
    }

    pub fn get_semaphore(&mut self) -> Result<Semaphore, Error> {
        self.semaphores.get_semaphore()
    }
    
    pub fn return_semaphore(&mut self, semaphore: Semaphore) {
        self.semaphores.return_semaphore(semaphore);
    }
    
    pub fn get_fence(&mut self) -> Result<Fence, Error> {
        self.fences.get_fence()
    }

    pub fn return_fence(&mut self, fence: Fence) {
        self.fences.return_fence(fence);
    }

    pub fn return_command_pool(&mut self, command_pool: CommandPool) {
        drop(command_pool);
    }
}
