pub enum EntityType {
    SphereLight,
    Mesh,
    Sphere,
    Plane
}

trait Entity {
    fn get_type(&self) -> EntityType;
}