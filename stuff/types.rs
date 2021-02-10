const PI: f32 = 3.14159;

fn main() {

    let mut x: f64 = 3.141;
    println!("{}", x);

    x = 89.89;
    println!("{}", x);

    let y: f64;
    y = 7.2;
    println!("{}", y);

    let z;
    z = "Hi";
    println!("{}", z);

    let a : f64 = 4.3;
    let b = 4.3f32; // by default this is f64

    println!(
        "{} {}",
        a, b
    );

    println!("{}", PI);
}