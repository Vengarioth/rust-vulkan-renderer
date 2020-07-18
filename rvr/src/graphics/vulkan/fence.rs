use ash::vk;

pub struct Fence {
    inner: vk::Fence,
}

impl Fence {
    pub fn new(inner: vk::Fence) -> Self {
        Self {
            inner,
        }
    }

    pub(crate) fn get_inner(&self) -> vk::Fence {
        self.inner
    }
}
