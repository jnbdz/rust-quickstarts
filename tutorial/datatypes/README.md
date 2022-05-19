# Data Type | Rust | Tutorial

## Declare a variable
To declare vars you need: `let` keyword.

Type System verifies: 
- Validity of the value before they are stored or manipulated by the program.
- Makes sure that the code behaves as expected.
- Richer code hinting.
- Helps automate documentation.

Statically typed language.

Every value is of a certain type.

It infers automatically the data type based on the value assigned to it.

```rust
fn main() {
   let string = "Hello World!";    	// string type
   let float = 8.4;                 	// float type
   let boolean = true;          	// boolean type
   let icon_char = '♥';                 //unicode character type

   println!("String:{}", string);
   println!("Float:{}", float);
   println!("Is it true:{}", boolean);
   println!("Icon:{}", icon_char);
}
```

