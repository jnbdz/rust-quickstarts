# Kernel | Rust | Quickstarts

## Windows

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
- The runtime needs to be called before the entry point (unsure)
- The entry point is the first thing that's called in a kernel module
- `0` that is being returned in the example means **success**
- `extern "system"` is to make sur the correct register is used for the returned value

```ini
[lib]
path = "src/lib.rs"
crate-type = ["cdylib"]
``` 

```ini
[build]
target = "x86_64-pc-windows-msvc"

rustflags = [
    # Pre Link args
    "-Z", "pre-link-arg=/NOLOGO",
    "-Z", "pre-link-arg=/NXCOMPAT",
    "-Z", "pre-link-arg=/NODEFAULTLIB",
    "-Z", "pre-link-arg=/SUBSYSTEM:NATIVE",
    "-Z", "pre-link-arg=/DRIVER",
    "-Z", "pre-link-arg=/DYNAMICBASE",
    "-Z", "pre-link-arg=/MANIFEST:NO",

    # Post Link Args
    "-C", "link-arg=/OPT:REF,ICF",
    "-C", "link-arg=/ENTRY:driver_entry",
    "-C", "link-arg=/MERGE:.edata=.rdata",
    "-C", "link-arg=/MERGE:.rustc=.data",
    "-C", "link-arg=/INTEGRITYCHECK",
]
```
- This is adjusting the linker settings

```rust
let windows_kits_dir = get_windows_kits_dir().unwrap();
let km_dir = get_km_dir(&windows_kits_dir).unwrap();
let target = var("TARGET").unwrap();

let arch = if target.contains("x86_64") {
    "x64"
} else if target.contains("i686") {
    "x86"
} else {
    panic!("Only support x86_64 and i686!");
};

let lib_dir = km_dir.join(arch);
println!(
    "cargo:rustc-link-search=native={}",
    lib_dir.to_str().unwrap()
);
```

## Linux
