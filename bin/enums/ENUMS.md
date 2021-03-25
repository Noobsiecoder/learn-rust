# Enumerations in Rust

- An enumerated type is a data type consisting of a set of named values called _elements, members, enumeral, or enumerators_ of the type.

## Defining an Enum

- Consider the code below :

  ```rust
  #[derive(Debug)]
  enum Language {
      _Java,
      _Python,
      JavaScript,
      _Rust,
  }

  fn main() {
      let nodejs = Language::JavaScript;
      println!("{:?}", nodejs); // JavaScript
  }
  ```

- The example declares a _enum_ which has variants as `_Java`, `_Python`, `JavaScript` and `_Rust`.

## Using struct and enum together

- The following example defines a structure `Person`.
- The field gender is of the type `Gender`; which is an _enum_ and can be assigned either `Male` or `Female` as value.
- For example :

  ```rust
  #[derive(Debug)]
  enum Gender {
      Male,
      Female,
  }

  struct Person {
      name: String,
      gender: Gender,
  }

  impl Person {
      fn build_user(name: String, gender: Gender) -> Person {
          Person { name, gender }
      }

      fn show_user(&self) {
          println!("Name: {}\nGender: {:?}", self.name, self.gender);
      }
  }

  fn main() {
      let user1 = Person::build_user("John Doe".to_string(), Gender::Male);
      let user2 = Person::build_user("Jane Doe".to_string(), Gender::Female);
      println!("1st User");
      user1.show_user();
      println!("2nd User");
      user2.show_user();
      /*
       * Output :-
       * 1st User
       * Name: John Doe
       * Gender: Male
       * 2nd User
       * Name: Jane Doe
       * Gender: Female
       */
  }
  ```

## Option Enum

- **Option** is a predefined _enum_ in the Rust standard library.
- This enum has two values − _Some data_ and _None_.
- Consider a code which checks if a number is even or odd :

  ```rust
  fn is_even(num: i32) -> bool {
      if num % 2 == 0 {
          true
      } else {
          false
      }
  }

  fn main() {
      let result = is_even(3);
      println!("Is 3 an even number? {}", result); // Is 3 an even number? false
  }
  ```

- This code can be written using _option enum_ :

  ```rust
  #[derive(Debug)]
  enum Option<T> {
      Some(T), // Returns some value
      None,    // Rust doesn't support { null } keyword, instead uses { None }
  }

  fn is_even(num: i32) -> Option<bool> {
      if num % 2 == 0 {
          Option::Some(true)
      } else {
          Option::None
      }
  }

  fn main() {
      let mut result = is_even(3);
      println!("Is 3 an even number? {:?}", result); // Is 3 an even number? None
      result = is_even(4);
      println!("Is 4 an even number? {:?}", result); // Is 3 an even number? Some(true)
  }
  ```

## Match Statement and Enum

- The match statement can be used to compare values stored in an enum.
- The following code compares programming languages :

  ```rust
  #[derive(Debug)]
  enum Language {
      Rust,
      _Cpp,
      _Python,
      _JavaScript,
  }

  fn match_language(language: Language) {
      match language {
          Language::Rust => {
              println!("You have given: {:?}", Language::Rust);
          }
          Language::_Cpp => {
              println!("You have given: {:?}", Language::_Cpp);
          }
          Language::_Python => {
              println!("You have given: {:?}", Language::_Python);
          }
          Language::_JavaScript => {
              println!("You have given: {:?}", Language::_JavaScript);
          }
      }
  }

  fn main() {
      match_language(Language::Rust); // You have given: Rust
  }
  ```

## Match with Option enum

```rust
#[derive(Debug)]
enum Option<T> { // <T> is the data type of { Option }
    Some(T), // Returns some value
    None,    // Rust doesn't support { null } keyword, instead uses { None }
}

fn is_even(num: i32) -> Option<bool> {
    if num % 2 == 0 {
        Option::Some(true)
    } else {
        Option::None
    }
}

fn print_output(num: i32) {
    match is_even(num) {
        Option::Some(_data) => println!("{} is an even number", num),
        Option::None => println!("{} is an odd number", num),
    }
}

fn main() {
    print_output(273687); // 273687 is an odd number
    print_output(171638630); // 171638630 is an even number
}
```

## Enums with data types

```rust
#[derive(Debug)]
enum User {
    Name(String), // String type
    Uid(u32),     // Unsigned 32 bit integer
}

fn main() {
    let user_name = User::Name(String::from("John Doe"));
    let user_uuid = User::Uid(713678163);
    println!("{:?}", user_name); // Name("John Doe")
    println!("{:?}", user_uuid); // Uid(713678163)
}
```
