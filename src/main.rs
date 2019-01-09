extern crate image;

use image::{Rgb, ImageBuffer};

use std::io;
use std::io::Write;
use std::f32;

struct Coord(f32, f32, f32);

enum Light {
	Ambient {intensity: f32},
	Point {intensity: f32, position: Coord},
	Directional {intensity: f32, direction: Coord}
}

struct Sphere {
	centre: Coord,
	radius: f32,
	color: (u8, u8, u8)
}

struct Cone {
	base: Coord,
	radius: f32,
	height: f32,
	color: (u8, u8, u8)
}

struct Background {
	color: (u8, u8,u8)
}

trait Intersection {
	fn is_intersect(&self, viewpoint: &Coord, viewport_pixel: &Coord) -> (bool, f32);
	fn get_normal(&self, intersect: &Coord) -> Coord;
	fn get_color(&self) -> (u8, u8, u8);
}

impl Intersection for Sphere {
	fn is_intersect(&self, viewpoint: &Coord, viewport_pixel: &Coord) -> (bool, f32) {
		let c = &self.centre;
		let r = &self.radius;
		let vect = Coord(viewpoint.0 - c.0, viewpoint.1 - c.1, viewpoint.2 - c.2);

		let k1 = scalar_product(&viewport_pixel, &viewport_pixel);
		let k2 = 2.0 * scalar_product(&vect, &viewport_pixel);
		let k3 = scalar_product(&vect, &vect) - r * r;

		let d: f32 = k2 * k2 - 4.0 * k1 * k3;

		if d < 0.0 {
			return (false, 0.0);  
		}

		let c1: f32 = (-k2 + d.sqrt()) / (2.0 * k1);
		let c2: f32 = (-k2 - d.sqrt()) / (2.0 * k1);

		if (c1 < viewpoint.2) && (c2 < viewpoint.2) {
			return (false, 0.0);
		}

		if (c1 < c2) && (c1 >= viewpoint.2) {
			return (true, c1);
		}

		if c2 >= viewpoint.2 {
			return (true, c2);
		}

		(false, 0.0)
	}

	fn get_color(&self) -> (u8, u8, u8) {
		self.color
	}

	fn get_normal(&self, intersect: &Coord) -> Coord {
		let normal = subtract(&intersect, &self.centre);
		division(&normal, length(&normal))
	}
}

impl Intersection for Background {
	fn is_intersect(&self, viewpoint: &Coord, viewport_pixel: &Coord) -> (bool, f32) {
		(false, 0.0)
	}
	fn get_color(&self) -> (u8, u8, u8) {
		self.color
	}
	fn get_normal(&self, intersect: &Coord) -> Coord {
		Coord(0.0, 0.0, 0.0)
	}
}

// impl Intersection for Cone {
// 	fn is_intersect(&self) -> bool {
// 		println!("I am cone! Base coord: x = {}, y = {}, z = {}",
// 			self.base.0, self.base.1, self.base.2);
// 		true
// 	}
// }

fn compute_lighting(sources: &Vec<&Light>, intersect: &Coord, normal: &Coord) -> f32 {
	let mut sum = 0.0;

	for &light in sources.iter() {
		match light {
			Light::Ambient{intensity} => {
				sum += *intensity;
			}

			Light::Point{intensity, position} => {
				let light_dir = subtract(position, &intersect);
				let dot = scalar_product(&normal, &light_dir);
				if dot > 0.0 {
					sum += *intensity * dot / (length(&normal) * length(&light_dir));
				}
			}

			Light::Directional{intensity, direction} => {
				let light_dir = direction;
				let dot = scalar_product(&normal, &light_dir);
				if dot > 0.0 {
					sum += *intensity * dot / (length(&normal) * length(&light_dir));
				}
			}
		}
	}

	sum
}

const IMAGE_SIZE: (u32, u32) = (500, 500);
const VIEW_POINT: Coord = Coord(0.0, 0.0, 0.0);
const VIEWPORT: (f32, f32) = (1.0, 1.0);
const VIEWPORT_DISTANCE: f32 = 1.0;

