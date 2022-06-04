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
```rust
fn main() {
   match is_even(5) {
      Some(data) => {
         if data==true {
            println!("Even no");
         }
      },
      None => {
         println!("not even");
      }
   }
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
not even
```

## `match` and `enum` with data types
```rust
#[derive(Debug)]
enum User {
   ID(i32),
   Name(String)
}

fn main() {
   let u1 = User::Name(String::from("Jack Smith"));
   let u2 = User::ID(100);
   println!("{:?}", u1);
   println!("{:?}", u2);

   match u1 {
      User::Name(val) => {
         println!("{}", val);
      }
      User::ID(val) => {
         println!("{}", val);
      }
   }
}
```
Output: 
```
Name("Jack Smith")
ID(100)
Jack Smith
```
