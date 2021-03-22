use std::io;

fn fibonacci(mut num1: i32, mut num2: i32, size: i32) {
    // User defined function for printing fibonacci series
    for _i in 0..size {
        println!("{}", num1);
        let temp_num = num1 + num2;
        num1 = num2;
        num2 = temp_num;
    }
}

fn main() {
    println!("Enter size of fibonacci series required");
    let mut size = String::new(); // Empty String
    io::stdin().read_line(&mut size).expect("Error reading");
    let size: i32 = size.trim().parse().expect("Enter a number!"); // Parsing to { i32 } int

    println!("The fibonacci series are: ");
    fibonacci(0, 1, size);
}
