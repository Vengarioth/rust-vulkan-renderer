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
}
