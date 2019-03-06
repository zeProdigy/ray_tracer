pub struct Point(pub f32, pub f32, pub f32);
pub struct Color(pub u8, pub u8, pub u8);

pub struct Ray<'a> {
    pub origin: &'a Point,
    pub direction: &'a Point
}
