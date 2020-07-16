use std::ffi::CString;
use ash::extensions::{ext, khr};

pub struct Extensions {
    extensions: Vec<CString>,
}

impl Extensions {
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    pub fn enable_ext_debug_report(&mut self) {
        self.extensions.push(CString::from(ext::DebugReport::name()));
    }

    pub fn enable_khr_surface(&mut self) {
        self.extensions.push(CString::from(khr::Surface::name()))
    }

    pub fn enable_khr_win32_surface(&mut self) {
        self.extensions.push(CString::from(khr::Win32Surface::name()))
    }

    pub fn get_names_raw(&self) -> Vec<*const i8> {
        let raw: Vec<*const i8> = self.extensions
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();
        
        raw
    }
}
