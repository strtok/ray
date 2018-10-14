use core::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector{x, y, z }
    }

    pub fn unit(&self) -> Vector {
        let k = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector::new(k*self.x, k*self.y, k*self.z)
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(self.y*other.z - self.z*other.y,
                    self.z*other.x - self.x*other.z,
                    self.x*other.y - self.y*other.x)
    }

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
        assert!(v.length() > 1.0);
    }
}