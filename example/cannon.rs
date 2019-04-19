extern crate raytracer;

use std::{thread,time};

struct Environment {
    gravity: raytracer::Drawable,
    wind: raytracer::Drawable
}

#[derive(Debug)]
struct Projectile {
    position: raytracer::Drawable,
    velocity: raytracer::Drawable
}

fn main() {
    let environment = Environment{
        gravity: raytracer::Drawable::vector(0.0, -0.1, 0.0),
        wind: raytracer::Drawable::vector(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: raytracer::Drawable::point(0.0, 1.0, 0.0),
        velocity: raytracer::Drawable::vector(1.0, 1.0, 0.0).normalize().unwrap(),
    };

    loop {
        projectile = tick(&environment, projectile);
        println!("{:?}", projectile);
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn tick(env: &Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity.clone();

    let velocity = (projectile.velocity + env.gravity.clone()).unwrap() + env.wind.clone();
    Projectile {
        position: position.unwrap(),
        velocity: velocity.unwrap()
    }
}
