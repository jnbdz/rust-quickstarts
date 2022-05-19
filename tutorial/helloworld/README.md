# HelloWorld | Rust | Tutorial
## Macros
The function you need to call to print the famous *Hello World!* message: 
```rust
println!()
```
It's important to note that this is a *macro*.

- It is for meta-programming.
- They look like functions except it ends with a *!*.
- It doesn't generate a function call.
- They expand into source-code.
- That expended code will be compiled with the rest of the code.
- They provide more runtime features to a program unlike functions.

```rust
println!(); // prints just a newline
println!("hello ");//prints hello
println!("format {} arguments", "some"); //prints format some arguments
```

## Comments
```rust
//
/* */
```
