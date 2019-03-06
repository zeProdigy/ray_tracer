use crate::core;

pub fn dot(c1: &core::Point, c2: &core::Point) -> f32 {
	c1.0 * c2.0 + c1.1 * c2.1 + c1.2 * c2.2
}

pub fn subtract(c1: &core::Point, c2: &core::Point) -> core::Point {
	core::Point(c1.0 - c2.0, c1.1 - c2.1, c1.2 - c2.2)
}

pub fn add(c1: &core::Point, c2: &core::Point) -> core::Point {
	core::Point(c1.0 + c2.0, c1.1 + c2.1, c1.2 + c2.2)
}

pub fn multiply(c1: &core::Point, k: f32) -> core::Point {
	core::Point(c1.0 * k, c1.1 * k, c1.2 * k)
}

pub fn divide(c1: &core::Point, k: f32) -> core::Point {
	core::Point(c1.0 / k, c1.1 / k, c1.2 / k)
}

pub fn length(c: &core::Point) -> f32 {
	dot(c, c).sqrt()
}
