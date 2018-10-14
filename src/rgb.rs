use core::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Vector {
        Rgb{r, g, b}
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rgb {{ r: {}, g: {}, b: {} }}", self.r, self.g, self.b)
    }
}