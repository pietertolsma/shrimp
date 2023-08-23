use super::Camera;

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
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % camera.dimensions.0 as usize;
            let y = i / camera.dimensions.0 as usize;
            let red = (x as f32 / camera.dimensions.0 as f32 * 255.0) as u8;
            let green = (y as f32 / camera.dimensions.1 as f32 * 255.0) as u8;
            pixel.copy_from_slice(&[red, green, 0, 255]);
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}