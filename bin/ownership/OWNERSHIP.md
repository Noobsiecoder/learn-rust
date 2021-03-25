# Ownership in Rust

- Ownership is the breakout feature of Rust.
- It allows Rust to be completely **memory-safe** and **efficient**, while avoiding garbage collection.

## Stack and Heap

![memory_architecture](https://qph.fs.quoracdn.net/main-qimg-1a1dbbe8b343484a55f3ff386b9cb48a)

## Difference between `String` and `str` in Rust

- `String` is a growable, heap-allocated data structure.
- Whereas `str` is an immutable fixed-length string somewhere in memory.

## Ownership and scope

- There are 3 rules about ownership in Rust and they are :

  1. **Each value in Rust has a variable that’s called its owner.**
  2. **There can only be one owner at a time.** (_Most Important_)
  3. **When the owner goes out of scope, the value will be dropped.**

- Consider the following code :

  ```rust
  fn main() {
      // string literal
      let hello = "Hello!"; // Memory allocated at compilation time. It is stored somewhere in read-only memory and a pointer to that string is stored on the stack
      let _new_hello = hello;
      println!("{}", hello); // "Hello!"
  } // Both variable are invalid or they are out of scope from here
  ```

- Now see the following code :

  ```rust
  fn main() {
      // String Type
      let hello = String::from("Hello object!");
      let _new_hello = hello;
      println!("{}", hello); // Error occurs here
  }
  ```

- When using a string literal, Rust copies the data. But in the case of _String_ type error is produced during compilation.
- This is because, the data is being moved from `hello` to `_new_hello`. Hence the ownership of the string is moved.
- Rust blocks this. To overcome this problem, we can copy the data from `hello` to `_new_hello`.

  ```rust
  fn main() {
      // String Type
      let hello = String::from("Hello object!");
      let _new_hello = hello.clone(); // Clones data
      println!("{}", hello); // No error occurs during compilation
  }
  ```

## Ownership and functions

- Consider the code :

  ```rust
  fn foo(string: &str) {
      println!("{:p}", string); // Prints the same memory address of variable allocated
  }

  fn main() {
      // string literal
      let string = "Hello, World!";
      println!("{:p}", string); // Prints the memory address of variable allocated
      foo(string);
      println!("{:p}", string); // Prints the same memory address of variable allocated
  }
  ```

- From this example it is clear that there are three pointers pointing to the same string.
- Hence we can conclude that _the pointers are being copied but not the value itself_.
- Now check this code :

  ```rust
  fn foo(string: String) {
      println!("{:p}", string.as_ptr()); // Prints the same memory address of variable allocated
  }

  fn main() {
      // String type
      let string = String::from("Hello, World!");
      println!("{:p}", string.as_ptr()); // Prints the memory address of variable allocated
      foo(string);
      println!("{:p}", string.as_ptr()); // Error occurs here
  }
  ```

- Error occurs during compilation because `main()` transfers the ownership to `foo()`.
- If we try to clone :

  ```rust
  fn foo(string: String) {
      println!("{:p}", string.as_ptr()); // 0xbc22e0
  }

  fn main() {
      // String type
      let string = String::from("Hello, World!");
      println!("{:p}", string.as_ptr()); // 0xbc22c0
      foo(string.clone());
      println!("{:p}", string.as_ptr()); // 0xbc22c0
  }
  ```

- Memory address for the cloned `string` is different from the original.
- This is because cloning a data means cloning the value from a variable and allocating memory for it separately.
- To give access to `string` variable to `foo()` function :

  ```rust
  fn foo(string: String) -> String {
      println!("{:p}", string.as_ptr()); // 0xb83860
      return string;
  }

  fn main() {
      // String type
      let string = String::from("Hello, World!");
      println!("{:p}", string.as_ptr()); // 0xb83860
      let string = foo(string);
      println!("{:p}", string.as_ptr()); // 0xb83860
  }
  ```

## References and borrowing

- From the above code, `main()` function is still the owner of `string` variable.
- At the end of `foo()` scope, `string` will not be dropped from memory.
- Thus `main()` is still responsible for `string` space in memory.
- Thus we can use the concept of _borrowing_ :

  ```rust
  fn foo(string: &String) -> &String {
      println!("{:p}", string.as_ptr()); // 0x1020e0
      return string;
  }

  fn main() {
      // String type
      let string = String::from("Hello, World!");
      println!("{:p}", string.as_ptr()); // 0x1020e0
      let string = foo(&string);
      println!("{:p}", string.as_ptr()); // 0x1020e0
  }
  ```

- `main()` passes a reference of `string` into `foo()`.
- Then `foo()` returns a **String** type reference.
- This is indicated by `&` symbol.
- After then end of `foo()` scope, execution returns to it’s caller; `main()` and `string` is still valid.
- `foo()` doesn’t have to return ownership, because it was never given ownership, it only borrowed.

## Mutability

- Consider the code where the function is responsible for pushing another string value into the original string :

  ```rust
  fn foo(string: &mut String) {
      println!("{:p}", string.as_ptr()); // 0x8ab20
      string.push_str(" from foo()");
  }

  fn main() {
      // String type
      let mut string = String::from("Hello");
      println!("{}", string); // Hello
      println!("{:p}", string.as_ptr()); // 0x8ab20
      foo(&mut string);
      println!("{}", string); // Hello from foo()
  }
  ```

- Now, we can still ensure that only `main()` is the responsibly for deallocating string, while also allowing other functions to mutate string!

## Dangling references

- Dangling references are pointers to data that has been deallocated, for example :

  ```rust
  fn foo() -> &String {
      let string = String::from("Hello!");
      &string // Returning a String type
  }

  fn main() {
      let main_string = foo(); // Error occurs
  }
  ```

- `foo()` returns a reference to string.
- However, once `foo()` scope ends, the _memory for string is deallocated_, which means the reference will point to a invalid place in memory!
- But Rust prevents this from compilation.

## Conclusion

- From this we can understand that scalar type copy data automatically.
- `String` is a growable, heap-allocated data structure.
- Whereas `str` is an immutable fixed-length string somewhere in memory.
- **There can only be one owner at a time.** (_Most Important_)

## Resource
- For more info on Ownership in detail :
  1. [Visit thomascountz article on Ownership in Rust](https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b)
  2. [Official docs by Rust](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)