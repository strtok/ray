use core::fmt;
use ray::Ray;
use ray::RayIntersect;
use ray::IntersectionResult;
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
    fn intersects(&self, ray: &Ray) -> Option<IntersectionResult> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * (oc.dot(&ray.direction));
        let c = oc.dot(&oc) - self.radius*self.radius;
        let d = b*b - 4.0*a*c;

        if d > 0.0 {
            let t = (-b - d.sqrt()) / (2.0*a);
            let normal = (ray.point_at(t) - self.center) / self.radius;
            Some(IntersectionResult{t, normal})
        } else {
            None
        }
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
        assert!(r.intersects(&s));
    }
}