# Kernel | Rust | Quickstarts
When coding for the kernel you cannot run this: 
```rust
fn main() {
	println!("Hello World!");
}
```
The simple reason is because the kernel does not have a **consol**.

To start you need to use the *attribute* `#![no_std]` (this removes the standard lib): 
```rust
#![no_std]

fn main() {
	println!("Hello World!");
}
```
But this will throw errors.

```
error: cannot find macro `println` in this scope
 --> no_std_helloworld.rs:4:2
  |
4 |     println!("Hello World!");
  |     ^^^^^^^

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`

error: `#[panic_handler]` function required, but not found

error: aborting due to 3 previous errors
```

When it panics so we need to catch it (since they are no consol in the kernel).

```rust
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop{}
}

fn main() {
	println!("Hello World!");
}
```
- The `!` means nothing is returned

Now we need to create our own entry point: 
```rust
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop{}
}

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
	0 /* STATUS_SUCCESS */
}
```


