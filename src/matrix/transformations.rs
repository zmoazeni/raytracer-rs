use super::*;
use crate::space::{Point,Vector};

use std::ops::Mul;

impl Matrix {
    pub fn translation<X, Y, Z>(x: X, y: Y, z: Z) -> Matrix
        where X: Into<f32>, Y: Into<f32>, Z: Into<f32> {
        matrix![
            1.0, 0.0, 0.0, x.into();
            0.0, 1.0, 0.0, y.into();
            0.0, 0.0, 1.0, z.into();
            0.0, 0.0, 0.0, 1.0
        ]
    }

    pub fn scale<X, Y, Z>(x: X, y: Y, z: Z) -> Matrix
        where X: Into<f32>, Y: Into<f32>, Z: Into<f32> {
        matrix![
            x.into(), 0.0,      0.0,      0.0;
            0.0,      y.into(), 0.0,      0.0;
            0.0,      0.0,      z.into(), 0.0;
            0.0,      0.0,      0.0,      1.0
        ]
    }
}

impl Mul<Point> for Matrix {
    type Output = Result<Point, String>;
    fn mul(self, rhs: Point) -> Self::Output {
        match self * rhs.matrix() {
            Err(e) => Err(e),
            Ok(matrix) => {
                Ok(Point::new(matrix[(0, 0)], matrix[(1, 0)], matrix[(2, 0)]))
            }
        }
    }
}

impl Mul<Point> for Result<Matrix, String> {
    type Output = Result<Point, String>;
    fn mul(self, rhs: Point) -> Self::Output {
        self.and_then(|lhs| lhs * rhs)
    }
}

impl Mul<Vector> for Matrix {
    type Output = Result<Vector, String>;
    fn mul(self, rhs: Vector) -> Self::Output {
        match self * rhs.matrix() {
            Err(e) => Err(e),
            Ok(matrix) => {
                Ok(Vector::new(matrix[(0, 0)], matrix[(1, 0)], matrix[(2, 0)]))
            }
        }
    }
}
impl Mul<Vector> for Result<Matrix, String> {
    type Output = Result<Vector, String>;
    fn mul(self, rhs: Vector) -> Self::Output {
        self.and_then(|lhs| lhs * rhs)
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
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(Ok(Point::new(2.0, 1.0, 7.0)), t * p);
    }

    #[test]
    fn translate_point2() {
        let t = Matrix::translation(5.0, -3.0, 2.0).inverse();
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(Ok(Point::new(-8.0, 7.0, 3.0)), t * p);
    }

    #[test]
    fn translate_vector_returns_itself() {
        let t = Matrix::translation(5.0, -3.0, 2.0).inverse();
        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(Ok(v), t * v);
    }

    #[test]
    fn scaling_point() {
        let s = Matrix::scale(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(Ok(Point::new(-8.0, 18.0, 32.0)), s * p);
    }

    #[test]
    fn scaling_vector() {
        let s = Matrix::scale(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(Ok(Vector::new(-8.0, 18.0, 32.0)), s * v);
    }

    #[test]
    fn scaling_vector_inverse() {
        let s = Matrix::scale(2.0, 3.0, 4.0).inverse();
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(Ok(Vector::new(-2.0, 2.0, 2.0)), s * v);
    }
}
