use super::*;
use crate::space::{Point,Vector};

use std::ops::Mul;

pub enum Shear {
    XY,
    XZ,
    YX,
    YZ,
    ZX,
    ZY
}

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

    pub fn rotation_x<T: Into<f32>>(radians: T) -> Matrix {
        let radians = radians.into();
        let sin = radians.sin();
        let cos = radians.cos();
        matrix![
            1.0, 0.0, 0.0,        0.0;
            0.0, cos, -1.0 * sin, 0.0;
            0.0, sin, cos,        0.0;
            0.0, 0.0, 0.0,        1.0
        ]
    }

    pub fn rotation_y<T: Into<f32>>(radians: T) -> Matrix {
        let radians = radians.into();
        let sin = radians.sin();
        let cos = radians.cos();
        matrix![
            cos,        0.0, sin, 0.0;
            0.0,        1.0, 0.0, 0.0;
            -1.0 * sin, 0.0, cos, 0.0;
            0.0,        0.0, 0.0, 1.0
        ]
    }

    pub fn rotation_z<T: Into<f32>>(radians: T) -> Matrix {
        let radians = radians.into();
        let sin = radians.sin();
        let cos = radians.cos();
        matrix![
            cos, -1.0 * sin, 0.0, 0.0;
            sin, cos,        0.0, 0.0;
            0.0, 0.0,        1.0, 0.0;
            0.0, 0.0,        0.0, 1.0
        ]
    }

    pub fn shear(relation: Shear) -> Matrix {
        let (xy, xz, yx, yz, zx, zy) = match relation {
            Shear::XY => (1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            Shear::XZ => (0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
            Shear::YX => (0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
            Shear::YZ => (0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            Shear::ZX => (0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
            Shear::ZY => (0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
        };

        matrix![
            1.0, xy,  xz,  0.0;
            yx,  1.0, yz,  0.0;
            zx,  zy,  1.0, 0.0;
            0.0, 0.0, 0.0, 1.0
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
    use std::f32::consts::PI;

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

    #[test]
    fn scaling_point_reflection() {
        // scaling a point by a negative value flips the axis
        let p = Point::new(2.0, 3.0, 4.0);
        let s = Matrix::scale(-1.0, 1.0, 1.0);
        assert_eq!(Ok(Point::new(-2.0, 3.0, 4.0)), s * p);
    }

    #[test]
    fn rotating_point_around_x_axis_regular() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        assert_eq!(Ok(Point::new(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0)), half_quarter * p);
        assert_eq!(Ok(Point::new(0.0, 0.0, 1.0)), full_quarter * p);
    }

    #[test]
    fn rotating_point_around_x_axis_inverse() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        assert_eq!(Ok(Point::new(0.0, 2.0_f32.sqrt() / 2.0, -1.0 * (2.0_f32.sqrt() / 2.0))), half_quarter.inverse() * p);
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);
        assert_eq!(Ok(Point::new(2.0_f32.sqrt() / 2.0, 0.0, 2.0_f32.sqrt() / 2.0)), half_quarter * p);
        assert_eq!(Ok(Point::new(1.0, 0.0, 0.0)), full_quarter * p);

    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);
        assert_eq!(Ok(Point::new(-1.0 * (2.0_f32.sqrt() / 2.0), 2.0_f32.sqrt() / 2.0, 0.0)), half_quarter * p);
        assert_eq!(Ok(Point::new(-1.0, 0.0, 0.0)), full_quarter * p);

    }

    #[test]
    fn shearing_x_by_y() {
        let p = Point::new(2.0, 3.0, 4.0);
        let s = Matrix::shear(Shear::XY);
        assert_eq!(Ok(Point::new(5.0, 3.0, 4.0)), s * p);
    }

    #[test]
    fn shearing_x_by_z() {
        let p = Point::new(2.0, 3.0, 4.0);
        let s = Matrix::shear(Shear::XZ);
        assert_eq!(Ok(Point::new(6.0, 3.0, 4.0)), s * p);
    }

    #[test]
    fn shearing_y_by_x() {
        let p = Point::new(2.0, 3.0, 4.0);
        let s = Matrix::shear(Shear::YX);
        assert_eq!(Ok(Point::new(2.0, 5.0, 4.0)), s * p);
    }

    #[test]
    fn shearing_y_by_z() {
        let p = Point::new(2.0, 3.0, 4.0);
        let s = Matrix::shear(Shear::YZ);
        assert_eq!(Ok(Point::new(2.0, 7.0, 4.0)), s * p);
    }

    #[test]
    fn shearing_z_by_x() {
        let p = Point::new(2.0, 3.0, 4.0);
        let s = Matrix::shear(Shear::ZX);
        assert_eq!(Ok(Point::new(2.0, 3.0, 6.0)), s * p);
    }

    #[test]
    fn shearing_z_by_6() {
        let p = Point::new(2.0, 3.0, 4.0);
        let s = Matrix::shear(Shear::ZY);
        assert_eq!(Ok(Point::new(2.0, 3.0, 7.0)), s * p);
    }
}