fn main() {
	let red_sphere = Sphere {
		centre: Coord(0.0, 0.0, 4.0),
		radius: 1.0,
		color: (255, 0, 0)
	};

	let green_sphere = Sphere {
		centre: Coord(-2.0, -1.0, 8.0),
		radius: 1.5,
		color: (0, 255, 0)
	};

	let blue_sphere = Sphere {
		centre: Coord(2.0, -1.0, 8.0),
		radius: 1.5,
		color: (0, 0, 255)
	};

	let light1 = Light::Ambient{intensity: 0.2};
	let light2 = Light::Point{intensity: 0.6, position: Coord(0.0, 0.0, 0.0)};
	let light3 = Light::Directional{intensity: 0.2, direction: Coord(1.0, 4.0, 4.0)}; 

	let lighting: Vec<&Light> = vec![&light1, &light2, &light3];

	let scene: Vec<&Intersection> = vec![&red_sphere, &green_sphere, &blue_sphere];
	let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(IMAGE_SIZE.0, IMAGE_SIZE.1);

	let mut progress = 0;
	let mut processed = 0;
	let one_percent = IMAGE_SIZE.0 * IMAGE_SIZE.1 / 100;

	for x in -(IMAGE_SIZE.0 as i32 / 2)..IMAGE_SIZE.0 as i32 / 2 {
		for y in -(IMAGE_SIZE.1 as i32 /2)..IMAGE_SIZE.1 as i32 / 2 {
			let scene_pixel = scene_to_viewport(x, y);
			let background = Background{color: (0, 0, 0)};
			let mut closest_shape: (&Intersection, f32) = (&background, f32::INFINITY);

			for &shape in scene.iter() {
				let intersect = shape.is_intersect(&VIEW_POINT, &scene_pixel);
				if (intersect.0 == true) && (intersect.1 < closest_shape.1) {
					closest_shape.0 = shape;
					closest_shape.1 = intersect.1;
				}
			}

			let p = add(&VIEW_POINT, &multiply(&scene_pixel, closest_shape.1));
			let n = closest_shape.0.get_normal(&p);
			let light_intensity = compute_lighting(&lighting, &p, &n);
			let color = closest_shape.0.get_color();
			let color = ((color.0 as f32 * light_intensity) as u8,
						 (color.1 as f32 * light_intensity) as u8,
						 (color.2 as f32 * light_intensity) as u8);

			img.get_pixel_mut((x + IMAGE_SIZE.0 as i32 / 2) as u32, (y + IMAGE_SIZE.1 as i32 / 2) as u32).data =
				[color.0, color.1, color.2];

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

fn scene_to_viewport(x: i32, y:i32) -> Coord {
	Coord(x as f32 * VIEWPORT.0 / IMAGE_SIZE.0 as f32,
		y as f32 * VIEWPORT.1 / IMAGE_SIZE.1 as f32, VIEWPORT_DISTANCE)
}

fn scalar_product(c1: &Coord, c2: &Coord) -> f32 {
	c1.0 * c2.0 + c1.1 * c2.1 + c1.2 * c2.2
}

fn subtract(c1: &Coord, c2: &Coord) -> Coord {
	Coord(c1.0 - c2.0, c1.1 - c2.1, c1.2 - c2.2)
}

fn add(c1: &Coord, c2: &Coord) -> Coord {
	Coord(c1.0 + c2.0, c1.1 + c2.1, c1.2 + c2.2)
}

fn multiply(c1: &Coord, k: f32) -> Coord {
	Coord(c1.0 * k, c1.1 * k, c1.2 * k)
}

fn division(c1: &Coord, k: f32) -> Coord {
	Coord(c1.0 / k, c1.1 / k, c1.2 / k)
}

fn length(c: &Coord) -> f32 {
	scalar_product(c, c).sqrt()
}
