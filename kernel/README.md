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


