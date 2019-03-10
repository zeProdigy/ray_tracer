use crate::core;
use crate::math;
use crate::shapes;

pub enum Light {
	Ambient {intensity: f32},
	Point {intensity: f32, position: core::Point},
	Directional {intensity: f32, direction: core::Point}
}

pub fn compute_lighting(sources: &Vec<&Light>, ray: &core::Ray, shape: &(&shapes::Intersection, f32)) -> f32 {
	let mut sum = 0.0;
	let intersect = math::add(&ray.origin, &math::multiply(&ray.direction, shape.1));
	let normal = shape.0.get_normal(&intersect);

	for &light in sources.iter() {
		match light {
			Light::Ambient{intensity} => {
				sum += *intensity;
			}

			Light::Point{intensity, position} => {
				let light_dir = math::subtract(position, &intersect);
				sum += *intensity * diffuse_light(&light_dir, &normal);
				sum += *intensity * reflection_light(&light_dir, &normal, ray, shape);
			}

			Light::Directional{intensity, direction} => {
				let light_dir = direction;
				sum += *intensity * diffuse_light(&light_dir, &normal);
				sum += *intensity * reflection_light(&light_dir, &normal, ray, shape);
			}
		}
	}

	sum
}

fn diffuse_light(light_dir: &core::Point,
                 normal: &core::Point) -> f32 {
	let dot = math::dot(&normal, &light_dir);
	if dot > 0.0 {
		return dot / (math::length(&normal) * math::length(&light_dir));
	}

	0.0
}

fn reflection_light(light_dir: &core::Point,
				    normal: &core::Point,
                    ray: &core::Ray,
					shape: &(&shapes::Intersection, f32)) -> f32 {
	let reflection_rate = shape.0.get_reflection_rate();
	if reflection_rate != -1 {
		let opposite_ray = core::Point(-ray.direction.0, -ray.direction.1, -ray.direction.2);
		let r = math::multiply(&normal, 2.0);
		let r = math::multiply(&r, math::dot(&normal, &light_dir));
		let r = math::subtract(&r, &light_dir);
		let r_dot = math::dot(&r, &opposite_ray);
		if r_dot > 0.0 {
			return (r_dot/(math::length(&r) * math::length(&opposite_ray))).powi(reflection_rate);
		}
	}

	0.0
}
