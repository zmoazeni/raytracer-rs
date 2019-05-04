use raytracer::*;

fn main() {
    println!("in here");
    let a = matrix![
        1.0, 1.0;
        1.0, 1.0
    ];
    println!("{:?}", a);
    println!("{:?}", a.submatrix(3, 3));
    println!("{:?}", a.inverse());


    let r: Result<i32, &str> = Ok(1);
    println!("{:?}", r);

    let r2: Result<i32, &str> = Err("this is an error");
    println!("{:?}", r2);

    println!("{:?}", r.map(|x| x + 1));
    println!("{:?}", r2.map(|x| x + 1));
    println!("{:?}", r.unwrap());
    // println!("{:?}", r2.unwrap());
    println!("{:?}", r.and_then(|x| Ok(x + 10)));
}
