use vector::Vector;
use ray::Ray;

pub struct Camera {
    origin: Vector,
    viewport_origin: Vector,
    viewport_height: Vector,
    viewport_width: Vector
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Vector::new(0.0, 0.0, 0.0),
            viewport_origin: Vector::new(-2.0, -1.0, -1.0),
            viewport_height: Vector::new(0.0, 2.0, 0.0),
            viewport_width: Vector::new(4.0, 0.0, 0.0)
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.viewport_origin + self.viewport_width*u + self.viewport_height*v)
    }
}