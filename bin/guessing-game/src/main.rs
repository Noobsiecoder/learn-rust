/*
 * std::io :
 *  - Input/ Output library from standard library.
 *  - Rust brings only a few types into scope known as "prelude".
 *  - Using this library, we accept value from the user.
 * rand::Rng :
 *  - Library to produce random values.
 * std::cmp::Ordering :
 *  - Library to order values respectively.
 */
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Main function
fn main() {
    println!("Guess the number");
    // { secret_number } stores a random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Looping stating to loop and break out of it once { break } statement is executed
    loop {
        println!("Please input your guess");
        /*
         * Creating a variable that is mutable.
         * By default, variables in Rust are immutable.
         * Hence we use the keyword "mut".
         */
        let mut guess = String::new(); // { String::new() } creates an empty string

        io::stdin()
            .read_line(&mut guess) // "&" references to the string variable { guess } without needing to copy that data into memory multiple times
            .expect("Failed to read line"); // Rust is strict when it comes to error handling during run-time too. If { expect } function isn't included, while compiling it will fail stating that we haven't handled the result from { read_line() } function

        /*
         * Rust shadows the string to an integer.
         * The input is stored as "<number>/n", so { trim() } function is used to remove any whitespace.
         * { parse() } is used to convert it's data type. We've used { : } which tells Rust to convert to { 32-bit int }
         */
        let guess: u32 = guess.trim().parse().expect("Please enter a string!");

        println!("You guessed: {}", guess); // The space inside the curly braces are used to hold respective value in place

        // Function to check if { guess } and { secret_number } are equivalent
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                // Arrow function similar to JavaScript
                println!("Numbers are Equal");
                break;
            }
        }
    }
}
