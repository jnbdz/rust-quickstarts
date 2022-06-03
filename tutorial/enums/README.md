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
- `derive` attribute auto creates the impl to make the `struct` printable with `fmt::Debug`
Example: 
```rust
#[derive(Debug)]
enum Gender {
   Male,
   Female
}

#[derive(Debug)]
struct Person {
   name:String,
   gender:Gender
}

fn main() {
   let p = Person {
      name:String::from("Toronto"),
      gender:Gender::Female
   };
   [...]
}
```

## `enum` - `Option`
- `Option` is a predefined `enum` in Rust standard lib
- Use `None` instead of `null` keyword

> Note: Rust does not support the `null` keyword

The `Option` `enum`: 
```rust
enum Option<T> {
   Some(T), // Used to return a value
   None     // Used to return null, as Rust does not have `null`
}
```
- `<T>` represent any data type
- When data can be return it returns `Some(T)`

Example: 
```rust
fn main() {
   let result = is_even(3);
   println!("{:?}", result);
   println!("{:?}", is_even(30));
}

fn is_even(no:i32)->Option<bool> {
   if no%2 == 0 {
      Some(true)
   } else {
      None
   }
}
```
Output: 
```
None
Some(true)
```

## `match` statement with `enum`
- Used to compare values in a `enum`
Example: 
```rust
enum Gender {
    Female,
    Male
}

fn print_gender(g:Gender) {
    match g {
        Gender::Female => {
            println!("It's a female!");
        }
        Gender::Male => {
            println!("It's a male!");
        }
    }
}

fn main() {
    print_gender(Gender::Female);
    print_gender(Gender::Male);
}
```
Output: 
```
It's a female!
It's a male!
```

## `match` with `Option`


## `match` and `enum` with data types

