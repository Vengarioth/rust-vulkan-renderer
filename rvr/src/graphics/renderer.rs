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
        let resources = Resources::new(device.create_semaphore_pool()?);
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
        
        graph.compile_schedule();
        
        if self.pending_frame_resources.len() > 3 {
            // TODO hacky solution for now, use fence in the future
            self.pending_frame_resources.remove(0);
        }

        let mut frame_resources = FrameResources::builder();

        let semaphore = self.resources.get_semaphore()?;
        if let Some(index) = self.swapchain.acquire_next_image(std::u64::MAX, semaphore, Fence::null())? {

            if self.swapchain.present(index, self.device.graphics_queue, semaphore)? {
                // TODO recreate swapchain
            }
        }

        frame_resources.add_semaphore(semaphore);

        self.pending_frame_resources.push(frame_resources.build());

        Ok(())
    }
}
