# Control Flow in Rust

- Deciding whether or not to run some code depending on if a condition is true and deciding to run some code repeatedly while a condition is true are basic building blocks is _control flow_.

## `if`, `else` and `else if` expression

- `if` condition is used to run a block of code when the condition is true

  ```rust
  fn main() {
      let number = 3;
      // Condition must always be enclosed inside a bracket even if it is a one-line statement
      if number < 5 {
          println!("Condition is true!");
      } else {
          println!("Condition is false");
      }
      // Prints "Condition is true!"
  }
  ```

- In JavaScript, strings evaluate to `true`, in rust :

  ```rust
  /*
  * The compiler will show an error
  * It will state that :- expected `bool`, found `&str`
  */
  fn main() {
      if ("true") {
          println!("How?");
      } else {
          println!("How again?");
      }
  }
  ```

- To combine more than 1 `if` condition, we can use `else if` :

  ```rust
  fn main() {
      let number = 6;

      // Checks the remainder
      if number % 4 == 0 {
          println!("Number is divisible by 4");
      } else if number % 3 == 0 {
          println!("Number is divisible by 3");
      } else {
          println!("Number is not divisible by 4 or 3");
      } // Prints "Number is divisible by 3"
  }
  ```

## Using `if` in `let` statement

```rust
fn main() {
    let condition: bool = false;
    let number: i32 = if condition { 25 } else { -25 };
    println!("Number is {}", number); // Number is -25
}
```

- **Note** : Both value inside `if` and `else` block must be of same type and must be different.

## Loops

- It’s often useful to execute a block of code more than once.
- For this task, Rust provides several loops.
- A loop runs through the code inside the loop body to the end and then starts immediately back at the beginning.

### Infinite Loop

```rust
fn main() {
    // An infinite loop
    loop {
        println!("Infinite! Press `crtl` + `C` to stop the program");
    }
}
```

### Returning Values from Loops

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 10 // Here ";" can be used
        }
    };
    println!("The result is {}", result); // The result is 100
}
```

### Conditional Loops with `while`

```rust
fn main() {
    let mut number = 5;
    while number >= 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("Success!");
    /*
     * Output :-
     * 5
     * 4
     * 3
     * 2
     * 1
     * Success!
     */
}
```

### Looping Through a Collection with `while` and `for`

1. `while` loop :

   ```rust
   fn main() {
       let arr: [i8; 5] = [1, 2, 3, 4, 5];
       let mut index = 0; // Since array index starts from 0
       while index < arr.len() {
           println!("The value of a[{}] = {}", index, arr[index]);
           index += 1;
       }
       /*
       * Output :-
       * The value of a[0] = 1
       * The value of a[1] = 2
       * The value of a[2] = 3
       * The value of a[3] = 4
       * The value of a[4] = 5
       */
   }
   ```

2. `for` loop :

   ```rust
   fn main() {
       let arr: [i8; 5] = [1, 2, 3, 4, 5];
       let mut index = 0; // Since array index starts from 0
       for element in arr.iter() {
           println!("The value of a[{}] = {}", index, element);
           index += 1;
       }
       /*
       * Output :-
       * The value of a[0] = 1
       * The value of a[1] = 2
       * The value of a[2] = 3
       * The value of a[3] = 4
       * The value of a[4] = 5
       */
   }
   ```

## Miscellaneous

1. [Print 1 to `n`, where `n` is user input](src/userInput.rs)
2. [Fibonacci series](src/fibonacci.rs)

## Note

- _Unlike **JavaScript**, **Rust** knows how to handle conditions XD_
- _Paranthesis shouldn't be used in conditional and looping statement_
