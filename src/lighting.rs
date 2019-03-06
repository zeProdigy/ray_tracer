use crate::core;
use crate::math;

pub enum Light {
	Ambient {intensity: f32},
	Point {intensity: f32, position: core::Point},
	Directional {intensity: f32, direction: core::Point}
}

pub fn compute_lighting(sources: &Vec<&Light>, intersect: &core::Point, normal: &core::Point) -> f32 {
	let mut sum = 0.0;

	for &light in sources.iter() {
		match light {
			Light::Ambient{intensity} => {
				sum += *intensity;
			}

			Light::Point{intensity, position} => {
				let light_dir = math::subtract(position, &intersect);
				let dot = math::dot(&normal, &light_dir);
				if dot > 0.0 {
					sum += *intensity * dot / (math::length(&normal) * math::length(&light_dir));
				}
			}

			Light::Directional{intensity, direction} => {
				let light_dir = direction;
				let dot = math::dot(&normal, &light_dir);
				if dot > 0.0 {
					sum += *intensity * dot / (math::length(&normal) * math::length(&light_dir));
				}
			}
		}
	}

	sum
}