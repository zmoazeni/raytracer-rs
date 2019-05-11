use super::*;
use crate::drawable;

use std::ops::Mul;

impl Matrix {
    pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
        matrix![
            1.0, 0.0, 0.0, x;
            0.0, 1.0, 0.0, y;
            0.0, 0.0, 1.0, z;
            0.0, 0.0, 0.0, 1.0
        ]
    }
}

impl Mul<drawable::Point> for Matrix {
    type Output = Result<drawable::Point, String>;
    fn mul(self, rhs: drawable::Point) -> Self::Output {
        match self * rhs.matrix() {
            Err(e) => Err(e),
            Ok(matrix) => {
                Ok(drawable::Point::new(matrix[(0, 0)], matrix[(1, 0)], matrix[(2, 0)]))
            }
        }
    }
}

impl Mul<drawable::Vector> for Matrix {
    type Output = Result<drawable::Vector, String>;
    fn mul(self, rhs: drawable::Vector) -> Self::Output {
        match self * rhs.matrix() {
            Err(e) => Err(e),
            Ok(matrix) => {
                Ok(drawable::Vector::new(matrix[(0, 0)], matrix[(1, 0)], matrix[(2, 0)]))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn translation() {
        let t = Matrix::translation(5.0, -3.0, 2.0);
        assert_eq!(matrix![
            1.0, 0.0, 0.0, 5.0;
            0.0, 1.0, 0.0, -3.0;
            0.0, 0.0, 1.0, 2.0;
            0.0, 0.0, 0.0, 1.0
        ], t);

        assert!(t.is_invertable())
    }

    #[test]
    fn translate_point1() {
        let t = Matrix::translation(5.0, -3.0, 2.0);
        let p = drawable::Point::new(-3.0, 4.0, 5.0);
        assert_eq!(Ok(drawable::Point::new(2.0, 1.0, 7.0)), t * p);
    }

    #[test]
    fn translate_point2() {
        let t = Matrix::translation(5.0, -3.0, 2.0).inverse().unwrap();
        let p = drawable::Point::new(-3.0, 4.0, 5.0);
        assert_eq!(Ok(drawable::Point::new(-8.0, 7.0, 3.0)), t * p);
    }

    #[test]
    fn translate_vector_returns_itself() {
        let t = Matrix::translation(5.0, -3.0, 2.0).inverse().unwrap();
        let v = drawable::Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(Ok(v), t * v);
    }
}
