use crate::{
    Error,
    Context,
    graphics::rendergraph::{GraphBuilder, ImageHandle},
};

pub trait Application {
    fn initialize(&mut self, context: &mut Context) -> Result<(), Error>;
    fn update(&mut self, context: &mut Context) -> Result<(), Error>;
    fn draw(&mut self, context: &mut Context, builder: &mut GraphBuilder, backbuffer: ImageHandle) -> Result<ImageHandle, Error>;
}
