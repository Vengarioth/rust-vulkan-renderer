use ash::{
    vk,
    extensions::khr,
};
use std::sync::Arc;
use super::{
    Surface,
};
use crate::{
    Error,
    graphics::{
        GraphicsError,
        vulkan::*,
    },
};

pub struct Swapchain {
    device: Arc<ash::Device>,
    extension_loader: khr::Swapchain,
    inner: vk::SwapchainKHR,
    surface_format: vk::SurfaceFormatKHR,
}

impl Swapchain {
    pub fn new(
        instance: &ash::Instance,
        device: Arc<ash::Device>,
        surface: &Surface,
        physical_device: vk::PhysicalDevice,
        min_image_count: u32,
        width: u32,
        height: u32
    ) -> Result<Self, Error> {

        let extension_loader = khr::Swapchain::new(instance, &*device);
        let surface_formats = surface.get_physical_device_surface_formats(physical_device)?;
        let surface_capabilities = surface.get_physical_device_surface_capabilities(physical_device)?;

        if surface_formats.len() < 1 {
            return Err(GraphicsError::NoSuitableSurfaceFormat.into());
        }

        let mut index = 0;

        for (i, format) in surface_formats.iter().enumerate() {
            if format.format == vk::Format::B8G8R8A8_SRGB {
                index = i;
            }
        }

        let surface_format = surface_formats[index];

        if surface_format.format != vk::Format::B8G8R8A8_SRGB || surface_format.color_space != vk::ColorSpaceKHR::SRGB_NONLINEAR {
            unimplemented!("TODO implement manual gamma correction");
        }

        let surface_resolution = vk::Extent2D {
            width,
            height,
        };

        let pre_transform = if surface_capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface.get_inner())
            .min_image_count(min_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution.clone())
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_DST)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::MAILBOX)
            .clipped(true)
            .image_array_layers(1);

        let inner = unsafe { extension_loader.create_swapchain(&swapchain_create_info, None)? };

        Ok(Swapchain {
            device,
            extension_loader,
            inner,
            surface_format,
        })
    }

    pub fn acquire_next_image(&self, timeout: u64, semaphore: vk::Semaphore, fence: vk::Fence) -> Result<Option<u32>, Error> {
        let result = unsafe {
            self.extension_loader.acquire_next_image(self.inner, timeout, semaphore, fence)
        };

        match result {
            Ok((index, false)) => {
                Ok(Some(index))
            },
            Ok((_, true)) => {
                Ok(None)
            }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                Ok(None)
            },
            Err(error) => {
                Err(error.into())
            },
        }
    }

    pub fn get_image_format(&self) -> vk::Format {
        self.surface_format.format
    }

    pub fn present(&self, index: u32, queue: vk::Queue, wait_semaphore: vk::Semaphore) -> Result<bool, Error> {

        let wait_semaphores = [wait_semaphore];
        let swapchains = [self.inner];
        let image_indices = [index];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);
        
        unsafe {
            match self.extension_loader.queue_present(queue, &present_info) {
                Ok(_) => Ok(false),
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok(true),
                Err(e) => Err(e.into()),
            }
        }
    }

    pub fn get_images(&self) -> Result<Vec<vk::Image>, Error> {
        let images = unsafe { self.extension_loader.get_swapchain_images(self.inner)? };
        Ok(images)
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.extension_loader.destroy_swapchain(self.inner, None);
        }
    }
}
