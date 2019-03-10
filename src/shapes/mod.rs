use crate::core;

pub mod sphere;
pub mod plane;

pub trait Intersection {
	fn is_intersect(&self, ray: &core::Ray) -> (bool, f32);
	fn get_normal(&self, intersect: &core::Point) -> core::Point;
	fn get_color(&self) -> &core::Color;
	fn get_reflection_rate(&self) -> i32;
}
