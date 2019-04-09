use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

const THRESHOLD: f32 = 0.00001;

#[derive(Debug)]
pub enum Drawable {
  Point(f32, f32, f32),
  Vector(f32, f32, f32),
}

impl Drawable {
  pub fn point(x: f32, y: f32, z: f32) -> Drawable {
    Drawable::Point(x, y, z)
  }

  pub fn vector(x: f32, y: f32, z: f32) -> Drawable {
    Drawable::Vector(x, y, z)
  }

  pub fn negate(&self) -> Drawable {
    match self {
      Drawable::Vector(x, y, z) => Drawable::Vector(-x, -y, -z),
      Drawable::Point(x, y, z) => Drawable::Point(-x, -y, -z),
    }
  }

  pub fn magnitude(&self) -> Option<f32> {
    match self {
      Drawable::Point(..) => None,
      Drawable::Vector(x, y, z) => {
        let v = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        Some(v)
      }
    }
  }

  pub fn normalize(&self) -> Option<Drawable> {
    match self {
      Drawable::Point(..) => None,
      Drawable::Vector(x, y, z) => {
        let magnitude = self.magnitude().unwrap();
        Some(Drawable::vector(x / magnitude, y / magnitude, z / magnitude))
      }
    }
  }
}

impl Add for Drawable {
    type Output = Option<Drawable>;

    fn add(self, other: Drawable) -> Option<Drawable> {
      match self {
        Drawable::Vector(x1, y1, z1) => {
          match other {
            Drawable::Vector(x2, y2, z2) => Some(Drawable::vector(x1 + x2, y1 + y2, z1 + z2)),
            Drawable::Point(x2, y2, z2) => Some(Drawable::point(x1 + x2, y1 + y2, z1 + z2)),
          }
        },
        Drawable::Point(x1, y1, z1) => {
          match other {
            Drawable::Vector(x2, y2, z2) => Some(Drawable::point(x1 + x2, y1 + y2, z1 + z2)),
            Drawable::Point(..) => None,
          }
        }
      }
    }
}

impl Sub for Drawable {
    type Output = Option<Drawable>;

    fn sub(self, rhs: Drawable) -> Option<Drawable> {
      match self {
        Drawable::Vector(x1, y1, z1) => {
          match rhs {
            Drawable::Vector(x2, y2, z2) => Some(Drawable::vector(x1 - x2, y1 - y2, z1 - z2)),
            Drawable::Point(..) => None,
          }
        },
        Drawable::Point(x1, y1, z1) => {
          match rhs {
            Drawable::Point(x2, y2, z2) => Some(Drawable::vector(x1 - x2, y1 - y2, z1 - z2)),
            Drawable::Vector(x2, y2, z2) => Some(Drawable::point(x1 - x2, y1 - y2, z1 - z2)),
          }
        }
      }
    }
}

impl Mul<f32> for Drawable {
    type Output = Drawable;

    fn mul(self, rhs: f32) -> Drawable {
      match self {
        Drawable::Point(x, y, z) => Drawable::Point(x * rhs, y * rhs, z * rhs),
        Drawable::Vector(x, y, z) => Drawable::Vector(x * rhs, y * rhs, z * rhs),
      }
    }
}

impl Div<f32> for Drawable {
    type Output = Drawable;

    fn div(self, rhs: f32) -> Drawable {
      match self {
        Drawable::Point(x, y, z) => Drawable::Point(x / rhs, y / rhs, z / rhs),
        Drawable::Vector(x, y, z) => Drawable::Vector(x / rhs, y / rhs, z / rhs),
      }
    }
}

impl PartialEq for Drawable {
    fn eq(&self, other: &Drawable) -> bool {
      match self {
        Drawable::Vector(x1, y1, z1) => {
          match other {
            Drawable::Vector(x2, y2, z2) => feq(*x1, *x2) && feq(*y1, *y2) && feq(*z1, *z2),
            Drawable::Point(..) => false,
          }
        },
        Drawable::Point(x1, y1, z1) => {
          match other {
            Drawable::Point(x2, y2, z2) => feq(*x1, *x2) && feq(*y1, *y2) && feq(*z1, *z2),
            Drawable::Vector(..) => false,
          }
        },
      }
    }
}

