use std::collections::HashMap;

use winit::event::VirtualKeyCode;


pub struct InputState {
    pub key_pressed_states: HashMap<VirtualKeyCode, bool>,
    pub key_toggle_states: HashMap<VirtualKeyCode, bool>,
    pub mouse_movement: (f64, f64),
}

impl InputState {
    pub fn new() -> Self {
        Self {
            key_pressed_states: HashMap::new(),
            key_toggle_states: HashMap::new(),
            mouse_movement: (0.0, 0.0),
        }
    }

    pub fn update_key(&mut self, key: VirtualKeyCode, pressed: bool) {
        self.key_pressed_states.insert(key, pressed);

        if !pressed {
            if let Some(state) = self.key_toggle_states.get_mut(&key) {
                *state = !*state;
            } else {
                self.key_toggle_states.insert(key, true);
            }
        }
    }

    pub fn is_key_pressed(&self, key: &VirtualKeyCode) -> bool {
        if let Some(state) = self.key_pressed_states.get(key) {
            return *state;
        }
        false
    }

    pub fn is_key_toggled(&self, key: &VirtualKeyCode) -> bool {
        if let Some(state) = self.key_toggle_states.get(key) {
            return *state;
        }
        false
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}