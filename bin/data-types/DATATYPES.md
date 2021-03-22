# Datatypes in Rust

- Consider the following code :

```rust
fn main() {
    let guess = "12".parse().expect("Not a number!"); // Error occurs here, since no annotation is given for Rust to know to convert it into the required keyword
}
```

- This can be resolved by :

```rust
fn main() {
    // When a variable is named after { _ } underscore, the Rust compiler won't warn us about unused variable
    let _guess: i32 = "12".parse().expect("Not a number!"); // Compilers knows the desired datatype now
}
```

## Scalar Types

- A scalar type represents a single value.
- Rust has four primary scalar types and they are :
  1. Integers
  2. Floating-point numbers
  3. Booleans
  4. Characters

### Integer Type

- An integer is a number without a fractional component.
- The table represents the different types of integers which can be used in Rust :
  |Length|Signed|Unsigned|
  |:----|:----|:------|
  |8-bit|`i8`|`u8`|
  |16-bit|`i16`|`u16`|
  |32-bit|`i32`|`u32`|
  |64-bit|`i64`|`u64`|
  |128-bit|`i128`|`u128`|
  |arch |`isize` |`usize`|
- Signed `i<bit>` - Integers ranging b/w positive and negative numbers.
- Unsigned `u<bit>` - Integers ranging b/w positive numbers to 0.
- Each _signed variants_ can store numbers from $`-2^(n-1)\space to\space 2^(n-1) `$.
- _Unsigned variants_ can store numbers from $`0\space to\space 2^n - 1`$
- The `isize` and `usize` types depend on the kind of computer your program is running on :
  1. 64 bits if you’re on a 64-bit architecture
  2. 32 bits if you’re on a 32-bit architecture
- If you’re unsure which type to use, Rust’s defaults are generally good choices, and integer types default to `i32`, this type is generally the quickest, even on 64-bit systems.

### Floating-Point Type

- Here’s an example that shows _floating-point numbers_ in a rust code :

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

### Boolean Type

- Here’s an example that shows _boolean values_ in a rust code :

```rust
fn main() {
    let isLogged = true;
    let isCompleted: bool = false;
}
```

### Character Type

- Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
- `char` literals are specified with single quotes, as opposed to string literals, which use double quotes.
- For instance :

```rust
fn main() {
    let c = 'c';
    let x = 'x';
    let heart = '💖';
}
```

## Compound Types

- Compound types can group multiple values into one type.
- Rust has two primitive compound types :
  1. Tuples
  2. Arrays

### Tuples

- Tuple is a general way of grouping together a number of values with a variety of types into one compound type.
- They have a _fixed length_, **once declared they cannot grow or shrink in size**.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 5.5, 1);
}
```

- To print a tuple, we can do as :

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 5.5, 1);
    // {:?} formats the "next" value passed to a formatting macro, and supports anything that implements "Debug"
    println!("tup is: {:?}", tup); // tup is: (500, 5.5, 1)
    println!("tup is: {:#?}", tup);
    /*
     * Pretty prints
     * tup is: (500,
     * 5.5,
     * 1,
     * )
     */
}
```

- To get the individual values out of a tuple, we can use two methods :
  1. Patern matching (destructuring).
  2. Using period `.` followed by the index of the value

```rust
fn main() {
    // Pattern matching to destructure a tuple value
    let tup: (i32, f64, u8) = (500, 5.5, 1);
    let (x, y, z) = tup;
    // By using a period (.) followed by the index of the value
    let five_hundred = tup.0;
    let five_point_five = tup.1;
    let one = tup.2;
}
```

### Array Type

-Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

```rust
fn main() {
    let a = [1, 2, 3];
}
```

- We can also include type and size of an array as :

```rust
fn main() {
    /*
     * { i32 } is the type of each element.
     * After the semicolon, the number { 3 } indicates the array contains three elements.
     */
    let a: [i8; 3] = [1, 2, 3];
}
```

- To print an array :

```rust
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr: {:#?}", arr); // arr: [1, 2, 3, 4, 5]
}

```

- If you want to create an array that contains the same value for each element, you can specify the initial value, followed by a semicolon, and then the length of the array in square brackets.

```rust
fn main() {
    let arr = [3; 5];
    // Same as
    let same_arr = [3, 3, 3, 3, 3];
}
```

- The array index starts from `0` for an _array_ which is similar to other popular programming languages.
