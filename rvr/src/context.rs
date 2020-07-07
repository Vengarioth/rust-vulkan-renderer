use crate::{
    platform::Window,
    graphics::Renderer,
};

pub struct Context {
    pub(crate) window: Window,
    pub(crate) renderer: Renderer,
}

impl Context {
    pub(crate) fn new(window: Window, renderer: Renderer) -> Self {
        Self {
            window,
            renderer,
        }
    }
}
