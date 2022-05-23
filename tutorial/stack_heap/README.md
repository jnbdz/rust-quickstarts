# Stack & Heap | Rust | Tutorial
A program's memory can be allocated with: 
- Stack
- Heap

## Stack
- Last in
- First out
- The size is known at compile time
- The size is fixed
- Note: Strings are _not_ stored in **Stack**; _they are_ stored in **Heap**

## Heap
- The size is unknown at compile time
- For dynamic data
- So the size can change whilst the program is running (life cycle)
- Memory is less organized compred to Stack

## Ownership
- Variable is also called **owner**
- All data stored has an **owner** attached to it
- Each piece of data only has one **owner** at a time
- Each variables points to a different memory location (always)

## Transfering ownership
- Values can be assigned from one variable to another
- Passing a value to a function
- The returning of a value in a function

### Assigning a value from one var to another
- Rust main selling point is its memory safety
- The memory safety works by tight control on who can use what and when.
- If you pass `let a = something; let a2 = a;` it throw an error **if you call on a instead of a2 because the memory is transfered**. Rust undertand that you are trying to get two owners to point to the same memory.
    - This is the same behaviour for when you use a function and pass a value in it. The **ownership is changed hands**.
- Exception for *primitive types*! You can have the same value with multiple variables.

## Borrowing
- Ownership can be trensferred temporaily to an entity and then returned to the original owner
- Achieved by passing a reference to the variable *(& var_name)*
- After the function executes the original variable regains ownership

Example: 
```rust
fn main() {
   let a = vec![10, 20, 30];
   print_a(&a);
   println!("Printing the value from main() a[0]={}", a[0]);
}

fn print_a(x:&Vec<i32>) {
   println!("Inside print_vector function {:?}", x);
}
```

## References that are mutable
- 
