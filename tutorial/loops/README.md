# Loops | Rust | Tutorial
- `while`
- `loop`
- `for`

## Examples
```rust
fn main(){
   for x in 1..12 { // 12 is not inclusive
      if x==6 { // This will skip 6
         continue;
      }
      println!("The value of `x` is {}", x);
   }
}
```

```rust
let mut x = 0;
while x < 5 {
   x+=1;
}
```

```rust
let mut x = 0;
loop {
  x+=1;
  println!("x={}",x);

  if x==15 {
     break; // When executed it will end the loop
  }
}
```
