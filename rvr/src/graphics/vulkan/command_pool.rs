use ash::{vk, version::DeviceV1_0};
use std::sync::Arc;
use crate::Error;
use super::*;

pub struct CommandPool {
    inner: vk::CommandPool,
    device: Arc<ash::Device>,
}

impl CommandPool {
    pub fn create(device: Arc<ash::Device>, queue_family_index: u32) -> Result<Self, Error> {
        
        let create_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family_index);

        let inner = unsafe {
            device.create_command_pool(&create_info, None)?
        };
        
        Ok(Self {
            inner,
            device,
        })
    }

    pub fn allocate_command_buffers(&self, count: u32, primary: bool) -> Result<Vec<CommandBuffer>, Error> {

        let buffer_level = if primary {
            vk::CommandBufferLevel::PRIMARY
        } else {
            vk::CommandBufferLevel::SECONDARY
        };

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_buffer_count(count)
            .command_pool(self.inner)
            .level(buffer_level)
            .build();

        let buffers = unsafe {
            self.device.allocate_command_buffers(&command_buffer_allocate_info)?
        };

        let buffers = buffers.into_iter()
            .map(|inner| CommandBuffer::new(Arc::clone(&self.device), inner))
            .collect();

        Ok(buffers)
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_command_pool(self.inner, None);
        }
    }
}
