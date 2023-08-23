use ray::Ray;

struct Bounds {
    pub position: [f32; 3],
    pub size: [f32; 3],
}

impl Bounds {
    pub fn new(position: [f32; 3], size: [f32; 3]) -> Self {
        Self {
            position,
            size,
        }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        todo!("Implement Bounds::intersects");
    }
}