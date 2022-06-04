# Async | Rust | Tutorial
- Also known as: *concurrent programming model*
- Used for: 
    - web server
    - database
    - operating system
- They are multiple concurrent models: 
    - **OS threads** does not require changes to the programming model (easy to use). What is hard the synchronizing between threads, and the performance overhead is large. Massive IO-bound workloads do not work well because there isn't thread pools.
    - **Event-driven programming** with *callbacks* are good for performance. But it is very verbose and *non-leaner* control flow. It is hard to debug because the data flow and error propagation are hard to follow.
    - **Coroutines** are like threads so no need to change the programming model. Easy to use. Supports large number of tasks. Down side is that it abstract low-level details that are important for systems programming and custom runtime implementors.
    - **The actor model** concurrent computation are devided into unitrs called actors. They communicate through fallible message passing, much like in distributed systems. The down side is that it leaves many practicle issues unanswered, such as flow control and retry logic.


    

> NOTE: Concurrent programming is less mature and "standardized" than regular, sequential programming. 

## Resources
- [Why Async? - Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html)
