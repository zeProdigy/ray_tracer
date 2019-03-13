use crate::core;
use crate::math;

use super::Intersection;

pub struct Sphere {
    pub center: core::Point,
    pub radius: f32,
    pub color: core::Color,
    pub reflection: i32,
    pub specular: f32
}

impl Intersection for Sphere {
    fn is_intersect(&self, ray: &core::Ray, tmin: f32, tmax: f32) -> (bool, f32) {
        let c = &self.center;
        let r = &self.radius;
        let oc = math::subtract(ray.origin, &c);

        let k1 = math::dot(ray.direction, ray.direction);
        let k2 = 2.0 * math::dot(&oc, ray.direction);
        let k3 = math::dot(&oc, &oc) - r * r;

        let d: f32 = k2 * k2 - 4.0 * k1 * k3;

        if d < 0.0 {
            return (false, 0.0);
        }

        let t1: f32 = (-k2 + d.sqrt()) / (2.0 * k1);
        let t2: f32 = (-k2 - d.sqrt()) / (2.0 * k1);

        if (t2 > tmin) && (t2 < tmax) {
            return (true, t2);
        }

        if (t1 > tmin) && (t1 < tmax) {
            return (true, t1);
        }

        (false, 0.0)
    }

    fn get_normal(&self, intersect: &core::Point) -> core::Point {
        let normal = math::subtract(&intersect, &self.center);
        math::divide(&normal, math::length(&normal))
    }

    fn get_color(&self) -> &core::Color {
        &self.color
    }

    fn get_reflection_rate(&self) -> i32 {
        self.reflection
    }

    fn get_specular_rate(&self) -> f32 {
        self.specular
    }
}
