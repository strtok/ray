use core::fmt;
use ray::Ray;
use ray::RayIntersect;
use ray::IntersectionResult;
use std::f32;
use vector::*;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Vector, radius: f32) -> Sphere {
        Sphere{center, radius}
    }
}

impl RayIntersect for Sphere {
    fn intersects(&self, ray: &Ray, bounds: (f32, f32)) -> Option<IntersectionResult> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius*self.radius;
        let d = b*b - a*c;

        let (tmin, tmax) = bounds;

        if d < 0.0 {
            return None;
        }

        let mut t = (-b - (b*b - a*c).sqrt()) / a;
        if t > tmin && t < tmax {
            return Some(IntersectionResult {
                t,
                point: ray.point_at(t),
                normal: (ray.point_at(t) - self.center) / self.radius
            })
        }

        t = (-b + (b*b - a*c).sqrt()) / a;
        if t > tmin && t < tmax {
            return Some(IntersectionResult {
                t,
                point: ray.point_at(t),
                normal: (ray.point_at(t) - self.center) / self.radius
            })
        }

        return None;
    }
}

impl fmt::Debug for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sphere {{ center: {:?}, radius: {:?}}}", self.center, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use sphere::*;
    use vector::*;
    use ray::*;

    #[test]
    fn intersection_with_ray() {
        let s = Sphere::new(Vector::new(0.0,0.0,-1.0), 0.5);
        let r = Ray::new(Vector::new(0.0,0.0,0.0),
                            Vector::new(0.0,0.0,-1.0));
        assert!(r.intersects(&s, (0.001, f32::MAX)).is_some());
    }
}