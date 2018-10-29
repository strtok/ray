use core::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb{r, g, b}
    }
}

impl fmt::Debug for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rgb {{ r: {}, g: {}, b: {} }}", self.r, self.g, self.b)
    }
}

impl Add for Rgb {
    type Output = Rgb;
    fn add(self, other: Rgb) -> Rgb {
        Rgb {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }
    }
}

impl<'a, 'b> Add<&'b Rgb> for &'a Rgb {
    type Output = Rgb;

    fn add(self, other: &'b Rgb) -> Rgb {
        Rgb {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }
    }
}

impl Add<f32> for Rgb {
    type Output = Rgb;
    fn add(self, other: f32) -> Rgb {
        Rgb {
            r: self.r + other,
            g: self.g + other,
            b: self.b + other
        }
    }
}


impl Sub for Rgb {
    type Output = Rgb;
    fn sub(self, other: Rgb) -> Rgb {
        Rgb {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b
        }
    }
}

impl Mul for Rgb {
    type Output = Rgb;

    fn mul(self, other: Rgb) -> Rgb {
        Rgb {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b
        }
    }
}


impl Mul<f32> for Rgb {
    type Output = Rgb;

    fn mul(self, other: f32) -> Rgb {
        Rgb {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other
        }
    }
}

impl Div for Rgb {
    type Output = Rgb;
    fn div(self, other: Rgb) -> Rgb {
        Rgb {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b
        }
    }
}


impl Div <f32>for Rgb {
    type Output = Rgb;
    fn div(self, other: f32) -> Rgb {
        Rgb {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other
        }
    }
}

impl<'a> Div<f32> for &'a Rgb {
    type Output = Rgb;

    fn div(self, other: f32) -> Rgb {
        Rgb {
            r: self.r + other,
            g: self.g + other,
            b: self.b + other
        }
    }
}