# Constant | Rust | Tutorial
The value cannot change.
```rust
const VAR_NAME:dataType = val;
```
## Naming rules
- Usually uppercase
- Case-sensitive
- Do not use `let` keyword for `const`

## Const vs Vars
- Const must have a data type but not var.
- With var you can use the keyword `mut` if you want it to be mutable. You do not have this option with constants.
- Const cannot be srt to a constant expression or the result of a function call or any value computed at runtime.
- Const can be declared in any scope like global scope.
- Vars you can override their previous value with the keyword `let` (it's called *shadowing*) (you can use diff. data types).
    ```rust
    let var = 100.00;
    let var = 15.00;
    ```
- Const cannot be *shadowed* like variables. If you do this you will get an error.
