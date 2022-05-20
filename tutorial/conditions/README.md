# Conditions
- `if`
- `if else`
- `else if`
- `match` (similar to `switch`)

Examples: 
```rust
if num > 0 {
      println!("Allo!") ;
   }
```
```rust
if bool {
  [...]
} else {
  [...]
}
```
```rust
if bool1 {
   [...]
} else if bool2 {
   [...]
} else {
   [...]
}
```
```rust
fn main(){
   let code = "MH";
   let status = match code {
      "1a" => {println!("Found match for 1a"); "Status 1a"},
      "1b" => "Good",
      "1c" => "Bad",
      "1d" => "Ugly",
      _ => "Unknown"
   };
   println!("State name is {}",state);
}
```
