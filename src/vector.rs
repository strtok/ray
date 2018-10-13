pub struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector{x, y, z }
    }

    fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    fn squared_length(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
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