use crate::util;

use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }

    pub fn white() -> Color {
        Self::new(1.0, 1.0, 1.0)
    }
    pub fn black() -> Color {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn to_256(f: f32) -> u16 {
        let mut f = f;
        if f > 1.0 {
            f = 1.0;
        } else if f < 0.0 {
            f = 0.0;
        }
        (255.0 * f).round() as u16
    }

    pub fn ppm(&self) -> String {
        format!(
            "{} {} {}",
            Self::to_256(self.red),
            Self::to_256(self.green),
            Self::to_256(self.blue)
        )
    }

    pub fn ppm_parts(&self) -> Vec<String> {
        vec![
            Self::to_256(self.red).to_string(),
            Self::to_256(self.green).to_string(),
            Self::to_256(self.blue).to_string(),
        ]
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        util::feq(self.red, other.red)
            && util::feq(self.green, other.green)
            && util::feq(self.blue, other.blue)
    }
}

impl Eq for Color {}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
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
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
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

    #[test]
    fn keyword_colors() {
        assert_eq!(Color::white(), Color::new(1.0, 1.0, 1.0));
        assert_eq!(Color::black(), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn generate_ppm() {
        assert_eq!(Color::new(1.0, 0.2, 0.4).ppm(), "255 51 102");
        assert_eq!(Color::new(1.5, 0.0, 0.0).ppm(), "255 0 0");
        assert_eq!(Color::new(-0.5, 0.5, 0.0).ppm(), "0 128 0");
    }

    #[test]
    fn ppm_parts() {
        let c = Color::new(1.0, 0.0, 0.4);
        assert_eq!(c.ppm_parts(), vec!["255", "0", "102"]);
    }
}
