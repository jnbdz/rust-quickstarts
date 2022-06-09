# Smart Pointers | Tutorial | Rust | Quickstarts
- *Pointer* is a variable that contains an address in a memory
- A common pointer is by reference: [Understanding Ownership](https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html)
    - They are indicated by `&`
    - They borrow the value they point to
    - All they do and can do is point is reffering to data
    - No overhead
    - The most used
- *Smart pointers*
    - Act like a pointer
    - Have additional metadata and capabilities
    - Originies: C++
    - Other languages have it too
    - References are pointers that only borrow data
    - Many cases, *smart pointers* own the data they point to
- Examples of *smart pointers*:
    - `String`
    - `Vec<T>`

## `Box<T>`

**Sources:** 
- [The Box Smart Pointer in Rust | YouTube](https://www.youtube.com/watch?v=m76sRj2VgGo)
- [Using Box&lt;T&gt; to Point to Data on the Heap - The Rust Programming Language | rust-lang.org](https://doc.rust-lang.org/book/ch15-01-box.html)
