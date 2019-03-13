extern crate image;

use image::{Rgb, ImageBuffer};

use std::io;
use std::io::Write;
use std::f32;

use crate::shapes::Intersection;

mod core;
mod math;
mod shapes;
mod lighting;

/*
       -Y   +Z
    |   /
    |  /
    | /
    |/
    *------ +X
*/

const IMAGE_SIZE: (u32, u32) = (500, 500);
const VIEW_POINT: core::Point = core::Point(0.0, 0.0, 0.0);
const VIEWPORT: (f32, f32) = (1.0, 1.0);
const VIEWPORT_DISTANCE: f32 = 1.0;

const SUBPIXELS_NUM: u32 = 4;
const JITTER_MATRIX: [(f32, f32); 4] = [
    (-1.0/4.0,  3.0/4.0),
    (3.0/4.0,  1.0/3.0),
    (-3.0/4.0, -1.0/4.0),
    (1.0/4.0, -3.0/4.0)
];

const REFLECTION_RECURSION_DEPTH: u8 = 4;

fn main() {
    let orange_sphere = shapes::sphere::Sphere {
        center: core::Point(-0.5, 0.5, 6.0),
        radius: 0.5,
        color:  core::Color(0xff, 0xa5, 0x00),
        reflection: 20,
        specular: 0.0
    };

    let cyan_sphere = shapes::sphere::Sphere {
        center: core::Point(-2.0, -0.5, 8.0),
        radius: 1.5,
        color:  core::Color(0, 0xce, 0xd1),
        reflection: 1,
        specular: 0.80
    };

    let coral_sphere = shapes::sphere::Sphere {
        center: core::Point(1.5, -1.0, 8.0),
        radius: 2.0,
        color:  core::Color(0xf0, 0x80, 0x80),
        reflection: 20,
        specular: 0.0
    };

    let floor = shapes::plane::Plane {
        center: core::Point(0.0, 1.0, 0.0),
        normal: core::Point(0.0, -1.0, 0.0),
        color:  core::Color(255, 255, 255),
        reflection: 20,
        specular: 0.0
    };

    let light1 = lighting::Light::Ambient {
        intensity: 0.05
    };

    let light2 = lighting::Light::Point {
        intensity: 0.35,
        position:  core::Point(-5.0, -5.0, -10.0)
    };

    let light3 = lighting::Light::Directional {
        intensity: 0.6,
        direction: core::Point(5.0, -10.0, 10.0)
    };

    let background = shapes::background::Background {
        color: core::Color(0, 0, 0),
        reflection: 0,
        specular: 0.0
    };

    // NOTE! Background должен быть первым в векторе.
    let scene: Vec<&Intersection> = vec![&background, &orange_sphere, &cyan_sphere, &coral_sphere, &floor];
    let light_sources: Vec<&lighting::Light> = vec![&light1, &light2, &light3];

    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(IMAGE_SIZE.0, IMAGE_SIZE.1);

    let one_percent = IMAGE_SIZE.0 * IMAGE_SIZE.1 / 100;
    let mut progress = 0;
    let mut processed = 0;

    for x in 0..IMAGE_SIZE.0 {
        for y in 0..IMAGE_SIZE.1 {
            let mut color_sum: (u32, u32, u32) = (0, 0, 0);

            for subpixel in 0..SUBPIXELS_NUM as usize {
                let x = x as f32 + JITTER_MATRIX[subpixel].0;
                let y = y as f32 + JITTER_MATRIX[subpixel].1;
                let viewport_pixel = scene_to_viewport(x, y);

                let ray = core::Ray {
                    origin: &VIEW_POINT,
                    direction: &viewport_pixel
                };

                let color = trace_ray(&scene, &light_sources, &ray, REFLECTION_RECURSION_DEPTH);

                color_sum.0 += color.0 as u32;
                color_sum.1 += color.1 as u32;
                color_sum.2 += color.2 as u32;
            }

            let pixel_color = average_color(color_sum);

            img.get_pixel_mut(x , y).data =
                [pixel_color.0, pixel_color.1, pixel_color.2];

            processed += 1;

            if (processed % one_percent) == 0 {
                progress += 1;
                print!("\rComputation {}%", progress);
                io::stdout().flush().unwrap();
            }
        }
    }

    img.save("out.png").unwrap();
}

fn scene_to_viewport(x: f32, y: f32) -> core::Point {
    let x: f32 = x - IMAGE_SIZE.0 as f32 / 2.0;
    let y: f32 = y - IMAGE_SIZE.1 as f32 / 2.0;

    core::Point(x * VIEWPORT.0 / IMAGE_SIZE.0 as f32,
        y * VIEWPORT.1 / IMAGE_SIZE.1 as f32, VIEWPORT_DISTANCE)
}

fn update_color(color: &core::Color, light_intensity: f32) -> core::Color {
    fn update_channel(ch: u8, light_intensity: f32) -> u8 {
        let res = ch as f32 * light_intensity;
        if res > 255.0 {
            return 255;
        }

        res as u8
    }

    core::Color(update_channel(color.0, light_intensity),
                update_channel(color.1, light_intensity),
                update_channel(color.2, light_intensity))
}

fn average_color(color_sum: (u32, u32, u32)) -> core::Color {
    fn convert(x: u32) -> u8 {
        if x > 255 {
            return 255;
        }

        x as u8
    }

    let r: u8 = convert(color_sum.0 / SUBPIXELS_NUM);
    let g: u8 = convert(color_sum.1 / SUBPIXELS_NUM);
    let b: u8 = convert(color_sum.2 / SUBPIXELS_NUM);

    core::Color(r, g, b)
}

fn trace_ray(scene: &Vec<&Intersection>,
             light_sources: &Vec<&lighting::Light>,
             ray: &core::Ray,
             recursion_depth: u8) -> core::Color {
    let mut closest_shape: (&Intersection, f32) = (scene[0], f32::INFINITY);
    for &shape in scene.iter() {
        let intersect = shape.is_intersect(&ray, VIEWPORT_DISTANCE, f32::INFINITY);
        if (intersect.0 == true) && (intersect.1 < closest_shape.1) {
            closest_shape.0 = shape;
            closest_shape.1 = intersect.1;
        }
    }

    let light_intensity = lighting::compute_lighting(&scene, &light_sources, &ray, &closest_shape);
    let color = closest_shape.0.get_color();
    let color = update_color(color, light_intensity);

    let specular = closest_shape.0.get_specular_rate();
    if (specular == 0.0) || (recursion_depth == 0) {
        return color;
    }

    // TODO: Повторяющийся код из ::compute_lighting
    let intersect = math::add(&ray.origin, &math::multiply(&ray.direction, closest_shape.1));
    let normal = closest_shape.0.get_normal(&intersect);

    let inverse_direction = core::Point(-ray.direction.0, -ray.direction.1, -ray.direction.2);
    let reflect_ray = math::dot(&normal, &inverse_direction);
    let reflect_ray = math::multiply(&normal, 2.0 * reflect_ray);
    let reflect_ray = math::subtract(&reflect_ray, &inverse_direction);

    let reflect_ray = core::Ray {
        origin: &intersect,
        direction: &reflect_ray
    };

    let reflected_color = trace_ray(scene, light_sources, &reflect_ray, recursion_depth - 1);

    core::Color((color.0 as f32 * (1.0 - specular) + reflected_color.0 as f32 * specular) as u8,
                (color.1 as f32 * (1.0 - specular) + reflected_color.1 as f32 * specular) as u8,
                (color.2 as f32 * (1.0 - specular) + reflected_color.2 as f32 * specular) as u8)
}
