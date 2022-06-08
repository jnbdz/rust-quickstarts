# Rust | Tutorial
In this directory I put my notes on Rust with code examples.

## Quick notes
- `'static` or `'` following any letters it means: lifetime (https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) (https://stackoverflow.com/questions/22048673/what-are-the-identifiers-denoted-with-a-single-apostrophe)
- `<T>` or `fn something<T>(list: &[T]) -> T {`-> Generic data type (https://doc.rust-lang.org/book/ch10-01-syntax.html)
- `dyn` -> Is a *keyword* to dynamically dispatch calls to methods associated to a `Trait` (unsure) (https://doc.rust-lang.org/std/keyword.dyn.html)
- `unsafe {` -> This might be used when you are calling functions that might not exist or cannot be validated by Rust at compile time. It is often used for more advance sys calls. (https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- `Box<T>` -> https://doc.rust-lang.org/book/ch15-01-box.html

Note: 
> Cargo is the package manager for Rust.
## Introduction
Advantages of Rust: 
- Write secure code
- Write multi-threaded code

Goals: 
- Safety
- Speed
- Concurrency

For: 
- High-level programs
- Hardware-specific programs

Note:
> Supported for Web Assembly (WASM)

### Performance
It doesn't have a Garbage Collector (GC) by design. This improves performance at runtime.

### Memory safety at compile time
- Dangling pointers
- Buffer overruns
- Memory leaks

### Multi-threaded applications
Safety rules provide concurrency without data races.

## Installation
### Windows
Intall [rustup](https://www.rust-lang.org/tools/install).

> Follow the instructions in the terminal

Following the installation on Windows you can find the related files at this location: `C:\Users\{user}\.cargo\bin`.

You should see a bunch of files.

#### Verify the intallation on Windows
```powershell
C:\Users>cargo -V
```
### Linux or Mac OS X
```bash
curl https://sh.rustup.rs -sSf | sh
```
To have the bin needed (this is where Cargo stores): 
```bash
source $HOME/.cargo/env
```
OR
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Podman
I created a BASH script with this command in it in mydotfiles repo.
```bash
podman run --rm -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.60.0 ${@}
```

## Resources
- [Rust Sandbox | bradtraversy | GitHub](https://github.com/bradtraversy/rust_sandbox) - Great basic example to learn from
