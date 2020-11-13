use crate::vector::{self, Vec3f};
use crate::light::Light;
use crate::material::Material;

use std::vec::Vec;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f
}


impl Ray {
    pub fn new (origin: Vec3f, dir: Vec3f) -> Ray {
        Ray {
            origin,
            direction: dir
        }
	}
	
	#[allow(non_snake_case)]
	pub fn scene_intersections(self, spheres: &mut Vec<Sphere>, hit: &mut Vec3f, N: &mut Vec3f, material: &mut Material) -> bool {
		let mut spheres_dist = f64::MAX;
		for sphere in spheres {
			let intersection = sphere.intersects_with_ray(self);
			if let Some(distance) = intersection {
				if distance < spheres_dist {
					spheres_dist = distance;
					*hit = self.origin + self.direction*distance;
					*N = (*hit - sphere.center).normalize();
					*material = sphere.material;
				}
			}
		}

		let mut checkerboard_dist = f64::MAX;


		if f64::abs(self.direction[1]) > 0.001 {
			let d = -(self.origin[1]+4.)/self.direction[1];
			let pt = self.origin + self.direction * d;
			if d > 0. && f64::abs(pt[0]) < 10. && pt[2] > -30. && d < spheres_dist {
				checkerboard_dist = d;
				*hit = pt;
				*N = Vec3f::new(0., 1., 0.);
				let val_a = (0.5*(hit[0])+1000.) as i32;
				let val_b = (0.5 * hit[2]) as i32;
				material.diffuse_color = if (val_a + val_b & 1) > 0 {
					Vec3f::new(0.3, 0.3, 0.3)
				} else {
					Vec3f::new(0.3, 0.2, 0.1)
				}
			}
		}

		f64::min(spheres_dist, checkerboard_dist) < 1000.
	}

	#[allow(non_snake_case)]
    pub fn cast (self, spheres: &mut Vec<Sphere>, lights: &mut Vec<Light>, depth: usize) -> Vec3f {
		let mut point = Vec3f::new(0., 0., 0.);
		let mut N = Vec3f::new(0., 0., 0.);
		let mut material = Material::new(
			Vec3f::new(0., 0., 0.),
		   Vec3f::new(1., 0., 0.),
		   0.
		);
        if depth > 4 || !self.scene_intersections(spheres, &mut point, &mut N, &mut material) {
			return Vec3f::new(0.6, 0.4, 0.4);
		}

		let reflect_dir = vector::reflect(self.direction, N);
		let reflect_orig = if reflect_dir.dot(&N) < 0. {
			point - N * 1e-3
		} else {
			point + N * 1e-3
		};

		let reflection = Ray::new (
			reflect_orig,
			reflect_dir
		);

		let reflection_color = reflection.cast(spheres, lights, depth + 1);
		

		let mut diffuse_light_intensity = 0.;
		let mut specular_light_intensity = 0.;
		for light in lights {
			let light_vec = light.position - point;
			let light_dir = light_vec.normalize();
			let light_distance: f64 = light_vec.norm();

			let shadow_origin = if light_dir.dot(&N) < 0. {
				point - N * 0.001
			} else {
				point + N * 0.001	
			};
			let mut shadow_point = Vec3f::new(0., 0., 0.);
			let mut shadow_N = Vec3f::new(0., 0., 0.);
			let mut unused_material = Material::new(
				Vec3f::new(0., 0., 0.),
				Vec3f::new(1., 0., 0.),
				0.
			);

			let shadow_ray = Ray::new(
				shadow_origin,
				light_dir
			);

			if shadow_ray.scene_intersections(spheres, &mut shadow_point, &mut shadow_N, &mut unused_material) && (shadow_point-shadow_origin).norm() < light_distance {	
				continue;
			}

			diffuse_light_intensity  += light.intensity * f64::max(0., light_dir.dot(&N));
			specular_light_intensity += f64::powf(
				f64::max(0., vector::reflect(light_dir, N).dot(&self.direction)), 
				   material.specular_exponent) * light.intensity;
		}

		material.albedo[0] * diffuse_light_intensity * material.diffuse_color +
		material.albedo[1] * Vec3f::new(1., 1., 1.) * specular_light_intensity +
		material.albedo[2] * reflection_color
	}
	
}

/// 
#[derive(Copy, Clone)]
pub struct Sphere {
	center: Vec3f,
	radius: f64,
	material: Material
}


impl Sphere {
	pub fn new(center: Vec3f, radius: f64, material: Material) -> Sphere {
		Sphere {
			center,
			radius,
			material
		}
	}

	/// intersects_with_ray checks if the ray originating from `origin`, in the direction of `dir`
	/// intersects with the sphere.
	#[allow(non_snake_case)]
	pub fn intersects_with_ray (&mut self, ray: Ray) -> Option<f64>
	{
		// Create a line segment between the ray origin and the center of the sphere
		let L: Vec3f = self.center - ray.origin;
		//Use l as a hypotenuse and find the length of the adjacent side
		let adj = L.dot(&ray.direction);
		//Find the length-squared of the opposite side
		//This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
		let d2 = L.dot(&L) - (adj * adj);
		let radius_sqr = self.radius * self.radius;
		//If that length-squared is less than radius squared, the ray intersects the sphere
		if d2 > radius_sqr {
			return None;
		}
		let thc = f64::sqrt(radius_sqr - d2);
		
		match (adj - thc, adj + thc) {
			(x, y) if x < 0. && y < 0. => {
				None
			},
			(x, y) if x < y => {
				Some(x)
			}
			(_, y) => {
				Some(y)
			}
		}
	}
}