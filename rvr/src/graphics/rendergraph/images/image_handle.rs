#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ImageHandle {
    pub(crate) id: u32,
    pub(crate) version: u32,
}

impl ImageHandle {
    pub(crate) fn new(id: u32, version: u32) -> Self {
        Self {
            id,
            version,
        }
    }

    pub(crate) fn previous_version(&self) -> Option<Self> {
        if self.version > 0 {
            Some(Self::new(self.id, self.version - 1))
        } else {
            None
        }
    }
}
