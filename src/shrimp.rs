use std::error::Error;

use pixels::{Pixels, SurfaceTexture};
use winit::{event::{Event, WindowEvent}, window::Window, dpi::LogicalPosition};

use crate::{input::InputHandler, core::World};

pub struct Shrimp {
    pub world: World,
    pub pixels: Pixels,
    pub input: InputHandler,
    center: LogicalPosition<f32>,
    dimensions: (u32, u32),
}

impl Shrimp {
    pub fn new(width: u32, height: u32, window: &Window) -> Result<Self, Box<dyn Error>>{
        Ok(Self {
            world: World::default(),
            pixels: Pixels::new(width, height, SurfaceTexture::new(width, height, &window))?,
            input: InputHandler::default(),
            center: LogicalPosition{x: width as f32 / 2.0, y: height as f32 / 2.0},
            dimensions: (width, height),
        })
    }

    pub fn render(&mut self) -> Result<(), Box<dyn Error>> {
        std::thread::sleep(std::time::Duration::from_millis(30));
        for (i, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = i % self.dimensions.0 as usize;
            let y = i / self.dimensions.0 as usize;
            let red = (x as f32 / self.dimensions.0 as f32 * 255.0) as u8;
            let green = (y as f32 / self.dimensions.1 as f32 * 255.0) as u8;
            pixel.copy_from_slice(&[red, green, 0, 255]);
        }
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