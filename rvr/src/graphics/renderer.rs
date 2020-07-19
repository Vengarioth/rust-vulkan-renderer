use raw_window_handle::RawWindowHandle;
use ash::vk::{
    Image,
    Fence,
    Semaphore,
};
use crate::{
    Error,
    graphics::*,
    graphics::rendergraph::*,
    graphics::vulkan::*,
};
use std::ffi::CString;

pub struct Renderer {
    configuration: Configuration,
    swapchain_images: Vec<Image>,
    swapchain: Swapchain,
    resources: Resources,
    pending_frame_resources: Vec<FrameResources>,
    device: Device,
}

impl Renderer {
    pub fn create(configuration: Configuration, window_handle: RawWindowHandle) -> Result<Self, Error> {

        let device = Device::create(&CString::new("").unwrap(), 0, window_handle, true)?;
        let resources = Resources::new(device.create_semaphore_pool()?, device.create_fence_pool()?);
        let pending_frame_resources = Vec::new();
        let swapchain = device.create_swapchain(2, 2560, 1440)?;

        let swapchain_images = swapchain.get_images()?;

        Ok(Self {
            configuration,
            swapchain_images,
            swapchain,
            resources,
            pending_frame_resources,
            device,
        })
    }

    pub fn render(&mut self, graph: Graph) -> Result<(), Error> {
        
        let schedule = graph.compile_schedule();

        for i in (0..self.pending_frame_resources.len()).rev() {
            let fence = self.pending_frame_resources[i].get_fence();
            let completed = self.device.get_fence_status(fence.get_inner())?;

            if completed {
                let resources = self.pending_frame_resources.remove(i);
                resources.recycle(&mut self.resources);
            }
        }

        let mut frame_resources = FrameResources::builder();

        let acquire_semaphore = self.resources.get_semaphore()?;
        let submit_semaphore = self.resources.get_semaphore()?;
        let finish_submit_fence = self.resources.get_fence()?;
        let command_pool = self.device.create_command_pool(self.device.graphics_queue.get_family_index())?;
        if let Some(index) = self.swapchain.acquire_next_image(std::u64::MAX, acquire_semaphore.get_inner(), Fence::null())? {

            let mut command_buffers = command_pool.allocate_command_buffers(1, true)?;
            let mut command_buffer = command_buffers.remove(0);

            command_buffer.begin()?;

            for instruction in schedule.get_instructions().iter() {
                match instruction {
                    Instruction::CreateImage => {},
                    Instruction::ReleaseImage => {},
                    Instruction::ExecutePass => {},
                    Instruction::Present => {},
                    Instruction::ImageLayoutBarrier { id, from, to } => {
                        // TODO transition image
                        // TODO just take swapchain image for now

                        let image = &self.swapchain_images[index as usize];

                        command_buffer.pipeline_barrier(*image);
                    },
                }
            }

            command_buffer.end()?;

            self.device.queue_submit(
                self.device.graphics_queue.get_inner(),
                &command_buffer,
                &[ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
                &[acquire_semaphore.get_inner()],
                &[submit_semaphore.get_inner()],
                finish_submit_fence.get_inner(),
            )?;

            if self.swapchain.present(index, self.device.graphics_queue.get_inner(), submit_semaphore.get_inner())? {
                // TODO recreate swapchain
            }
        }

        frame_resources.add_semaphore(acquire_semaphore);
        frame_resources.add_semaphore(submit_semaphore);
        frame_resources.add_command_pool(command_pool);

        self.pending_frame_resources.push(frame_resources.build(finish_submit_fence));

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.device.wait_idle().unwrap();

        while self.pending_frame_resources.len() > 0 {
            let frame_resources = self.pending_frame_resources.remove(0);
            frame_resources.recycle(&mut self.resources);
        }
    }
}
