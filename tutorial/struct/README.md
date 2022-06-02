# Struct | Rust | Tutorial
- Also known as *structure*
- Every field needs to be set with a data type

Example: 
```rust
struct Person {
   name:String,
   family_name:String,
   age:u32
}

fn main() {
    let person = Person {
      family_name:String::from("Smith"),
      name:String::from("Bob"),
      age:56
   };
}
```

You can add `mut` so that you can modify it later.

Example: 
```rust
person.age = 40;
```

Passing a struct to a function: 
```rust
fn display(p:Employee){[...]}
```

Returning it in a function: 
```rust
fn display(p:Person)->Person {[...]}
```

## Method in `struct`
- Like functions but in a `struct`
- Declared with the keyword **fn**
- Must be added in the `struct` block
- For the methods declared outside of the `struct` block it will use the **impl** keyword
- For _all_ methods the first param it will be **self**

