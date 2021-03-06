use core::fmt;
use vector::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray{origin, direction}
    }
    pub fn point_at(&self, t: f32) -> Vector {
        return self.origin + self.direction*t;
    }

    pub fn intersects<T>(&self, obj: &T, bounds: (f32, f32)) -> Option<IntersectionResult>
        where T: RayIntersect
    {
        return obj.intersects(&self, bounds);
    }
}

impl fmt::Debug for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ray {{ origin: {:?}, direction: {:?}}}", self.origin, self.direction)
    }
}

pub struct IntersectionResult {
    pub t: f32,
    pub point: Vector,
    pub normal: Vector
}

pub trait RayIntersect {
    fn intersects(&self, ray: &Ray, bounds: (f32, f32)) -> Option<IntersectionResult>;
}