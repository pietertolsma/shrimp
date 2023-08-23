pub struct Camera {
    pub dimensions: (u32, u32),
}

impl Camera {
    pub fn new(dimensions: (u32, u32)) -> Self {
        Self {
            dimensions,
        }
    }
}