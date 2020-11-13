use crate::vector::Vec3f;

pub struct Light {
    pub position: Vec3f,
    pub intensity: f64
}

impl Light {
    pub fn new(position: Vec3f, intensity: f64) -> Light {
        Light {
            position,
            intensity
        }
    }
}