use ash::{vk, version::DeviceV1_0};
use std::sync::Arc;
use crate::Error;

pub struct CommandBuffer {
    inner: vk::CommandBuffer,
    device: Arc<ash::Device>,
}

impl CommandBuffer {
    pub fn new(device: Arc<ash::Device>, inner: vk::CommandBuffer) -> Self {
        Self {
            inner,
            device,
        }
    }

    pub(crate) fn get_inner(&self) -> vk::CommandBuffer {
        self.inner
    }

    pub fn begin(&self) -> Result<(), Error> {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.device.begin_command_buffer(self.inner, &command_buffer_begin_info)?;
        }

        Ok(())
    }

    pub fn end(&self) -> Result<(), Error> {
        unsafe {
            self.device.end_command_buffer(self.inner)?;
        }

        Ok(())
    }

    pub fn pipeline_barrier(&mut self, image: vk::Image) {

        let image_barrier = vk::ImageMemoryBarrier::builder()
            .image(image)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            })
            .build();

        unsafe {
            self.device.cmd_pipeline_barrier(
                self.inner,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::BY_REGION,
                &[],
                &[],
                &[image_barrier],
            );
        }
    }
}
