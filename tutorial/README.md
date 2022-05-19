# Rust | Tutorial
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
podman run --rm -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.60.0 rustc ${@}
```
