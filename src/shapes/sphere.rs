use crate::core;
use crate::math;

use super::Intersection;

pub struct Sphere {
	pub center: core::Point,
	pub radius: f32,
	pub color: core::Color
}

impl Intersection for Sphere {
	fn is_intersect(&self, ray: &core::Ray) -> (bool, f32) {
		let c = &self.center;
		let r = &self.radius;
		let vect = core::Point(ray.origin.0 - c.0, ray.origin.1 - c.1, ray.origin.2 - c.2);

		let k1 = math::dot(ray.direction, ray.direction);
		let k2 = 2.0 * math::dot(&vect, ray.direction);
		let k3 = math::dot(&vect, &vect) - r * r;

		let d: f32 = k2 * k2 - 4.0 * k1 * k3;

		if d < 0.0 {
			return (false, 0.0);
		}

		let c1: f32 = (-k2 + d.sqrt()) / (2.0 * k1);
		let c2: f32 = (-k2 - d.sqrt()) / (2.0 * k1);

		if (c1 < ray.origin.2) && (c2 < ray.origin.2) {
			return (false, 0.0);
		}

		if (c1 < c2) && (c1 >= ray.origin.2) {
			return (true, c1);
		}

		if c2 >= ray.origin.2 {
			return (true, c2);
		}

		(false, 0.0)
	}

	fn get_color(&self) -> &core::Color {
		&self.color
	}

	fn get_normal(&self, intersect: &core::Point) -> core::Point {
		let normal = math::subtract(&intersect, &self.center);
		math::divide(&normal, math::length(&normal))
	}
}