extern crate raytracer;

use raytracer::{Point,Vector};

struct Environment {
    gravity: Vector,
    wind: Vector
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector
}

fn main() {
    let environment = Environment{
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize() * 1.5,
    };

    let mut count = 0;
    loop {
        println!("{}: {:?}", count, projectile);
        let Point(_, y, _) = projectile.position;
        if y < 0.0 {
            break;
        } else {
            projectile = tick(&environment, projectile);
            count += 1;
        }

    }
}

fn tick(env: &Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;

    let velocity = projectile.velocity + env.gravity + env.wind;
    Projectile {
        position: position,
        velocity: velocity,
    }
}
