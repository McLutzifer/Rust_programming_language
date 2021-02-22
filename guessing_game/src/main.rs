use std::io;                //to get user input
use std::cmp::Ordering;     //enum with LEss, Greater, Equal
use rand::Rng;              // add rand = "0.3.14" to dependencies in toml

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);       //creating random number

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win"),
    }

}