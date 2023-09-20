//standard library input/output and compare
use std::io;
use std::cmp::Ordering;
//rand library included in cargo.toml
use rand::Rng;

fn main() {
    //println with an "!" is a macro
    println!("Guess the number!");

    //use the rand library to generate a random number between 1-100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //infinite loop
    loop {
        println!("Please input your guess:");

        //let statement: creates a variable. let foo = bar.
        //variables in rust are immutable by default.
        //"mut" statement makes the variable mutable (can change the value later)

        //here, guess is being assigned to a new,empty instance of a String object
        let mut guess = String::new();

        //read user input into the "guess" String
        //.expect uses the Result type returned from read_line, handles input errors.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); //best practice in Rust to enter new line for .foo()

        //parse the entered string into a number type for comparison, continue if invalid
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //match expresseion to direct flow based on outcome
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it! The secret number was: {}", secret_number);
                //exit loop with correct guess
                break;
            },
        }
        println!("You guessed: {}", guess);
    }
}
