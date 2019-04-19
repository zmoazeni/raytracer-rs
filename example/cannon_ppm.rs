extern crate raytracer;

use std::path;
use raytracer::{Point,Vector,Canvas,Color};

struct Environment {
    gravity: Vector,
    wind: Vector
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector
}

const HEIGHT: usize = 550;
const WIDTH: usize = 900;
const STRENGTH_FACTOR: f32 = 11.25;

fn main() {
    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(0.3, 1.0, 0.0).normalize() * STRENGTH_FACTOR,
    };

    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut count = 0;
    let cannon_color = Color::new(1.0, 0.5, 0.5);
    loop {
        let Point(x, y, _) = projectile.position;
        if y < 0.0 {
            break;
        } else {
            println!("{}: {:?}", count, projectile);

            let x = x.round() as usize;
            let y = y.round() as usize;

            if y <= HEIGHT {
                canvas.set((x, HEIGHT - y), cannon_color);
            }

            projectile = tick(&environment, projectile);
            count += 1;
        }
    }

    let is_saved = canvas.save_as_ppm(path::Path::new("./tmp/cannon.ppm"));
    if is_saved.is_err() {
        panic!("Couldn't write the file");
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
