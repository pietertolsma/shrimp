use winit::window::{Window, CursorGrabMode};

use crate::input::InputState;

pub struct UserInterface {

}

impl UserInterface {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn update(&mut self, window: &Window, input: &InputState) {
        let mouse_locked = !input.is_key_toggled(&winit::event::VirtualKeyCode::Escape);
        if mouse_locked {
            window.set_cursor_position(winit::dpi::PhysicalPosition::new(window.inner_size().width/2, window.inner_size().height/2)).unwrap();
        }
        window.set_cursor_visible(!mouse_locked);
        window.set_cursor_grab([CursorGrabMode::None, CursorGrabMode::Locked][mouse_locked as usize]).unwrap();
    }
}

impl Default for UserInterface {
    fn default() -> Self {
        Self::new()
    }
}