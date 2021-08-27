fn add(x: i32, y: i32) -> i32 {
    // implicit return (no semicolon)
    x + y
}

fn main() {
    // Nubers //

    // Immutable bindings
    let x: i32 = 1;

    // Integer/float suffixes
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;

    // Type inference
    let implicit_x = 1;
    let implicit_f = 1.3;

    // Arithmetic
    let sum = x + y + 13;

    // Mutable variable
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // Strings //

    // String literals
    let x: &str = "hello world!";

    // Printing
    println!("{} {}", f, x);  // 1.3 hello world

    // A `String` - a heap-allocated string
    let s: String = "hello world".to_string();
}