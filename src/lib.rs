use std::ops::Add;

const THRESHOLD: f32 = 0.00001;

#[derive(Debug)]
pub enum Drawable {
  Point (f32, f32, f32),
  Vector (f32, f32, f32),
}

impl Drawable {
  pub fn point(x: f32, y: f32, z: f32) -> Drawable {
    Drawable::Point(x, y, z)
  }

  pub fn vector(x: f32, y: f32, z: f32) -> Drawable {
    Drawable::Vector(x, y, z)
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
mod tests {
  use super::*;

  #[test]
  fn vector_added_to_point() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    let vector = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(vector + point, Some(Drawable::Point(2.0, 4.0, 6.0)));
  }

  #[test]
  fn vector_added_to_vector() {
    let vector1 = Drawable::vector(1.0, 2.0, 3.0);
    let vector2 = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(vector1 + vector2, Some(Drawable::Vector(2.0, 4.0, 6.0)));
  }

  #[test]
  fn point_added_to_point() {
    let point1 = Drawable::point(1.0, 2.0, 3.0);
    let point2 = Drawable::point(1.0, 2.0, 3.0);
    assert_eq!(point1 + point2, None);
  }

  #[test]
  fn point_added_to_vector() {
    let point = Drawable::point(1.0, 2.0, 3.0);
    let vector = Drawable::vector(1.0, 2.0, 3.0);
    assert_eq!(point + vector, Some(Drawable::Point(2.0, 4.0, 6.0)));
  }
}
