use nalgebra::{Vector3};

pub type Vec3f = Vector3<f64>;
// pub type Vec2f = Vector2<f64>;

#[allow(non_snake_case)]
pub fn reflect(I: Vec3f, N: Vec3f) -> Vec3f {
    I - N * 2. * I.dot(&N)
}