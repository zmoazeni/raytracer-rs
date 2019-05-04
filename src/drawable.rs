use crate::util;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, Copy, Clone)]
pub struct Point(pub f32, pub f32, pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Vector(pub f32, pub f32, pub f32);

impl Point {
    pub fn new<X, Y, Z>(x: X, y: Y, z: Z) -> Point
        where X: Into<f32>, Y: Into<f32>, Z: Into<f32> {
        Point(x.into(), y.into(), z.into())
    }

    pub fn negate(&self) -> Point {
        *self * -1.0
    }
}

impl Vector {
    pub fn new<X, Y, Z>(x: X, y: Y, z: Z) -> Vector
        where X: Into<f32>, Y: Into<f32>, Z: Into<f32> {
        Vector(x.into(), y.into(), z.into())
    }

    pub fn negate(&self) -> Vector {
        *self * -1.0
    }

    // magnitude is the distance traveled if you were to walk the vector
    pub fn magnitude(&self) -> f32 {
        let Vector(x, y, z) = self;
        let v = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        v
    }

    // converts the vector into a unit vector
    pub fn normalize(&self) -> Vector {
        let Vector(x, y, z) = self;
        let magnitude = self.magnitude();
        Self::new(x / magnitude, y / magnitude, z / magnitude)
    }

    // The smaller the output, the larger the angle between vectors
    pub fn dot(&self, rhs: &Vector) -> f32 {
        let Vector(x1, y1, z1) = self;
        let Vector(x2, y2, z2) = rhs;
        return (x1 * x2) + (y1 * y2) + (z1 * z2)
    }

    // Finds new vector perpendicular to two vectors
    pub fn cross(&self, rhs: &Vector) -> Vector {
        let Vector(x1, y1, z1) = self;
        let Vector(x2, y2, z2) = rhs;
        Self::new(
            (y1 * z2) - (z1 * y2),
            (z1 * x2) - (x1 * z2),
            (x1 * y2) - (y1 * x2)
        )
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        let Vector(x1, y1, z1) = self;
        let Vector(x2, y2, z2) = rhs;
        Self::new(x1 + x2, y1 + y2, z1 + z2)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        let Vector(x1, y1, z1) = self;
        let Point(x2, y2, z2) = rhs;
        Point::new(x1 + x2, y1 + y2, z1 + z2)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        let Point(x1, y1, z1) = self;
        let Vector(x2, y2, z2) = rhs;
        Self::new(x1 + x2, y1 + y2, z1 + z2)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        let Vector(x1, y1, z1) = self;
        let Vector(x2, y2, z2) = rhs;
        Self::new(x1 - x2, y1 - y2, z1 - z2)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        let Point(x1, y1, z1) = self;
        let Point(x2, y2, z2) = rhs;
        Vector::new(x1 - x2, y1 - y2, z1 - z2)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        let Point(x1, y1, z1) = self;
        let Vector(x2, y2, z2) = rhs;
        Point::new(x1 - x2, y1 - y2, z1 - z2)
    }
}

impl<T: Into<f32>> Mul<T> for Point {
    type Output = Point;

    fn mul(self, rhs: T) -> Point {
        let Point(x, y, z) = self;
        let factor = rhs.into();
        Self::new(x * factor, y * factor, z * factor)
    }
}

impl<T: Into<f32>> Mul<T> for Vector {
    type Output = Vector;

    fn mul(self, rhs: T) -> Vector {
        let Vector(x, y, z) = self;
        let factor = rhs.into();
        Self::new(x * factor, y * factor, z * factor)
    }
}

impl<T: Into<f32>> Div<T> for Point {
    type Output = Point;

    fn div(self, rhs: T) -> Point {
        let Point(x, y, z) = self;
        let factor = rhs.into();
        Self::new(x / factor, y / factor, z / factor)
    }
}

impl<T: Into<f32>> Div<T> for Vector {
    type Output = Vector;

    fn div(self, rhs: T) -> Vector {
        let Vector(x, y, z) = self;
        let factor = rhs.into();
        Self::new(x / factor, y / factor, z / factor)
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        let Point(x1, y1, z1) = self;
        let Point(x2, y2, z2) = rhs;
        util::feq(*x1, *x2) && util::feq(*y1, *y2) && util::feq(*z1, *z2)
    }
}

impl PartialEq for Vector {
    fn eq(&self, rhs: &Vector) -> bool {
        let Vector(x1, y1, z1) = self;
        let Vector(x2, y2, z2) = rhs;
        util::feq(*x1, *x2) && util::feq(*y1, *y2) && util::feq(*z1, *z2)
    }
}

impl Eq for Point {}
impl Eq for Vector {}


#[cfg(test)]
mod test_add {
    use super::*;

    #[test]
    fn vector_to_point() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector + point, Point::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector_to_vector() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector1 + vector2, Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn point_to_vector() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(point + vector, Point::new(2.0, 4.0, 6.0));
    }
}

#[cfg(test)]
mod test_subtract {
    use super::*;

    #[test]
    fn point_from_a_point() {
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(10.0, 20.0, 30.0);
        assert_eq!(point2 - point1, Vector::new(9.0, 18.0, 27.0));
    }

    #[test]
    fn vector_from_a_vector() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(10.0, 20.0, 30.0);
        assert_eq!(vector2 - vector1, Vector::new(9.0, 18.0, 27.0));
    }

    #[test]
    fn vector_from_a_point() {
        let point = Point::new(10.0, 20.0, 30.0);
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(point - vector, Point::new(9.0, 18.0, 27.0));
    }
}

#[cfg(test)]
mod test_negation {
    use super::*;

    #[test]
    fn point() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(point.negate(), Point::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn vector() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector.negate(), Vector::new(-1.0, -2.0, -3.0));
    }
}

#[cfg(test)]
mod test_mult_and_div {
    use super::*;

    #[test]
    fn multiplying_point_and_vector() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(point * 2.0, Point::new(2.0, 4.0, 6.0));

        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector * 2.0, Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn dividing_point_and_vector() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(point / 2.0, Point::new(0.5, 1.0, 1.5));

        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector / 2.0, Vector::new(0.5, 1.0, 1.5));
    }
}

#[cfg(test)]
mod test_magnitude_and_normalization {
    use super::*;

    #[test]
    fn magnitude_of_vector() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        let x: f32 = 14.0;
        assert_eq!(vector.magnitude(), x.sqrt());
    }

    #[test]
    fn normalize_vector() {
        let vector = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(vector.normalize(), Vector::new(1.0, 0.0, 0.0));

        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector.normalize(), Vector::new(0.26726, 0.53452, 0.80178))
    }

    #[test]
    fn magnitude_of_a_normalized_vector_is_always_one() {
        let vector = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(vector.normalize().magnitude(), 1.0);
    }
}

#[cfg(test)]
mod test_dot_and_cross {
    use super::*;

    #[test]
    fn dot_product_with_vectors() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(vector1.dot(&vector2), 20.0);
        assert_eq!(vector2.dot(&vector1), 20.0);
    }

    #[test]
    fn cross_product_with_vectors() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(vector1.cross(&vector2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(vector2.cross(&vector1), Vector::new(1.0, -2.0, 1.0));
    }
}
