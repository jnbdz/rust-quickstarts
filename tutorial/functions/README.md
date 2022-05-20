# Constant | Rust | Tutorial

Examples: 
```rust
fn hello() {
   println!("hello");
}
```

Invoking: 
```rust
fn main(){
   hello();
}
```

Return: 
```rust
fn main(){
   println!("The value of P is {}", p());
}

fn p()->f64 {
   23.0/9.0
}
```
Ouput: 
```
The value of P is 2.5555555555555554
```

Params: 
```rust
fn main(){
   let num:i32 = 10;
   mutate(num);
   println!("The value of num is: {}", num);
}

fn mutate(mut param_num: i32) {
   param_num = param_num*0;
   println!("param_num value is: {}", param_num);
}
```

Output: 
```
param_num value is: 0
The value of num is: 10
```

By reference: 
```rust

```

Output: 
```

```
