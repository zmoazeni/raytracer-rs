extern crate raytracer;

use std::{thread,time};

struct Environment {
    gravity: raytracer::Vector,
    wind: raytracer::Vector
}

#[derive(Debug)]
struct Projectile {
    position: raytracer::Point,
    velocity: raytracer::Vector
}

fn main() {
    let environment = Environment{
        gravity: raytracer::Vector::new(0.0, -0.1, 0.0),
        wind: raytracer::Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: raytracer::Point::new(0.0, 1.0, 0.0),
        velocity: raytracer::Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    loop {
        projectile = tick(&environment, projectile);
        println!("{:?}", projectile);
        thread::sleep(time::Duration::from_secs(1));
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
