use std::error::Error;

use winit::event::{KeyboardInput, DeviceEvent};

use super::input_state::InputState;

pub struct InputHandler {
    input_state: InputState
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            input_state: InputState::new()
        }
    }

    pub fn is_key_pressed(&self, key: &winit::event::VirtualKeyCode) -> bool {
        self.input_state.is_key_pressed(key)
    }

    pub(crate) fn device_event(&mut self, event: &DeviceEvent) -> Result<(), Box<dyn Error>> {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.input_state.mouse_movement = *delta;
        }
        Ok(())
    }

    pub(crate) fn key_event(&mut self, input: &KeyboardInput) -> Result<(), Box<dyn Error>>{
        self.input_state.update_key(input.virtual_keycode.unwrap(), input.state == winit::event::ElementState::Pressed);
        Ok(())
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}