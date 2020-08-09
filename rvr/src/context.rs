use crate::{
    platform::Window,
    graphics::Renderer,
    assets::AssetManager,
};

pub struct Context {
    pub(crate) window: Window,
    pub(crate) renderer: Renderer,
    pub(crate) assets: AssetManager,
}

impl Context {
    pub(crate) fn new(window: Window, renderer: Renderer, assets: AssetManager) -> Self {
        Self {
            window,
            renderer,
            assets,
        }
    }
}
