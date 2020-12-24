//! This module contains super straightforward implementations
//! for whatever we need from vectorial math.
//! The entire implementation is simple, without any clever
//! optimizations (although it did manage to beat Nalgebra in speed, somehow)

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
/// Simple three-dimensional euclidian vector
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

	/// Dot product
	pub fn dot(& self, other: &Vec3f) -> f64 {
		self.x * other.x +
		self.y * other.y +
		self.z * other.z
	}

	/// Cross product
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

// TODO: I'm pretty sure this is wrong
#[cfg(feature = "rotate_camera")]
pub fn rotate_camera(origin: Vec3f, dir: Vec3f, target: Vec3f)  -> Vec3f {
   let z_axis  = (origin - target).normalize();
   let x_axis  = Vec3f::new(0., 1., 0.).cross(&z_axis);
   let y_axis  = z_axis.cross(&x_axis).normalize();

   let transformation_matrix: Mat4f = [
   		Vec4f::new( x_axis.x, x_axis.y, x_axis.z, 0. ),
   		Vec4f::new( y_axis.x, y_axis.y, y_axis.z, 0. ),
   		Vec4f::new( z_axis.x, z_axis.y, z_axis.z, 0. ),
   		Vec4f::new( origin.x, origin.y, origin.z, 1. )
   	];

   let new_dir = transformation_matrix *
   				 Vec4f::new(dir.x, dir.y, dir.z, 0.);

   Vec3f::new(new_dir.x, new_dir.y, new_dir.z)
}

#[cfg(feature = "rotate_camera")]
#[derive(Debug)]
pub struct Vec4f {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub w: f64	
}

#[cfg(feature = "rotate_camera")]
impl Vec4f {
	pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4f
	{
		Vec4f {
			x,
			y,
			z,
			w
		}
	}
}

#[cfg(feature = "rotate_camera")]
pub type Mat4f = [Vec4f; 4];

/// Multiplication of a 4x4 matrix with a 4x1 vector.
#[cfg(feature = "rotate_camera")]
impl Mul<Vec4f> for Mat4f {
	type Output = Vec4f;
	fn mul (self, other: Vec4f) -> Vec4f {
		let mut res = [0.0; 4];
		let mut idx = 0;
		for row in &self {
			res[idx] += row.x * other.x + row.y * other.y + row.z * other.z + row.w * other.w;
			idx += 1;
		}
		Vec4f::new(res[0], res[1], res[2], res[3])
	}	
}