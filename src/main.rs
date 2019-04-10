extern crate raytracer;

fn main() {
    println!("Hello, world!");
    let c = raytracer::Color::new(0.5, 1.0, 1.0);
    let p = raytracer::Drawable::point(0.5, 1.0, 1.0);
    println!("{:?}", c);
    println!("{:?}", p);
    println!("{}", raytracer::feq(1.0, 1.0000002));
}
