# Tokio | Tutorial | Rust | Quickstarts
- [Tokio - An asynchronous Rust runtime](https://tokio.rs/)

Website description: 
> Build reliable network applications without compromising speed.
>
> Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing network applications. It gives the flexibility to target a wide range of systems, from large servers with dozens of cores to small embedded devices.

## Tokio - The stack
- Runtime
    > Including I/O, timer, filesystem, synchronization, and scheduling facilities, the Tokio runtime is the foundation of asynchronous applications.
- Hyper
    > An HTTP client and server library supporting both the HTTP 1 and 2 protocols.
- Tonic
    > A boilerplate-free gRPC client and server library. The easiest way to expose and consume an API over the network.
- Tower
    > Modular components for building reliable clients and servers. Includes retry, load-balancing, filtering, request-limiting facilities, and more.
- Mio
    > Minimal portable API on top of the operating-system's evented I/O API.
- Tracing
    > Unified insight into the application and libraries. Provides structured, event-based, data collection and logging.
- Bytes
    > At the core, networking applications manipulate byte streams. Bytes provides a rich set of utilities for manipulating byte arrays.
