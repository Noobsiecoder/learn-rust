# Cargo in rust

- Cargo is Rust’s build system and package manager.
- Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call libraries your code needs dependencies).

## Check version

```bash
cargo --version
```

## Creating a project with Cargo

- To create a new boilerplate of rust program, we run :

```bash
# Git files won’t be generated if you run cargo new within an existing Git repository
cargo new <app_name>
cd <app_name>
```

## Building and running a Cargo Project

- To build :

```bash
# This will build and store the ".exe" file in "/target/debug/"
cargo build
```

- To run the `<file_name>.exe` file :

```bash
./target/debug/<file_name>.exe
```

- To compile and run :

```bash
cargo run
```

- Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable :

```bash
cargo check
```

- If building for production for better optimized app usage, use :

```bash
cargo build --release
```

---

### Code

[Click here](src/main.rs)

