use sphere::Sphere;

pub enum Object {
    Sphere(Sphere)
}

pub struct Scene {
    pub objects: Vec<Object>
}
