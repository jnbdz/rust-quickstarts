# Data Type | Rust | Tutorial
## Naming rules
* Must begin with: 
    - Letters
    - Underscore
* Vars can only compose of: 
    - letter
    - digits
    - underscore
* Upper and lowercase letters
* Case-sensitive

## Syntax
- Data types are optional.
- It is inferred by the value assigned.

```rust
let var_name = some_value;
let another_var_name:dataType = value;
```

## Immutable
- Read only. The value cannot be changed after a value is bound to a var.

Here is an example of an error: 
```
error[E0384]: re-assignment of immutable variable `fees`
 --> main.rs:6:3
   |
 3 | let fees = 25_000;
   | ---- first assignment to `fees`
...
 6 | fees=35_000;
   | ^^^^^^^^^^^ re-assignment of immutable variable

error: aborting due to previous error(s)
```

## Mutable
Adding `mut` infront of a var makes it mutable.

```rust
let mut var:i32 = 15_000;
var = 19_000;
```
In this example `var` will now be equal to `19000`.
