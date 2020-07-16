use ash::vk;
use crate::graphics::Resources;

pub struct FrameResourceBuilder {
    semaphores: Vec<vk::Semaphore>,
}

impl FrameResourceBuilder {
    fn new() -> Self {
        Self {
            semaphores: Vec::new(),
        }
    }

    pub fn add_semaphore(&mut self, semaphore: vk::Semaphore) {
        self.semaphores.push(semaphore);
    }

    pub fn build(self) -> FrameResources {
        FrameResources::new(self.semaphores)
    }
}

pub struct FrameResources {
    semaphores: Vec<vk::Semaphore>,
}

impl FrameResources {
    pub fn builder() -> FrameResourceBuilder {
        FrameResourceBuilder::new()
    }
    
    fn new(semaphores: Vec<vk::Semaphore>) -> Self {
        Self {
            semaphores,
        }
    }

    pub fn recycle(self, resources: &mut Resources) {
        for semaphore in self.semaphores {
            resources.return_semaphore(semaphore);
        }
    }
}
