use super::*;
use crate::{
    Error,
    util::IdGenerator,
};

pub struct GraphBuilder {
    create_images: Vec<ImageResource>,
    import_images: Vec<ImageResource>,
    id_generator: IdGenerator,
    passes: Vec<Pass>,
}

impl GraphBuilder {
    pub(crate) fn new() -> Self {
        Self {
            create_images: Vec::new(),
            import_images: Vec::new(),
            id_generator: IdGenerator::new(),
            passes: Vec::new(),
        }
    }

    pub(crate) fn import_image(&mut self, name: &str, description: ImageDescription) -> Result<ImageHandle, Error> {
        let id = self.id_generator.next();
        self.import_images.push(ImageResource::new(id, name.to_string(), description));

        Ok(ImageHandle::new(id, 0))
    }

    pub fn create_image(&mut self, name: &str, description: ImageDescription) -> Result<ImageHandle, Error> {
        let id = self.id_generator.next();
        self.create_images.push(ImageResource::new(id, name.to_string(), description));

        Ok(ImageHandle::new(id, 0))
    }

    pub fn add_pass<I, E: 'static, PassData: 'static + Clone>(&mut self, name: &str, initialize: I, execute: E) -> PassData
        where I: FnOnce(&mut PassBuilder) -> PassData,
        E: FnOnce(PassData, &mut ExecuteContext)
    {
        let id = self.id_generator.next();
        let mut pass_builder = PassBuilder::new(id, name.to_string(), &mut self.id_generator);
        let pass_data = initialize(&mut pass_builder);
        
        let executor = FnOnceExecutor::new(pass_data.clone(), Box::new(execute));
        // Box::new(executor)
        // pass_builder.build();
        let pass = Pass::new();

        self.passes.push(pass);

        pass_data
    }
}
