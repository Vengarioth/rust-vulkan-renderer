use raw_window_handle::*;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    video::{Window as SdlWindow},
    EventPump,
    Sdl as SdlContext,
    VideoSubsystem,
};
use crate::{
    Error,
    RuntimeError,
    graphics::{
        ImageFormat,
        ImageType,
        ImageLayout,
        SampleCount,
        rendergraph::*,
    }
};

pub struct Window {
    inner: SdlWindow,
    video_subsystem: VideoSubsystem,
    event_pump: EventPump,

    width: u32,
    height: u32,
    refresh_rate: u32,
    format: ImageFormat,
}

impl Window {
    pub fn new(title: &str) -> Result<Self, Error> {
        
        let sdl_context = sdl2::init()
            .map_err(|e| RuntimeError::PlatformError(e))?;

        let video_subsystem = sdl_context.video()
            .map_err(|e| RuntimeError::PlatformError(e))?;

        let display_mode = video_subsystem.current_display_mode(0)
            .map_err(|e| RuntimeError::PlatformError(e))?;

        dbg!(&display_mode);

        let width = display_mode.w as u32;
        let height = display_mode.h as u32;
        let format = display_mode.format.into();
        let refresh_rate = display_mode.refresh_rate as u32;

        let mut inner = video_subsystem.window(title, display_mode.w as u32, display_mode.h as u32)
            .vulkan()
            .borderless()
            .build()
            .unwrap();

        inner.set_display_mode(display_mode)
            .map_err(|e| RuntimeError::PlatformError(e))?;

        let event_pump = sdl_context.event_pump()
            .map_err(|e| RuntimeError::PlatformError(e))?;
        
        Ok(Self {
            inner,
            video_subsystem,
            event_pump,

            width,
            height,
            format,
            refresh_rate,
        })
    }

    pub fn get_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }

    pub fn get_backbuffer_image_description(&self) -> ImageDescription {
        ImageDescription::new(self.width, self.height, self.format, ImageType::Type2D, SampleCount::Type_1, ImageLayout::Unknown)
    }

    pub fn poll_events(&mut self) -> bool {
        let mut exit = false;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit = true;
                },
                _ => {}
            }
        }

        exit
    }
}
