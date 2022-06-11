# Lifetime elision | Tutorial | Rust | Quickstarts
- You need to understand Ownership and Borrowing **first**
- Lifetimes can be merged in various places (the compiler can infer a sensible default choice)
- Rust allows one and only one owner of memory
- Rust allows multiple references
- Lifetimes enforce a piece of memory is still valid for a reference


## Lifetime elision in functions
- Lifetime arguments can be *elided* in: 
    - [function item](https://doc.rust-lang.org/reference/types/function-item.html)
    - [function pointer](https://doc.rust-lang.org/reference/types/function-pointer.html)
    - [closure trait](https://doc.rust-lang.org/reference/types/closure.html)
