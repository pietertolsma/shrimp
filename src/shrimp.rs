use std::error::Error;

use pixels::{Pixels, SurfaceTexture};
use winit::{event::{Event, WindowEvent}, window::Window, dpi::LogicalPosition};

use crate::{input::InputHandler, core::{World, Camera}};

pub struct Shrimp {
    pub world: World,
    pub camera: Camera,
    pub pixels: Pixels,
    pub input: InputHandler,
    center: LogicalPosition<f32>,
}

impl Shrimp {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>>{
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        Ok(Self {
            world: World::default(),
            camera: Camera::new((width, height)),
            pixels: Pixels::new(width, height, SurfaceTexture::new(width, height, &window))?,
            input: InputHandler::default(),
            center: LogicalPosition{x: width as f32 / 2.0, y: height as f32 / 2.0},
        })
    }

    pub fn render(&mut self) -> Result<(), Box<dyn Error>> {
        self.world.render(self.pixels.frame_mut(), &self.camera);

        self.pixels.render()?;
        Ok(())
    }

    pub fn update(&mut self){
        // sleep for 0.5s
    }

    pub fn handle_input(&mut self, event: &Event<()>, window: &Window) -> Result<(), Box<dyn Error>> {
        match event {
            Event::WindowEvent {event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => self.input.key_event(input),
                    WindowEvent::CursorMoved { .. } => {
                        // center mouse position
                        window.set_cursor_visible(false);
                        window.set_cursor_position(self.center)?;
                        Ok(())
                    }
                    WindowEvent::Destroyed => {
                        Err("Window was destroyed".into())
                    },
                    _ => Ok(()),
                }
            }
            Event::DeviceEvent {event, ..} => self.input.device_event(event),
            _ => Ok(()),
        }
    }
}