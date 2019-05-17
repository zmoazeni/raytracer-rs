extern crate rand;

use crate::util;
use crate::space::{Point,Vector};

use rand::Rng;
use std::ops::Index;

#[derive(Debug,Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray{origin, direction}
    }

    pub fn position<T: Into<f32>>(&self, t: T) -> Point {
        self.origin + (self.direction * t.into())
    }
}

#[derive(Debug,Clone)]
pub struct Sphere {
    id: i32
}

impl Sphere {
    pub fn new() -> Sphere {
        let mut rng = rand::thread_rng();
        Sphere{id: rng.gen::<i32>()}
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersections> {
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * (ray.direction.dot(&sphere_to_ray));
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            return None;
        }
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some(Intersections(vec![
            Intersection{t: t1, object: self},
            Intersection{t: t2, object: self}
        ]))
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct Intersections<'a>(Vec<Intersection<'a>>);


#[derive(Debug,Clone)]
pub struct Intersection<'a> {
    t: f32,
    object: &'a Sphere,
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, rhs: &Self) -> bool {
        util::feq(self.t, rhs.t) && self.object.id == rhs.object.id
    }
}

impl<'a> Intersections<'a> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn hit(&self) -> Option<&'a Intersection> {
        let mut ret = None;
        for intersection in &self.0 {
            if intersection.t > 0.0 {
                ret = match ret {
                    None => Some(intersection),
                    Some(i) => {
                        if i.t < intersection.t {
                            Some(i)
                        } else {
                            Some(intersection)
                        }
                    }
                };
            }
        }
        ret
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, x: usize) -> &Intersection<'a> {
        &self.0[x]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computing_distance() {
        let point = Point::new(2.0, 3.0, 4.0);
        let vector = Vector::new(1.0, 0.0, 0.0);
        let ray = Ray::new(point, vector);
        assert_eq!(ray.position(0.0),  Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0),  Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5),  Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = sphere.intersect(ray).unwrap();
        assert_eq!(2, intersections.len());
        assert_eq!(4.0, intersections[0].t);
        assert_eq!(6.0, intersections[1].t);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = sphere.intersect(ray).unwrap();
        assert_eq!(2, intersections.len());
        assert_eq!(5.0, intersections[0].t);
        assert_eq!(5.0, intersections[1].t);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = sphere.intersect(ray);
        assert_eq!(None, intersections);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = sphere.intersect(ray).unwrap();
        assert_eq!(2, intersections.len());
        assert_eq!(-1.0, intersections[0].t);
        assert_eq!(1.0, intersections[1].t);
    }


    #[test]
    fn ray_infront_of_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let intersections = sphere.intersect(ray).unwrap();
        assert_eq!(2, intersections.len());
        assert_eq!(-6.0, intersections[0].t);
        assert_eq!(-4.0, intersections[1].t);
    }

    #[test]
    fn hit_with_all_positive() {
        let s = Sphere::new();
        let intersections = Intersections(vec![
            Intersection{t: 1.0, object: &s},
            Intersection{t: 2.0, object: &s},
        ]);
        assert_eq!(1.0, intersections.hit().unwrap().t);
    }

    #[test]
    fn hit_with_some_negative() {
        let s = Sphere::new();
        let intersections = Intersections(vec![
            Intersection{t: -1.0, object: &s},
            Intersection{t: 2.0, object: &s},
        ]);
        assert_eq!(2.0, intersections.hit().unwrap().t);
    }

    #[test]
    fn hit_with_all_negative() {
        let s = Sphere::new();
        let intersections = Intersections(vec![
            Intersection{t: -1.0, object: &s},
            Intersection{t: -2.0, object: &s},
        ]);
        assert_eq!(None, intersections.hit());
    }


    #[test]
    fn hit_always_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let intersections = Intersections(vec![
            Intersection{t: 5.0, object: &s},
            Intersection{t: 7.0, object: &s},
            Intersection{t: -3.0, object: &s},
            Intersection{t: 2.0, object: &s},
        ]);
        assert_eq!(2.0, intersections.hit().unwrap().t);
    }
}
