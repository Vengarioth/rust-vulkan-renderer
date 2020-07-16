use std::ffi::CString;

pub struct Layers {
    layers: Vec<CString>,
}

impl Layers {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
        }
    }

    pub fn enable_validation_layers(&mut self) {
        if cfg!(debug_assertions) {
            self.layers.push(CString::new("VK_LAYER_LUNARG_standard_validation").unwrap());
            self.layers.push(CString::new("VK_LAYER_KHRONOS_validation").unwrap());
        }
    }

    pub fn get_names_raw(&self) -> Vec<*const i8> {
        let raw: Vec<*const i8> = self.layers
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();
        
        raw
    }
}
