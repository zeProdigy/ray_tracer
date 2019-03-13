use crate::core;
use super::Intersection;

pub struct Background {
    pub color: core::Color,
    pub reflection: i32,
    pub specular: f32
}

impl Intersection for Background {
    fn is_intersect(&self, _ray: &core::Ray, _tmin: f32, _tmax: f32) -> (bool, f32) {
        (false, 0.0)
    }

    fn get_color(&self) -> &core::Color {
        &self.color
    }

    fn get_normal(&self, _intersect: &core::Point) -> core::Point {
        core::Point(0.0, 0.0, 0.0)
    }

    fn get_reflection_rate(&self) -> i32 {
        self.reflection
    }

    fn get_specular_rate(&self) -> f32 {
        self.specular
    }
}
