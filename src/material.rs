use crate::vector::{Vec3f};

#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse_color: Vec3f,
    pub albedo: Vec3f,
    pub specular_exponent: f64
}

impl Material {
    pub fn new (color: Vec3f, albedo: Vec3f, spec_exp: f64) -> Material {
        Material {
            diffuse_color: color,
            albedo,
            specular_exponent: spec_exp
        }
    }
}