use core::fmt;
use vector::*;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vector,
    direction: Vector
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray{origin, direction}
    }
    pub fn point_at(&self, t: f32) -> Vector {
        return self.origin + self.direction*t;
    }
}

impl fmt::Debug for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ray {{ origin: {:?}, direction: {:?}}}", self.origin, self.direction)
    }
}
