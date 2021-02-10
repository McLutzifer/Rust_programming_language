pub fn run() {
    // Print to cosnole
    println!("Hello from the print.rs file");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");
    
}