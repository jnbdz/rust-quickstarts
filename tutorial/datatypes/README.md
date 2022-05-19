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
| 6       | Arch*   | isize | usize  |

***arch**: The size will be derived from the *architecture* of the machine.

> 32 bits == x86
> 
> 64 bits == x64

Examples: 
```rust
fn main() {
   let result = 10;    // i32 by default
   let age:u32 = 20;
   let sum:i32 = 5-15;
   let mark:isize = 10;
   let count:usize = 30;
   println!("result value is {}",result);
   println!("sum is {} and age is {}",sum,age);
   println!("mark is {} and count is {}",mark,count);
}
```
Result: 
```
result value is 10
sum is -10 and age is 20
mark is 10 and count is 30
```

> If you give a float to *u32* it will throw an error.

### Integer range
Signed variant can store: -(2^(n-1) to 2^(n-1) -1

*n* => number of bits that the variants uses.

### Integer overflow
When the value exceeds.

Example of error: **− warning − literal out of range for u8**

### Float
Classified as: 
- f32 (single-precision)
- f64 (double-precision) (default)

Example: 
```rust
let result = 10.00;        //f64 by default
let interest:f32 = 8.35;
let cost:f64 = 15000.600;  //double precision
```

### Casting
There are no automatic casting.

Example of **mismatched type error**: 
```
error[E0308]: mismatched types
   --> main.rs:2:22
   |
 2 | let interest:f32=8;
   |    ^ expected f32, found integral variable
   |
   = note: expected type `f32`
      found type `{integer}`
error: aborting due to previous error(s)
```

### Number seperator
For long number you can do this: `70_000`

### Bool
Only two possible values... I know it's hard!

### Char
What is supported: 
- Numbers
- Alphabets
- Unicode
- Special characters

Unicode Scalar values range from **U+0000** to **U+D7FF** and **U+E000** to **U+10FFFF** inclusive.
