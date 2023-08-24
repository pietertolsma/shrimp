use std::error::Error;

use pixels::{Pixels, SurfaceTexture};
use winit::{event::{Event, WindowEvent}, window::Window};

use crate::{input::InputHandler, core::{World, Camera}, ui::UserInterface};

pub struct Shrimp {
    pub world: World,
    pub camera: Camera,
    pub ui: UserInterface,
    pub pixels: Pixels,
    pub input: InputHandler,
}

impl Shrimp {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>>{
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        // center mouse
        Ok(Self {
            world: World::default(),
            camera: Camera::new(45.0,(width, height)),
            ui: UserInterface::new(),
            pixels: Pixels::new(width, height, SurfaceTexture::new(width, height, &window))?,
            input: InputHandler::default(),
        })
    }

    pub fn render(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: Limit framerate?

        self.world.render(self.pixels.frame_mut(), &self.camera);

        self.pixels.render()?;
        Ok(())
    }

    pub fn update(&mut self, window: &Window){
        self.camera.update(&mut self.input.input_state);
        self.ui.update(window, &self.input.input_state);
    }

    pub fn handle_input(&mut self, event: &Event<()>) -> Result<(), Box<dyn Error>> {
        match event {
            Event::WindowEvent {event: WindowEvent::KeyboardInput { input, ..}, .. } => self.input.key_event(input),
            Event::DeviceEvent {event, ..} => self.input.device_event(event),
            _ => Ok(()),
        }
    }
}