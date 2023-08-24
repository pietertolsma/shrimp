use super::Camera;

use rayon::prelude::*;

pub struct World {
}

impl World {

    pub fn new() -> Self {
        Self {
        }
    }

    pub fn update(&mut self) {
        // ...
    }

    pub fn render(&mut self, frame: &mut [u8], camera: &Camera) {
        let world_coords = &camera.world_coords();

        // loop using rayon
        frame.par_chunks_exact_mut(4).enumerate().for_each(|(i, pixel)| {
            let mut color: [u8; 4] = [255, 255, 255, 255];

            if world_coords[(i, 1)] < 0.0 {
                color = [0, 0, 0, 255];
            } else if world_coords[(i, 0)] <= 0.0 {
                color = [255, 0, 0, 255];
            }

            pixel.copy_from_slice(&color);
        });
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}