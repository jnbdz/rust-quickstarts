# Rust Multipart Async | Examples | Tokio | Tutorial | Rust | Quickstarts
[cetra3/mpart-async: Asynchronous Multipart Requests for Rust | GitHub](https://github.com/cetra3/mpart-async)

## Hyper Client Example
The breakdown follows.
```rust
use hyper::{header::CONTENT_TYPE, Body, Client, Request};
use hyper::{service::make_service_fn, service::service_fn, Response, Server};
use mpart_async::client::MultipartRequest;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //Setup a mock server to accept connections.
    setup_server();

    let client = Client::new();

    let mut mpart = MultipartRequest::default();

    mpart.add_field("foo", "bar");
    mpart.add_file("test", "Cargo.toml");

    let request = Request::post("http://localhost:3000")
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", mpart.get_boundary()),
        )
        .body(Body::wrap_stream(mpart))?;

    client.request(request).await?;

    Ok(())
}

fn setup_server() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Error>(service_fn(mock)) });
    let server = Server::bind(&addr).serve(make_svc);

    tokio::spawn(server);
}

async fn mock(_: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("")))
}
```
### Breakdown
```rust
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
```
- [`type`](https://doc.rust-lang.org/reference/items/type-aliases.html) - Also known as [Type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html). 
- [`Box<>`](https://doc.rust-lang.org/book/ch15-01-box.html) - Is a [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) for allocating values on the heap. [Examples - Box, stack and heap](https://doc.rust-lang.org/rust-by-example/std/box.html), [Box in std::boxed - Rust](https://doc.rust-lang.org/std/boxed/struct.Box.html)
    - [`dyn`](https://doc.rust-lang.org/std/keyword.dyn.html) - Is a keyword. For prefixing of a [trait object](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) type.
    - [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html) - Is a trait representing the basic expectations for error values, values of type `E` in [`Result<T, E>`](https://doc.rust-lang.org/std/result/enum.Result.html).
    - [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) - Types that can be transferred across thread boundaries.
    - [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html) - Types for which it is safe to share references between threads.
    - [`'static`](https://doc.rust-lang.org/std/keyword.static.html) - [Examples](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html), [Static items](https://doc.rust-lang.org/reference/items/static-items.html)

```rust
#[tokio::main]
```
- It is an Rust attribute
- [...]

## Warp Server Example
```rust
use warp::Filter;

use bytes::Buf;
use futures::stream::TryStreamExt;
use futures::Stream;
use mime::Mime;
use mpart_async::server::MultipartStream;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any()
        .and(warp::header::<Mime>("content-type"))
        .and(warp::body::stream())
        .and_then(mpart);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn mpart(
    mime: Mime,
    body: impl Stream<Item = Result<impl Buf, warp::Error>> + Unpin,
) -> Result<impl warp::Reply, Infallible> {
    let boundary = mime.get_param("boundary").map(|v| v.to_string()).unwrap();

    let mut stream = MultipartStream::new(
        boundary,
        body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining())),
    );

    while let Ok(Some(mut field)) = stream.try_next().await {
        println!("Field received:{}", field.name().unwrap());
        if let Ok(filename) = field.filename() {
            println!("Field filename:{}", filename);
        }

        while let Ok(Some(bytes)) = field.try_next().await {
            println!("Bytes received:{}", bytes.len());
        }
    }

    Ok(format!("Thanks!\n"))
}
```

```rust
use warp::Filter;
```

```rust
use bytes::Buf;
```

```rust
use futures::stream::TryStreamExt;
```

```rust
use futures::Stream;
```

```rust
use mime::Mime;
```

```rust
use mpart_async::server::MultipartStream;
```

```rust
use std::convert::Infallible;
```

```rust
#[tokio::main]
```

```rust
async fn main() {
[...]
```
