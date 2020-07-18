use ash::vk;
use crate::graphics::Resources;
use crate::graphics::vulkan::*;

pub struct FrameResourceBuilder {
    semaphores: Vec<Semaphore>,
    command_pools: Vec<CommandPool>,
}

impl FrameResourceBuilder {
    fn new() -> Self {
        Self {
            semaphores: Vec::new(),
            command_pools: Vec::new(),
        }
    }

    pub fn add_semaphore(&mut self, semaphore: Semaphore) {
        self.semaphores.push(semaphore);
    }

    pub fn add_command_pool(&mut self, command_pool: CommandPool) {
        self.command_pools.push(command_pool);
    }

    pub fn build(self, fence: Fence) -> FrameResources {
        FrameResources::new(
            fence,
            self.semaphores,
            self.command_pools,
        )
    }
}

pub struct FrameResources {
    fence: Fence,
    semaphores: Vec<Semaphore>,
    command_pools: Vec<CommandPool>,
}

impl FrameResources {
    pub fn builder() -> FrameResourceBuilder {
        FrameResourceBuilder::new()
    }
    
    fn new(
        fence: Fence,
        semaphores: Vec<Semaphore>,
        command_pools: Vec<CommandPool>,
    ) -> Self {
        Self {
            fence,
            semaphores,
            command_pools,
        }
    }

    pub fn get_fence(&self) -> &Fence {
        &self.fence
    }

    pub fn recycle(self, resources: &mut Resources) {
        self.semaphores.into_iter().for_each(|semaphore| resources.return_semaphore(semaphore));
        self.command_pools.into_iter().for_each(|command_pool| resources.return_command_pool(command_pool));
        resources.return_fence(self.fence);
    }
}
