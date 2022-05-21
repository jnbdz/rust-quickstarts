# Turple | Rust | Tutorial
- Multiple values
- Different types can be used
- Fixed length (cannot grow nor shrink)

## Examples

```rust
fn main() {
   let tuple:(i32, f64, u8) = (-756, 5.8, 33);
   println!("{:?}", tuple);
}
```

Output: 
```
(-756, 5.8, 33)
```

To prin the indivdual values do this to your turple var: `turple.0` (zero is the index)

Here is how you pass it in a function: `fn print(x:(i32,bool,f64)) {}`

**Destructing**
```rust
let b:(i32, bool, f64) = (40, true, 9.9);
let (age, is_woman, a) = x;
```
