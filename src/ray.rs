use core::fmt;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vector,
    direction: Vector
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray{origin, direction}
    }

    pub fn point_at(t: f32) -> Vector {
        return A + B*t;
    }
}
