pub struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
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
    fn another() {
        let v = Vector {x: 1.0, y: 1.0, z: 1.0};
        assert_eq!(v.length(), 1.0);
    }
}