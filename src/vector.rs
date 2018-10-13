use core::fmt;

pub struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector{x, y, z }
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

#[cfg(test)]
mod tests {
    use vector::*;
    #[test]
    fn length() {
        let v = Vector::new(1.0, 1.0, 1.0);
        assert!(v.length() > 1.0);
    }
}