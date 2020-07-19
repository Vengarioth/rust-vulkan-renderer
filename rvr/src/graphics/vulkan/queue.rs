use ash::vk;

pub struct Queue {
    family_index: u32,
    index: u32,
    inner: vk::Queue,

    supports_present: bool,
    supports_graphics: bool,
    supports_compute: bool,
    supports_transfer: bool,
}

impl Queue {
    pub fn new(
        family_index: u32,
        index: u32,
        inner: vk::Queue,
        supports_present: bool,
        supports_graphics: bool,
        supports_compute: bool,
        supports_transfer: bool,
    ) -> Self {
        Self {
            family_index,
            index,
            inner,
            supports_present,
            supports_graphics,
            supports_compute,
            supports_transfer,
        }
    }

    pub(crate) fn get_inner(&self) -> vk::Queue {
        self.inner
    }

    pub fn get_family_index(&self) -> u32 {
        self.family_index
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }
}
