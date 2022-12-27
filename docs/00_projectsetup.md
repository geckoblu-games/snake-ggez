## Create a project

To get started, make a new folder and initialize a Rust project with `cargo init`.
If you don't have rust installed, install it with [rustup](https://rustup.rs/).

After running `cargo init`, you should have a folder structure like this:
```
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```

Cargo.toml contains the manifest info for your Rust crate (crate is Rustacean for package). Mine looks like this:

```toml
[package]
name = "snake-ggez"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```

The src/ directory is where all of your Rust code goes. main.rs comes with Hello World already written:
```rust
fn main() {
    println!("Hello, world!");
}
```

Run `cargo build` to build your project, and `cargo run` to run it.
If you just want to check for project for syntax and type errors, run `cargo check`.
It's a lot faster than `cargo build`.

## ggez basics

Adding ggez as a dependency is pretty simple.
Just go to the `dependencies` section of your Cargo.toml and add `ggez = "0.8"`

```toml
[package]
name = "snake-ggez"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
ggez = "0.8.1"
```

Run `cargo build` now to download and build ggez and all of its dependencies. It might take a few minutes.

The behavior of the program hasn't actually changed at all yet. `cargo run` will still just output "Hello, world!"
