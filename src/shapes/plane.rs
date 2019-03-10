use crate::core;
use crate::math;
use super::Intersection;

pub struct Plane {
	pub center: core::Point,
	pub normal: core::Point,
	pub color: core::Color,
	pub reflection: i32
}

impl Intersection for Plane {
	fn is_intersect(&self, ray: &core::Ray) -> (bool, f32) {
		let denominator = math::dot(&self.normal, ray.direction);

		if denominator.abs() > 0.0001 {
			let diff = math::subtract(&self.center, ray.origin);
			let t = math::dot(&diff, &self.normal) / denominator;

			if t > 0.0001 {
				return (true, t) // NOTE! нужно возвращать t?
			}
		}

		(false, 0.0)
	}

	fn get_color(&self) -> &core::Color {
		&self.color
	}

	fn get_normal(&self, _intersect: &core::Point) -> core::Point {
		core::Point(self.normal.0, self.normal.1, self.normal.2)
	}

    fn get_reflection_rate(&self) -> i32 {
		self.reflection
	}
}
