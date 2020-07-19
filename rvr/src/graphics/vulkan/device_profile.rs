use ash::{
    vk,
    version::{EntryV1_0, InstanceV1_0},
};
use std::{
    fmt,
    ffi::CStr,
};
use super::{
    surface::Surface,
};
use crate::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct DeviceIdentifier {
    inner: [u8; vk::UUID_SIZE],
}

impl DeviceIdentifier {
    pub fn new(inner: [u8; vk::UUID_SIZE]) -> Self {
        Self {
            inner,
        }
    }
}

impl fmt::Display for DeviceIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeviceType {
    DiscreteGPU,
    IntegratedGPU,
    VirtualGPU,
    CPU,
    Other,
}

impl DeviceType {
    pub fn from_vk_device_type(device_type: vk::PhysicalDeviceType) -> Self {
        match device_type {
            vk::PhysicalDeviceType::CPU => Self::CPU,
            vk::PhysicalDeviceType::INTEGRATED_GPU => Self::IntegratedGPU,
            vk::PhysicalDeviceType::VIRTUAL_GPU => Self::VirtualGPU,
            vk::PhysicalDeviceType::DISCRETE_GPU => Self::DiscreteGPU,
            _ => Self::Other,
        }
    }

    pub fn get_score(&self) -> f32 {
        match self {
            Self::DiscreteGPU => 1.0,
            Self::IntegratedGPU => 0.9,
            Self::VirtualGPU => 0.7,
            Self::CPU => 0.5,
            Self::Other => 0.1,
        }
    }

    pub fn to_vk_device_type(&self) -> vk::PhysicalDeviceType {
        match self {
            Self::CPU => vk::PhysicalDeviceType::CPU,
            Self::IntegratedGPU => vk::PhysicalDeviceType::INTEGRATED_GPU,
            Self::VirtualGPU => vk::PhysicalDeviceType::VIRTUAL_GPU,
            Self::DiscreteGPU => vk::PhysicalDeviceType::DISCRETE_GPU,
            Self::Other => vk::PhysicalDeviceType::OTHER,
        }
    }
}

#[derive(Debug)]
pub struct DeviceProfile {
    physical_device: vk::PhysicalDevice,
    device_type: DeviceType,
    device_identifier: DeviceIdentifier,
    device_name: String,
    graphics_queue_index: Option<usize>,
    transfer_queue_index: Option<usize>,
    compute_queue_index: Option<usize>,
    score: f32,
}

impl DeviceProfile {

    pub fn get_physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device.clone()
    }

    pub fn get_graphics_queue_index(&self) -> Option<usize> {
        self.graphics_queue_index
    }

    pub fn get_compute_queue_index(&self) -> Option<usize> {
        self.compute_queue_index
    }

    pub fn get_transfer_queue_index(&self) -> Option<usize> {
        self.transfer_queue_index
    }

    pub unsafe fn query_device_profiles<E: EntryV1_0, I: InstanceV1_0>(entry: &E, instance: &I, surface: &Surface) -> Result<Vec<DeviceProfile>, Error> {
        let profiles = instance.enumerate_physical_devices()?
            .iter()
            .map(|pdevice| {
                let properties = instance.get_physical_device_properties(*pdevice);
                let queue_families = instance.get_physical_device_queue_family_properties(*pdevice);

                let mut score = 0.0;
                
                let device_type = DeviceType::from_vk_device_type(properties.device_type);
                score += device_type.get_score() * 1000.0;

                let device_identifier = DeviceIdentifier::new(properties.pipeline_cache_uuid);

                let device_name = CStr::from_ptr(properties.device_name.as_ptr())
                    .to_str()
                    .unwrap_or("Unknown")
                    .to_owned();


                let mut graphics_queue_index = None;
                let mut transfer_queue_index = None;
                let mut compute_queue_index = None;

                for index in 0..queue_families.len() {
                    let queue_family = queue_families[index];
                    let surface_support = surface.get_physical_device_surface_support(*pdevice, index as u32);
                    let supports_graphics = queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                    let supports_compute = queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE);
                    let supports_transfer = queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER);

                    if supports_graphics && surface_support && supports_compute && supports_transfer && graphics_queue_index.is_none() {
                        graphics_queue_index = Some(index);
                    } else if supports_compute && supports_transfer && compute_queue_index.is_none() {
                        compute_queue_index = Some(index);
                    } else if supports_transfer && !supports_graphics && !supports_compute && transfer_queue_index.is_none() {
                        transfer_queue_index = Some(index);
                    }
                }

                DeviceProfile {
                    physical_device: *pdevice,
                    device_type,
                    device_identifier,
                    device_name,
                    graphics_queue_index,
                    transfer_queue_index,
                    compute_queue_index,
                    score,
                }
            })
            .collect();

        Ok(profiles)
    }

    pub fn find_highest_score(profiles: &Vec<DeviceProfile>) -> Option<usize> {
        let mut max_value = std::f32::NEG_INFINITY;
        let mut index = None;

        for i in 0..profiles.len() {
            let score = profiles[i].score;
            if score > max_value {
                max_value = score;
                index = Some(i);
            }
        }

        index
    }
}
