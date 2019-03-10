use crate::core;

pub mod sphere;
pub mod plane;
pub mod background;

pub trait Intersection {
    fn is_intersect(&self, ray: &core::Ray, tmin: f32, tmax: f32) -> (bool, f32);
    fn get_normal(&self, intersect: &core::Point) -> core::Point;
    fn get_color(&self) -> &core::Color;
    fn get_reflection_rate(&self) -> i32;
}
