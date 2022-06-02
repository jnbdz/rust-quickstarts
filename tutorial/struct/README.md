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
- **self** the calling instance

Example: 
```rust
struct Rectangle {
   width:u32,
   height:u32
}

impl Rectangle {
   fn area(&self)->u32 {
      self.width * self.height
   }
}

fn main() {
   // instanatiate the structure
   let small = Rectangle {
      width:10,
      height:20
   };
   //print the rectangle's area
   println!("Width is {} the height is {} and the area of the Rectangle is {}", small.width, small.height, small.area());
}
```

## Static method in `struct`
- Methods that are static are often used as utility reasons
- These methods exist before the struct is init
- Invoked using the struct name
- No need the instance to access them
- it does not take the **&self** parameter

Example: 
```rust
struct Point {
   x: i32,
   y: i32,
}

impl Point {
   //static method
   fn getInstance(x: i32, y: i32) -> Point {
      Point { x: x, y: y }
   }

   fn display(&self){
      println!("x={} y={}", self.x, self.y);
   }
}

fn main() {
   let p = Point::getInstance(10, 20);
   p.display();
}
```
