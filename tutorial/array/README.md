# Array | Rust | Tutorial
- Sequential memory blocks
- Static (cannot be resized)
- Array element is a memory block
- Elements in a array are called subscript/index
- Pupulating a element is called array initialization
- Elements can have their values changed but cannot be removed

## Init. of arrays
```rust
let arr:[i32;4] = [10, 30, 50, 80];
```
You can use `.len()` to get the array length.

You can also declare without array data type: `let arr = [1, 20, 60, 70];`

**Default values:**
```rust
let arr:[i32;4] = [-1;4];
```
This will output four `-1`.

**Array with for loop:**
```rust
for index in 0..4 {
    println!("Index is: {} & value is: {}", index, arr[index]);
}
```

Output... Will be four different values that will be shown.

`iter()` function: 
```rust
let arr:[i32;4] = [1, 25, 40, 50];
for val in arr.iter() {
  println!("Value is: {}", val);
}
```

**Mutable array:**
```rust
let mut arr:[i32;4] = [15, 40, 60, 76];
arr[1] = 0;
println!("{:?}",arr);
```
Output: 
```
[15, 40, 60, 76]
```

**Arrays passing in functions**
```rust
fn update(mut arr:[i32;3]){}
```

**Pass ref:**
```rust
let mut arr = [10, 20, 30];
update(&mut arr);
[...]
fn update(arr:&mut [i32;3]) {}
```

**Array declaration and constants:**
```rust
const N: usize = 37; 
let arr = [0; N];
```
Important! You cannot use a var `let` compared to `const`.

**Array slice**
```rust
let numbers = [1, 2, 20, 4];
let slice: &[i32] = &numbers[1..3];
println!("Slice: {:?}", slice);
```

Output: 
```
Slice: [1, 2]
```
