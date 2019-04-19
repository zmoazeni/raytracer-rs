extern crate raytracer;

use std::env;
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
    let args: Vec<String> = env::args().collect();
    let mut strength_factor = 1.0;
    if args.len() > 1 {
        strength_factor = args[1].parse::<f32>().unwrap_or(1.0);
    }
    println!("Strength factor: {}", strength_factor);

    let environment = Environment{
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize() * strength_factor,
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
