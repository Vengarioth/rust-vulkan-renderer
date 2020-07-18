use ash::vk;

pub struct Semaphore {
    inner: vk::Semaphore,
}

impl Semaphore {
    pub fn new(inner: vk::Semaphore) -> Self {
        Self {
            inner,
        }
    }

    pub(crate) fn get_inner(&self) -> vk::Semaphore {
        self.inner
    }
}
