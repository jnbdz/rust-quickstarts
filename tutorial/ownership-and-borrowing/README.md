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

To better understand you need to look into **Stack and Heap**: 
- Stack
    - Fast memory creation and retrieval
    - All about speed!!!
    - 
