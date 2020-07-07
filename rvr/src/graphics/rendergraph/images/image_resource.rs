use crate::graphics::rendergraph::*;

#[derive(Debug)]
pub struct ImageResource {
    pub id: u32,
    pub name: String,
    pub description: ImageDescription,
}

impl ImageResource {
    pub fn new(id: u32, name: String, description: ImageDescription) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}