impl Eq for Drawable {}

fn feq(f1: f32, f2: f32) -> bool {
  (f1 - f2).abs() < THRESHOLD
}

#[cfg(test)]
mod test_add {
  use super::*;

  #[test]
  fn vector_to_point() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    let vector = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(vector + point, Some(Drawable::Point(2.0, 4.0, 6.0)));
  }

  #[test]
  fn vector_to_vector() {
    let vector1 = Drawable::vector(1.0, 2.0, 3.0);
    let vector2 = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(vector1 + vector2, Some(Drawable::Vector(2.0, 4.0, 6.0)));
  }

  #[test]
  fn point_to_point() {
    let point1 = Drawable::point(1.0, 2.0, 3.0);
    let point2 = Drawable::point(1.0, 2.0, 3.0);
    assert_eq!(point1 + point2, None);
  }

  #[test]
  fn point_to_vector() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    let vector = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(point + vector, Some(Drawable::Point(2.0, 4.0, 6.0)));
  }
}

#[cfg(test)]
mod test_subtract {
  use super::*;

  #[test]
  fn point_from_a_point() {
    let point1 = Drawable::point(1.0, 2.0, 3.0);
    let point2 = Drawable::point(10.0, 20.0, 30.0);
    assert_eq!(point2 - point1, Some(Drawable::Vector(9.0, 18.0, 27.0)));
  }

  #[test]
  fn point_from_a_vector() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    let vector = Drawable::vector(10.0, 20.0, 30.0);
    assert_eq!(vector - point, None);
  }

  #[test]
  fn vector_from_a_vector() {
    let vector1 = Drawable::vector(1.0, 2.0, 3.0);
    let vector2 = Drawable::vector(10.0, 20.0, 30.0);
    assert_eq!(vector2 - vector1, Some(Drawable::Vector(9.0, 18.0, 27.0)));
  }

  #[test]
  fn vector_from_a_point() {
    let point = Drawable::point(10.0, 20.0, 30.0);
    let vector = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(point - vector, Some(Drawable::Point(9.0, 18.0, 27.0)));
  }
}

#[cfg(test)]
mod test_negation {
  use super::*;

  #[test]
  fn point() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    assert_eq!(point.negate(), Drawable::Point(-1.0, -2.0, -3.0));
  }

  #[test]
  fn vector() {
    let vector = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(vector.negate(), Drawable::Vector(-1.0, -2.0, -3.0));
  }
}

#[cfg(test)]
mod test_mult_and_div {
  use super::*;

  #[test]
  fn multiplying_point_and_vector() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    assert_eq!(point * 2.0, Drawable::Point(2.0, 4.0, 6.0));

    let vector = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(vector * 2.0, Drawable::Vector(2.0, 4.0, 6.0));
  }

  #[test]
  fn dividing_point_and_vector() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    assert_eq!(point / 2.0, Drawable::Point(0.5, 1.0, 1.5));

    let vector = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(vector / 2.0, Drawable::Vector(0.5, 1.0, 1.5));
  }
}

#[cfg(test)]
mod test_magnitude_and_normalization {
  use super::*;

  #[test]
  fn magnitude_of_point() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    assert_eq!(point.magnitude(), None);
  }

  #[test]
  fn magnitude_of_vector() {
    let vector = Drawable::vector(1.0, 2.0, 3.0);
    let x: f32 = 14.0;
    assert_eq!(vector.magnitude().unwrap(), x.sqrt());
  }

  #[test]
  fn normalize_point() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    assert_eq!(point.normalize(), None);
  }

  #[test]
  fn normalize_vector() {
    let vector = Drawable::vector(4.0, 0.0, 0.0);
    assert_eq!(vector.normalize(), Some(Drawable::vector(1.0, 0.0, 0.0)));

    let vector = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(vector.normalize(), Some(Drawable::vector(0.26726, 0.53452, 0.80178)))
  }

  #[test]
  fn magnitude_of_a_normalized_vector_is_always_one() {
    let vector = Drawable::vector(4.0, 0.0, 0.0);
    assert_eq!(vector.normalize().unwrap().magnitude(), Some(1.0));
  }
}
