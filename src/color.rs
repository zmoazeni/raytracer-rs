use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug)]
pub struct Color { red: f32, green: f32, blue: f32 }

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        super::feq(self.red, other.red) && super::feq(self.green, other.green) && super::feq(self.blue, other.blue)
    }
}

impl Eq for Color {}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(self.red + rhs.red, self.green + rhs.green, self.blue + rhs.blue)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color::new(self.red - rhs.red, self.green - rhs.green, self.blue - rhs.blue)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(self.red * rhs.red, self.green * rhs.green, self.blue * rhs.blue)
    }
}

#[cfg(test)]
mod test_colors {
    use super::*;

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0))
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5))
    }

    #[test]
    fn multiplying_by_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c1 * 2.0, Color::new(0.4, 0.6, 0.8))
    }

    #[test]
    fn multiplying_by_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04))
    }
}
