# Slices | Rust | Tutorial
- Pointer to a block of memory
- Access portions of data stored in contiguous memory blocks
- For usage with arrays, vectors and strings
- Index numbers are used to access portions of data
- Size is determined at runtime
- A slice is a pointer to the actual data
- Passed by reference to functions (borrowing)
- Minimum is 0

Example: 
```rust
let c1 = &n1[4..9];
```
This will show the value of *4, 5, 6, 7, 8* indexes.

Example with a function: 
```rust
[...]
use_slice(&data[1..4]);
[...]
fn use_slice(slice:&[i32]) {[...]}
```

Example with a function that is mutable: 
```rust
use_slice(&mut data[1..4]);
[...]
fn use_slice(slice:&mut [i32]) {[...]}
```
