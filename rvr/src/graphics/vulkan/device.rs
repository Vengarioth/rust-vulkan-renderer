use ash::{
    vk,
    Entry,
    extensions::khr,
    version::*,
};
use vk_mem::{Allocator, AllocatorCreateInfo};
use raw_window_handle::RawWindowHandle;
use std::{
    ffi::{CStr, CString},
    mem::ManuallyDrop,
    sync::Arc,
};
use crate::{
    Error,
    graphics::GraphicsError,
    graphics::vulkan::*,
};

pub struct Device {
    entry: Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,

    layers: Layers,
    extensions: Extensions,

    debugger: Option<ManuallyDrop<Debugger>>,
    surface: ManuallyDrop<Surface>,
    allocator: ManuallyDrop<Arc<Allocator>>,

    inner: Arc<ash::Device>,

    pub graphics_queue_index: u32,
    pub graphics_queue: vk::Queue,
    pub transfer_queue_index: u32,
    pub transfer_queue: vk::Queue,
}

impl Device {
    pub fn create(app_name: &CStr, app_version: u32, window_handle: RawWindowHandle, debugging_enabled: bool) -> Result<Self, Error> {
        let entry = ash::Entry::new()?;

        let engine_name = CString::new("Rust Vulkan Renderer").unwrap();

        let application_info = vk::ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(app_version)
            .engine_name(&engine_name)
            .engine_version(vk::make_version(0, 1, 0))
            .api_version(vk::make_version(1, 2, 138));
        
        let mut layers = Layers::new();
        let mut extensions = Extensions::new();

        if debugging_enabled {
            layers.enable_validation_layers();
            extensions.enable_ext_debug_report();
        }

        extensions.enable_khr_surface();
        extensions.enable_khr_win32_surface();

        let layer_names_raw = layers.get_names_raw();
        let extension_names_raw = extensions.get_names_raw();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&layer_names_raw)
            .enabled_extension_names(&extension_names_raw);

        let instance: ash::Instance = unsafe { entry.create_instance(&create_info, None)? };

        let debugger = if debugging_enabled {
            Some(ManuallyDrop::new(Debugger::create(&entry, &instance)?))
        } else {
            None
        };

        let surface = Surface::create(&entry, &instance, window_handle)?;

        let profiles = unsafe { DeviceProfile::query_device_profiles(&entry, &instance, &surface)? };
        let device_index = DeviceProfile::find_highest_score(&profiles).ok_or(GraphicsError::NoSuitableDevice)?;
        let profile = &profiles[device_index];
        let physical_device = profile.get_physical_device();

        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            shader_uniform_buffer_array_dynamic_indexing: 1,
            shader_sampled_image_array_dynamic_indexing: 1,
            shader_storage_buffer_array_dynamic_indexing: 1,
            shader_storage_image_array_dynamic_indexing: 1,
            sampler_anisotropy: 1,
            ..Default::default()
        };

        let mut descriptor_features = vk::PhysicalDeviceDescriptorIndexingFeatures::builder()
            .descriptor_binding_partially_bound(true)
            .descriptor_binding_update_unused_while_pending(true)
            .descriptor_binding_sampled_image_update_after_bind(true)
            .descriptor_binding_variable_descriptor_count(true);

        let graphics_queue_index = profile
            .get_graphics_queue_index()
            .ok_or(GraphicsError::NoSuitableGraphicsQueue)? as u32;
        let transfer_queue_index = profile
            .get_transfer_queue_index()
            .ok_or(GraphicsError::NoSuitableTransferQueue)? as u32;

        let graphics_priorities = [1.0];
        let transfer_priorities = [0.8];
        let device_extension_names_raw = [khr::Swapchain::name().as_ptr()];

        let queue_infos = [
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(graphics_queue_index as u32)
                .queue_priorities(&graphics_priorities)
                .build(),
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(transfer_queue_index as u32)
                .queue_priorities(&transfer_priorities)
                .build(),
        ];

        let device_create_info = vk::DeviceCreateInfo::builder()
            .push_next(&mut descriptor_features)
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let inner = unsafe { instance.create_device(physical_device, &device_create_info, None)? };
        let graphics_queue = unsafe { inner.get_device_queue(graphics_queue_index, 0) };
        let transfer_queue = unsafe { inner.get_device_queue(transfer_queue_index, 0) };

        let allocator_create_info = AllocatorCreateInfo {
            physical_device: physical_device,
            device: inner.clone(),
            instance: instance.clone(),
            frame_in_use_count: 3,
            ..Default::default()
        };
        let allocator = vk_mem::Allocator::new(&allocator_create_info)?;

        Ok(Self {
            entry,
            instance,
            physical_device,
            
            layers,
            extensions,
            
            debugger,
            surface: ManuallyDrop::new(surface),
            allocator: ManuallyDrop::new(Arc::new(allocator)),

            inner: Arc::new(inner),

            graphics_queue_index,
            graphics_queue,
            transfer_queue_index,
            transfer_queue,
        })
    }

    pub fn create_swapchain(&self, min_image_count: u32, width: u32, height: u32) -> Result<Swapchain, Error> {
        Swapchain::new(
            &self.instance,
            Arc::clone(&self.inner),
            &self.surface,
            self.physical_device,
            min_image_count,
            width,
            height,
        )
    }

    pub fn create_semaphore_pool(&self) -> Result<SemaphorePool, Error> {
        Ok(SemaphorePool::new(Arc::clone(&self.inner)))
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.allocator);
            self.inner.destroy_device(None);
            ManuallyDrop::drop(&mut self.surface);

            if let Some(ref mut debugger) = self.debugger {
                ManuallyDrop::drop(debugger);
            }

            self.instance.destroy_instance(None);
        }
    }
}
