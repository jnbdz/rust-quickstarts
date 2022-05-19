# String | Rust | Tutorial
String classifications: 
- String Literal (`&str`)
- String Object (`String`)

## Literal
- When the value is know at compiled time.
- Hardcoded in the variable (example: `let name = "Bob Smith";`).
- Found in `std::str`.
- Also known as string *slices*.
- Static by default.

```rust
let name:&str = "Bob Smith";
let location:&str = "Toronto";
println!("Name is : {} location :{}", name, location);
```

To force *static*: 
```rust
let name:&'static str = "Bob Smith";
```

## Object
- Provided by the Standard Library.
- Not apart of the core language.
- Public struture (`pub struct String`).
- Growable collection (it can expand in size).
- Mutable.
- UTF-8 encoded.
- String object is allocated in the heap.
- At runtime String object can be used to represent string values.

### Syntax 
Create an empty string object: 
```rust
String::new()
```
Pass as a param to the `from()` method will be used as default value for the string object.

Example: 
```rust
let empty_string = String::new();
println!("length is {}", empty_string.len());

let content_string = String::from("Some string");
println!("length is {}", content_string.len());
```

### Common methods for string object
| Sr.No. | Method      | Signature                       | Description                             |
|--------|-------------|---------------------------------|-----------------------------------------|
| 1      | new()       | pub const fn new() → String     | New empty string |
| 2      | to_strin()  | fn to_string(&self) → String    | Coverts given value to a String |
| 3      | replace()   | pub fn replace<'a, P>(&'a self, from: P, to: &str) → String | Matches are replace with another string |
| 4      | as_str()    | pub fn as_str(&self) → &str     | Extracts a string slice containing the enire string |
| 5      | push()      | pub fn push(&mut self, ch: char) | Appends the given char to the end of this String |
| 6      | push_str()  | pub fn push_str(&mut self, string: &str) | Appends a given string slice onto the end of this String |
| 7      | len()       | pub fn len(&self) → usize       | Returns the length of this String, in bytes |
| 8      | trim()      | pub fn trim(&self) → &str       | Returns a string slice with leading and trailing whitespace removed |
| 9      | split_whitespace() | pub fn split_whitespace(&self) → SplitWhitespace | Splits a string slice by whitespace and returns an iterator |
| 10     | split()     | pub fn split<'a, P>(&'a self, pat: P) → Split<'a, P> , where P is pattern can be &str, char, or a closure that determines the split. | Returns an iterator over substrings of this string slice, separated by characters matched by a pattern |
| 11     | chars()     | pub fn chars(&self) → Chars | Returns an iterator over the chars of a string slice |

### Examples
`new()`
```rust
let mut name = String::new();
name.push_str("Bob");
println!("{}", name);
```
Output: `Bob`

`to_string()`
```rust
let name = "Bob Smith".to_string(); 
println!("{}", name);
```
Output: `Bob Smith`

`replace()`
```rust

```
