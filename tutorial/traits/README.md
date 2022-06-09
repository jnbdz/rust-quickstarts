# Traits | Tutorial | Rust | Quickstarts
- 


Basic example: 
```rust
pub trait Computer {
    fn get_answer(&self) -> u8;
}

// Unit `struct`
pub struct DeepThought;

// A `Trait` for `struct`
impl Computer for DeepThought {
    fn get_answer(&self) -> u8 {
        42
    }
}

fn main() {
    let c = DeepThought {};
    let _result = c.get_answer();
}
```

## Resources
- [Rust Linz, July 2021 - Rainer Stropek - Traits, not your grandparents' interfaces | YouTube](https://www.youtube.com/watch?v=B0fL3WmJZsc)
    - [Traits - Not Your Grandparents' Interfaces | Slides.com](http://slides.com/rainerstropek/rust-traits/fullscreen)
    - [Tic Tac Toe (example) | rust-samples | GitHub](https://github.com/rstropek/rust-samples/tree/master/tictactoe)
