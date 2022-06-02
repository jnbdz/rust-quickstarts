# Enums | Rust | Tutorial
- Also known as *enumeration*
- Is a list of possible variants

Example: 
```rust
#[derive(Debug)]
enum Genders {
   Male,
   Female
}
fn main() {
   let male = Genders::Male;
   let female = Genders::Female;
   [...]
}
```

> Note: `#[derive(Debug)]` is an attribute used to suppress the error *trait std::fmt::Debug is not implemented for Genders*

## `struct` and `enum`

## `enum` - `Option`

## `match` statement with `enum`

## `match` with `Option`

## `match` and `enum` with data types

