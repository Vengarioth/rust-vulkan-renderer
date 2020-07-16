use std::ptr;
use std::os::raw::c_void;
use ash::{
    vk,
    extensions::khr,
    version::{EntryV1_0, InstanceV1_0},
};
use raw_window_handle::RawWindowHandle;
use crate::Error;

pub struct Surface {
    inner: vk::SurfaceKHR,
    extension_loader: khr::Surface,
}

impl Surface {
    #[cfg(target_os = "windows")]
    pub fn create<E: EntryV1_0, I: InstanceV1_0>(entry: &E, instance: &I, handle: RawWindowHandle) -> Result<Self, Error> {
        let (hwnd, hinstance) = match handle {
            RawWindowHandle::Windows(windows) => (windows.hwnd, windows.hinstance),
            _ => panic!("unknown window handle"),
        };

        let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
            s_type: vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: Default::default(),
            hinstance: hinstance,
            hwnd: hwnd as *const c_void,
        };

        let win32_surface_loader = khr::Win32Surface::new(entry, instance);
        let extension_loader = khr::Surface::new(entry, instance);

        let inner = unsafe {win32_surface_loader.create_win32_surface(&win32_create_info, None)?};

        Ok(Self {
            inner,
            extension_loader,
        })
    }

    #[cfg(target_os = "macos")]
    pub fn create<E: EntryV1_0, I: InstanceV1_0>(entry: &E, instance: &I, handle: RawWindowHandle) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn get_physical_device_surface_support(&self, physical_device: vk::PhysicalDevice, queue_family_index: u32) -> bool {
        unsafe {
            self.extension_loader.get_physical_device_surface_support(physical_device, queue_family_index, self.inner).unwrap()
        }
    }

    pub fn get_physical_device_surface_formats(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<vk::SurfaceFormatKHR>, Error> {
        let formats = unsafe {
            self.extension_loader.get_physical_device_surface_formats(physical_device, self.inner)?
        };

        Ok(formats)
    }

    pub fn get_physical_device_surface_capabilities(&self, physical_device: vk::PhysicalDevice) -> Result<vk::SurfaceCapabilitiesKHR, Error> {
        let capabilities = unsafe {
            self.extension_loader.get_physical_device_surface_capabilities(physical_device, self.inner)?
        };

        Ok(capabilities)
    }

    pub fn get_inner(&self) -> vk::SurfaceKHR {
        self.inner
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.extension_loader.destroy_surface(self.inner, None);
        }
    }
}
