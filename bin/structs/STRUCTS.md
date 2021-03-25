# Structs in Rust

- A _struct_, or _structure_, is a custom data type that lets you name and package together multiple related values that make up a meaningful group.
- It is another user defined data type available in Rust that allows us to combine data items of different types, including another _structure_.
- A _structure_ defines data as a key-value pair.
- For example -

  ```rust
  struct User {
      username: String,
      email: String,
      active: bool,
  }
  ```

## Basic functions using structs

- To print selective value from the key-value pair in a structure, we can use the dot (`.`) operator.

  ```rust
  struct User {
      // { _ } is prefixed before the variable name to indicate that the variable wouldn't be used during run-time
      _username: String,
      email: String,
      _active: bool,
  }

  fn main() {
      let user = User {
          _username: String::from("John Doe"),
          email: String::from("johndoe@mail.com"),
          _active: true,
      }; // Data is immutable
      println!("{}", user.email); // johndoe@mail.com
  }
  ```

- It becomes tedios to always craete `user` more than once. We can use a function :

  ```rust
  struct User {
      username: String,
      email: String,
      active: bool,
  }

  // User defined function to store data as a struct and return it
  fn build_user(username: String, email: String) -> User {
      User {
          username: username,
          email: email,
          active: true,
      }
  }

  fn main() {
      let user1 = build_user(String::from("John Doe"), String::from("johndoe@mail.com"));
      let user2 = build_user(String::from("Jane Doe"), String::from("janedoe@mail.com"));
      let user3 = build_user(String::from("Mike Doe"), String::from("mikedoe@mail.com"));
      println!("{}", user1.email); // johndoe@mail.com
      println!("{}", user2.username); // Jane Doe
      println!("{}", user3.active); // true
  }
  ```

- If the parameters are same as the key name in the struct, we can use the _field init shorthand syntax_

  ```rust
  struct User {
      username: String,
      email: String,
      active: bool,
  }

  fn build_user(username: String, email: String) -> User {
      User {
          // Field init shorthand syntax being used for { username } and { email }
          username,
          email,
          active: true,
      }
  }

  fn main() {
      let mut user1 = build_user(String::from("John Doe"), String::from("johndoe@mail.com"));
      let user2 = build_user(String::from("Jane Doe"), String::from("janedoe@mail.com"));
      let user3 = build_user(String::from("Mike Doe"), String::from("mikedoe@mail.com"));
      user1.email = String::from("notjohndoe@mail.com");
      println!("{}", user1.email); // notjohndoe@mail.com
      println!("{}", user2.username); // Jane Doe
      println!("{}", user3.active); // true
  }
  ```

- To create instances from other instances by using _struct update syntax_ :

  ```rust
  struct User {
      // { _ } is prefixed before the variable name to indicate that the variable wouldn't be used during run-time
      _username: String,
      email: String,
      _active: bool,
  }

  fn build_user(_username: String, email: String) -> User {
      User {
          // Field init shorthand syntax being used for { username } and { email }
          _username,
          email,
          _active: true,
      }
  }

  fn main() {
      let mut user1 = build_user(String::from("John Doe"), String::from("johndoe@mail.com"));
      let user2 = User { ..user1 }; // Struct update syntax
      user1.email = String::from("notjohndoe@mail.com");
      println!("{}", user1.email); // notjohndoe@mail.com
      println!("{}", user2.email); // johndoe@mail.com
  }
  ```

- The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

## Using Tuple Structs without Named Fields to Create Different Types

- You can also define structs that look similar to tuples, called _tuple structs_.
- Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.
- Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples.
- For example :

  ```rust
  /*
  * { #[...] } is ant attribute
  * { derive(Debug) } tells the compiler to auto-generate a suitable implementation of the Debug trait, which provides the result of {:?}
  *
  */
  #[derive(Debug)]
  struct Point(i32, i32, i32);

  fn main() {
      let origin = Point(0, 0, 0);
      println!("{:?}", origin); // Point(0, 0, 0)
  }
  ```

## Example using struct

- Consider the following code :

  ```rust
  fn area(width: u32, height: u32) -> u32 {
      width * height
  }

  fn main() {
      let width = 30;
      let height = 50;
      println!(
          "The area of the rectangle is {}*{} = {}",
          width,
          height,
          area(width, height)
      ); // The area of the rectangle is 30*50 = 1500
  }
  ```

- The `area(width: u32, height: u32)` is supposed to calculate the area of one rectangle, but the function we wrote has two parameters.
- The parameters are related, but that’s not expressed anywhere in our program.
- It would be more readable and more manageable to group width and height together.
- Refactoring with _tuple_ :

  ```rust
  // { dimensions } is a tuple
  fn area(dimensions: (u32, u32)) -> u32 {
      dimensions.0 * dimensions.1
  }

  fn main() {
      let rect = (30, 50);
      println!(
          "The area of the rectangle is {}*{} = {}",
          rect.0,
          rect.1,
          area(rect)
      ); // The area of the rectangle is 30*50 = 1500
  }
  ```

- Refactoring with _structs_ :

  ```rust
  // Struct with width and height as variable
  struct Rectangle {
      width: u32,
      height: u32,
  }

  // Function to calculate the area of rectangle by taking ownership of { Rectangle } struct
  fn area(rectangle: &Rectangle) -> u32 {
      rectangle.width * rectangle.height
  }

  fn main() {
      let rect = Rectangle {
          width: 20,
          height: 40,
      };
      println!(
          "The area of the rectangle is {}*{} = {}",
          rect.width,
          rect.height,
          area(&rect) // Refernce is being sent to the function
      ); // The area of the rectangle is 20*40 = 800
  }
  ```

## Implementation in Rust struct

- The `impl` keyword is primarily used to define implementations on types.
- There are two types of implementations :
  1. Inherent implementation
  2. Trait implementation

### Inherent implementations

- They are standalone and are tied to a single concrete `self` type that is specified after the `impl` keyword.
- These implementations, _unlike standard functions, are always in scope_.
- For example :

  ```rust
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }

  fn main() {
      let rect = Rectangle {
          width: 20,
          height: 40,
      };
      println!(
          "The area of the rectangle is {}*{} = {}",
          rect.width,
          rect.height,
          rect.area() // Borrowing
      ); // The area of the rectangle is 20*40 = 800
  }
  ```

### Associated Functions

- Another useful feature of `impl` blocks is that we’re allowed to define functions within `impl` blocks that don’t take self as a parameter.
- These are called _associated functions_ because they’re associated with the struct.
- For example :

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
      // Function which doesn't take { &self } as a parameter
      fn square_dim(size: u32) -> Rectangle {
          Rectangle {
              width: size,
              height: size,
          }
      }
  }

  fn main() {
      let rect = Rectangle {
          width: 20,
          height: 40,
      };
      println!(
          "The area of the rectangle is {}*{} = {}",
          rect.width,
          rect.height,
          rect.area() // Borrowing
      ); // The area of the rectangle is 20*40 = 800
      let square = Rectangle::square_dim(30);
      println!("{:?}", square); // Rectangle { width: 30, height: 30 }
  }
  ```
