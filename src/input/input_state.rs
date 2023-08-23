use std::collections::HashMap;

use winit::event::VirtualKeyCode;


pub struct InputState {
    pub key_states: HashMap<VirtualKeyCode, bool>,
    pub mouse_movement: (f64, f64),
}

impl InputState {
    pub fn new() -> Self {
        Self {
            key_states: HashMap::new(),
            mouse_movement: (0.0, 0.0),
        }
    }

    pub fn update_key(&mut self, key: VirtualKeyCode, pressed: bool) {
        self.key_states.insert(key, pressed);
    }

    pub fn is_key_pressed(&self, key: &VirtualKeyCode) -> bool {
        if let Some(state) = self.key_states.get(key) {
            return *state;
        }
        false
    }
}