use std::io;

fn main() {
    // Loop statement to process until user inputs a number below or equal to { 10 }
    loop {
        println!("Enter a number below 10");
        let mut number = String::new(); // Empty String
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        // Shadowing { number }
        let number: i32 = number.trim().parse().expect("Enter a number!"); // Parsing to { i32 } int
        if number <= 10 {
            for i in 1..number {
                println!("{}", i);
            }
            break;
        } else {
            println!("{} is greater than 10, enter again!", number);
        }
    }
}
