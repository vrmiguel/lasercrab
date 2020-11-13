// use nalgebra::{Vector3};

// pub type Vec3f = Vector3<f64>;
// pub type Vec2f = Vector2<f64>;



use std::ops::Index;
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::Add;
use std::ops::Sub;

/// The origin of a three-dimensional Euclidean space
pub const ORIGIN: Vec3f = Vec3f::new(0., 0., 0.);
/// The î unit vector.
pub const I: Vec3f      = Vec3f::new(1., 0., 0.);

#[derive(Copy, Clone, Debug, PartialEq)]
/// Simple Three-dimensional euclidian otherector
pub struct Vec3f {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Vec3f {
	pub const fn new(x: f64, y: f64, z: f64) -> Vec3f
	{
		Vec3f {
			x,
			y,
			z
		}
	}

	pub fn dot(& self, other: &Vec3f) -> f64 {
		self.x * other.x +
		self.y * other.y +
		self.z * other.z
	}

	pub fn cross(&self, other: &Vec3f) -> Vec3f {
		Vec3f::new (
			self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
		)
	}

	/// Euclidean norm
	pub fn norm(& self) -> f64 {
		(self.x * self.x +
		 self.y * self.y +
		 self.z * self.z
		).sqrt()
	}

	pub fn normalize(&self) -> Vec3f {
		let norm = self.norm();
		*self * (1.0/norm)
	}
}

impl Add for &Vec3f {
	type Output = Vec3f;
	fn add(self, other: &Vec3f) -> Vec3f {
		Vec3f::new ( 
			self.x + other.x, 
			self.y + other.y,
			self.z + other.z
		)
	}
}

// impl Sub<f64> for Vec3f {
// 	type Output = Vec3f;

// }

impl Sub for Vec3f {
	type Output = Vec3f;
	fn sub(self, other: Vec3f) -> Vec3f {
		self + (other * -1.)
	}
}


impl Add for Vec3f {
	type Output = Vec3f;
	fn add(self, other: Vec3f) -> Vec3f {
		Vec3f::new ( 
			self.x + other.x, 
			self.y + other.y,
			self.z + other.z
		)
	}
}

impl AddAssign for Vec3f {
	fn add_assign(&mut self, other: Self) {
		*self = Self {
			x: self.x + other.x,
			y: self.x + other.y,
			z: self.z + other.z
		};
	}
}

impl Mul for Vec3f {
	type Output = Vec3f;
	fn mul (self, other: Vec3f) -> Vec3f {
		self.cross(&other)
	}
}

impl Mul<Vec3f> for f64 {
	type Output = Vec3f;
	fn mul (self, other: Vec3f) -> Vec3f {
		other * self
	}
}

impl Index<usize> for Vec3f {
	type Output = f64;
	fn index(&self, idx: usize) -> &f64
	{
		match idx {
			0 => {
				&self.x
			}
			1 => {
				&self.y
			} 
			2 => {
				&self.z
			} 
			_ => {
				panic!("Index out of bounds!");
			}
		}
	}
}

impl Mul<f64> for Vec3f {
	type Output = Vec3f;
	fn mul(self, scalar: f64) -> Vec3f {
		Vec3f::new(
			self.x * scalar,
			self.y * scalar,
			self.z * scalar
		)
	}
}

#[allow(non_snake_case)]
pub fn reflect(J: Vec3f, N: Vec3f) -> Vec3f {
    J - N * 2. * J.dot(&N)
}