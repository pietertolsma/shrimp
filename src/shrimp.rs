use std::error::Error;

use pixels::{Pixels, SurfaceTexture};
use winit::{event::{Event, WindowEvent}, window::{Window, CursorGrabMode}};

use crate::{input::InputHandler, core::{World, Camera}};

pub struct Shrimp {
    pub world: World,
    pub camera: Camera,
    pub pixels: Pixels,
    pub input: InputHandler,
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
        })
    }

    pub fn render(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: Limit framerate?

        self.world.render(self.pixels.frame_mut(), &self.camera);

        self.pixels.render()?;
        Ok(())
    }

    pub fn update(&mut self, window: &Window){
        // Game tick
        // Example: move player based on wasd and mouse input

        let mouse_locked = self.input.input_state.is_key_toggled(&winit::event::VirtualKeyCode::Escape);
        window.set_cursor_visible(mouse_locked);
        window.set_cursor_grab([CursorGrabMode::Locked, CursorGrabMode::None][mouse_locked as usize]).unwrap();
    }

    pub fn handle_input(&mut self, event: &Event<()>) -> Result<(), Box<dyn Error>> {
        match event {
            Event::WindowEvent {event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => self.input.key_event(input),
                    WindowEvent::CursorMoved { .. } => {            
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