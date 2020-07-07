use rvr::{
    catch,
    Error,
    Application,
    Context,
    Runtime,
    graphics::rendergraph::*,
};

pub struct RVRApplication;

impl Application for RVRApplication {
    fn initialize(&mut self, context: &mut Context) -> Result<(), Error> {
        Ok(())
    }
    fn update(&mut self, context: &mut Context) -> Result<(), Error> {
        Ok(())
    }
    fn draw(&mut self, context: &mut Context, builder: &mut GraphBuilder, backbuffer: ImageHandle) -> Result<ImageHandle, Error> {
        Ok(backbuffer)
    }
}

fn main() {
    catch(|| {
        let application = RVRApplication;
        let runtime = Runtime::create("Rust Vulkan Renderer", Box::new(application))?;
        runtime.run()?;
    
        Ok(())
    });
}
