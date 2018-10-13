pub struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
    fn from(x: f32, y: f32, z: f32) -> Vector {
        return Vector{x: x, y: y, z: z};
    }

    fn length(&self) -> f32 {
        return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
    }

    fn squared_length(&self) -> f32 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }
}

#[cfg(test)]
mod tests {
    use vector::*;
    #[test]
    fn length() {
        let v = Vector::from(1.0, 1.0, 1.0);
        assert!(v.length() > 1.0);
    }
}