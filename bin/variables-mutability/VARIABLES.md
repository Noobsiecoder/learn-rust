# Variables in Rust

- Consider the following code :

```rust
fn main() {
 let x = 5;
 println!("The value of x is {}", x); // 5
 x = 12; // Error occures here, won't compile
 println!("The value of x is: {}", x);
}
```

- In Rust, all variables are considered to be immutable.
- To change them during run-time, we use the keyword _"mut"_ :

```rust
fn main() {
 let mut x = 5;
 println!("The value of x is {}", x); // 5
 x = 12; // No error occurs
 println!("The value of x is: {}", x); // 12
}
```

## Difference b/w `const` and `mut`

- First case :

  1. First, you aren’t allowed to use mut with constants.
  2. Constants aren’t just immutable by default—they’re always immutable.
  3. Hence variables declared with `let` can be mutated.

- Second case :
  1. Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
  2. Constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

## Shadowing

- Consider the code :

```rust
fn main() {
    let x = 5;
    let x = 12; // Variable is being shadowed
    println!("The value is {}", x);
}
```

- From the following, the first variable is being shadowed.
- By doing this, we also change the data type during run time. For example :
```rust
fn main() {
    let y = 12; // Number
    let y = "John"; // String
    println!("The value of y is {}", y); // "John"

    let mut data = 12;
    data = "John"; // Cannot be converted to string
}

```
