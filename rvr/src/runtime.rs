use crate::{
    Application,
    Error,
    Context,
    platform::Window,
    graphics::{
        Renderer,
        Configuration,
        rendergraph::{GraphBuilder, ImageHandle},
    },
};

pub struct Runtime {
    application: Box<dyn Application>,
    context: Context,
}

impl Runtime {
    pub fn create(window_title: &str, application: Box<dyn Application>) -> Result<Self, Error> {
        
        let window = Window::new(window_title)?;
        
        let configuration = Configuration::new();
        let renderer = Renderer::create(configuration, window.get_window_handle())?;

        let context = Context::new(
            window,
            renderer,
        );
        
        Ok(Self {
            application,
            context,
        })
    }

    pub fn run(mut self) -> Result<(), Error> {

        self.application.initialize(&mut self.context)?;
        
        loop {
            let exit = self.context.window.poll_events();

            self.application.update(&mut self.context)?;


            let mut builder = GraphBuilder::new();
            let backbuffer = builder.import_image("Backbuffer", self.context.window.get_backbuffer_image_description())?;
            let present_image = self.application.draw(&mut self.context, &mut builder, backbuffer)?;
            self.context.renderer.render(builder.build(&[present_image]))?;

            if exit {
                break;
            }
        }

        Ok(())
    }
}
