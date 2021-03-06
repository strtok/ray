use core::fmt;
use rand::prelude::*;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector{x, y, z }
    }

    pub fn random_unit() -> Vector {
        loop {
            let v = Vector::new(random::<f32>(), random::<f32>(), random::<f32>())*2.0 - Vector::new(1.0, 1.0, 1.0);
            if v.squared_length() >= 1.0 {
                return v;
            }
        }
    }

    pub fn unit(&self) -> Vector {
        let k = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector::new(k*self.x, k*self.y, k*self.z)
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    #[allow(dead_code)]
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(self.y*other.z - self.z*other.y,
                    self.z*other.x - self.x*other.z,
                    self.x*other.y - self.y*other.x)
    }

    #[allow(dead_code)]
    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector {{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}


impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<f32> for Vector {
    type Output = Vector;
    fn add(self, other: f32) -> Vector {
        Vector {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}


impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}


impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Div for Vector {
    type Output = Vector;
    fn div(self, other: Vector) -> Vector {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}


impl Div <f32>for Vector {
    type Output = Vector;
    fn div(self, other: f32) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

#[cfg(test)]
mod tests {
    use vector::*;
    #[test]
    fn length() {
        let v = Vector::new(1.0, 1.0, 1.0);
        assert_abs_diff_eq!(v.length(), 1.7320508);
    }

    #[test]
    fn add() {
        let v = Vector::new(5.0, 5.0, 5.0);
        assert_abs_diff_eq!((v+v).x, 10.0);
        assert_abs_diff_eq!((v+v).y, 10.0);
        assert_abs_diff_eq!((v+v).z, 10.0);
        assert_abs_diff_eq!((v+10.0).x, 15.0);
        assert_abs_diff_eq!((v+10.0).y, 15.0);
        assert_abs_diff_eq!((v+10.0).z, 15.0);
    }

    #[test]
    fn mul() {
        let v = Vector::new(5.0, 5.0, 5.0);
        assert_abs_diff_eq!((v*v).x, 25.0);
        assert_abs_diff_eq!((v*v).y, 25.0);
        assert_abs_diff_eq!((v*v).z, 25.0);
        assert_abs_diff_eq!((v*10.0).x, 50.0);
        assert_abs_diff_eq!((v*10.0).y, 50.0);
        assert_abs_diff_eq!((v*10.0).z, 50.0);
    }

    #[test]
    fn div() {
        let v = Vector::new(10.0, 10.0, 10.0);
        assert_abs_diff_eq!((v/v).x, 1.0);
        assert_abs_diff_eq!((v/v).y, 1.0);
        assert_abs_diff_eq!((v/v).z, 1.0);
        assert_abs_diff_eq!((v/5.0).x, 2.0);
        assert_abs_diff_eq!((v/5.0).y, 2.0);
        assert_abs_diff_eq!((v/5.0).z, 2.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Vector::new(4.0, 5.0, 5.0);
        let v2 = Vector::new(-5.0, -5.0, -5.0);
        let v3 = v1.cross(&v2);
        assert_abs_diff_eq!(v3.x, 0.0);
        assert_abs_diff_eq!(v3.y, -5.0);
        assert_abs_diff_eq!(v3.z, 5.0);
    }

    #[test]
    fn squared_length() {
        let v1 = Vector::new(10.0, 10.0, 10.0);
        let length = v1.squared_length();
        assert_abs_diff_eq!(length, 300.0);
    }

    #[test]
    fn unit() {
        let v1 = Vector::new(10.0, 10.0, 10.0);
        assert_abs_diff_eq!(v1.unit().length(), 1.0);
    }

    #[test]
    fn dot() {
        let v1 = Vector::new(10.0, 10.0, 10.0);
        let v2 = Vector::new(20.0, 20.0, 20.0);
        assert_abs_diff_eq!(v1.dot(&v2), 600.0);
    }
}