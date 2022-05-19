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
Note: 
`println!` macro has two args: 
- `{ }` is a placeholder.
- Var name or a constant.

## Scalar Types
Types: 
- Integer
- Float
- Bool
- Char

### Integer
Whole number.

Classified as: Signed and Unsigned.

Can be: Positive or negative.

Unsigned: Can only have positive value.

Interger types: 
| Sr.No.  | Size | Signed | Unsigned |
|---------|------|--------|----------|
| 1       | 8 bit | i8    | u8       |
| 2       | 16 bit | i16  | u16      |
| 3       | 32 bit | i32  | u32      |
| 4       | 64 bit | i64  | u128     |
| 5       | 128 bit | i128 | u64     |
| 6       | Arch    | isize | usize  |


