# Functions in Rust

- Functions are pervasive in Rust code.
- the `main` function, which is the entry point of many programs. We have also seen `fn` keyword, which allows you to declare new functions.
- Rust code uses _snake case as the conventional style_ for function and variable names.

```rust
fn foo() {
    // User defined function
    println!("foo");
}

fn main() {
    // Main function
    foo(); // "foo"
}
```

## Function with parameter and argument

- In a function :
  1. **Parameters** are special variables that are part of a function’s signature.
  2. **Arguments** are the concrete values that are passed during the function call.

```rust
fn add_numbers(x: i32, y: i32) {
    // (x, y) are function parameters
    println!("{} + {} = {}", x, y, x + y); // 4 + 5 = 9
}

fn main() {
    add_numbers(4, 5); // (4, 5) are the function arguments
}
```

## Function bodies with statements and expressions

- _Statements_ are instructions that perform some action and do not return a value.
- _Expressions_ evaluate to a resulting value.
- Consider the following example :

```rust
fn main() {
    let x = 10;
    let y = {
        // New block scope is created, hence { x } is different from the one in the outer scope.
        let x = 5;
        x + 1 // ";" is excluded since it is an expression
    };
    println!("The value of x and y are {} and {}", x, y); // The value of x and y are 10 and 6
}
```

## Function with return values

```rust
/*
 * Expression returns the sum of { x } and { y }
 * "->" operator specifies the return type
 */
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let result: i32 = add_numbers(4, 5);
    println!("The result is {}", result); // The result is 9
}
```

## Conclusion

- In Rust, the function return is possible only if it is an expression. 
- An expression doesn't have an semicolon `;`.
