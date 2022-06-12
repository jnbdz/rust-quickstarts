# Ownership and Borrowing | Tutorial | Rust | Quickstarts
- No garbage collection
    - Meaning the dev needs to manage the memory in his/her code
- This relates to *Stack vs Heap*
- Benefits
    - Faster! (often banchmarks faster than C++)
    - Parallel and Concurrent processing (is smuch easier)
    - **Safety** (less error or crashes)

## Example
```rust
#[allow(unused_variables)]
fn main() {
	let var_a = String::from("Howdy");
	let var_b = var_a;
	println!("{}", var_a);
}
```
This will cause this error: 
```
error[E0382]: borrow of moved value: `var_a`
 --> simplevarexample.rs:5:24
  |
3 |         let var_a = String::from("Howdy");
  |             ----- move occurs because `var_a` has type `String`, which does not implement the `Copy` trait
4 |         let var_b = var_a;
  |                     ----- value moved here
5 |         println!("{}", var_a);
  |                        ^^^^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
```
This occures because `var_a` has given it's ref to `var_b`.

### Stack vs Heap
To better understand you need to look into **Stack and Heap**: 

#### STACK
- Stack
    - Fast memory creation and retrieval
    - All about speed!!!
    - Memory management is very easy
    - The memory of the var is automatically recaptured by the program after the var gets out of *scope*
    - Rust uses *stack* by default for its memory needs
    - The data is actually store next too each other on the hardware
    - Scope: 
        - At the end of the `}` the scope ends (a var will be cleared out) (Also known as *LIFO*)
        - Even a *if* statement has a scope
        - **LIFO:** Last In, First Out
##### Example
```rust
fn main() {
    let stack_i8: i8 = 10;
    let stack_f32: f32 = 20.;
    let stack_bool: bool = true;
    let stack_char: char = 'a';
}
```
What do the vars have in common: 
- They all memory sizes that is known the Rust at compile time
- Fixed in size

> *NOTE:* Collections, vectors, and Strings cannot be *Stack*.
> Because they can change in size.
> Exception: Is the *Array* because it is fixed (it is known at compile time).

#### HEAP
- Flexible
- Memory that can grow in size (e.g.: Vectors, HashMap, Strings, etc)
- Runtime performance cost
- Memory can live beyond the *scope* that created it
- Memory is auto. recaptured when the last *owner* goes out of scope

##### Example
```rust
fn main() {
    let heap_vector: Vec<i8> = Vec::new(); // You can always use: `vec![4, 3];`
    let heap_string: String = String::from("Howdy"); // You can **never** allocate a `String` onto a Stack
    let heap_i8: Box<i8> = Box::new(30); // 
}
```

> **NOTE:** `String` is actually a collection of *u8* under the hood.
> Collections always need the flexibility to grow (explains why `String` uses *Heap*).


