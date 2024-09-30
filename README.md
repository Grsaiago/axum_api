# Tutorial

## Dependencies

We add dotenvy for .env management, and for Axum we need to add:
- Axum
- The Tokio runtime (here added with 'full' features make our lifes easier)
- Tower for middlewares

To add everything, just run:

```sh
cargo add dotenvy axum tokio tower -F tokio/full
```

## How To

The first line is just a call to ``dotenvy::dotenv()`` to load our env vars from the .env file.
We then initialize a [Tokio Tcp Listener](https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html).

```Rust
    let host = match std::env::var("SERVER_HOST") {
        Ok(host) => host,
        Err(err) => {
            match err {
                VarError::NotPresent => {
                    eprintln!("Warn: SERVER_HOST not setup, fallback to {}", DEFAULT_HOST);
                }
                VarError::NotUnicode(_) => {
                    eprintln!(
                        "Warn: SERVER_HOST contains non unicode data, fallback to {}",
                        DEFAULT_HOST
                    );
                }
            }
            DEFAULT_HOST.to_string()
        }
    };

```

There is A LOT to unpack in these few lines:

You can think of Rust matches like a powered up switch case!
In Rust, 'match' are expressions, which means that they return a value,
so the result of a match expression can be binded to a variable.

The following in Rust:

```Rust
let result_of_match = match some_result_returning_fn() {
    Ok(value) => value_if_ok,
    Err(err) => value_if_error
}
```

Can be thought of as something like this in C:

```c++
int result_of_match;
int return_of_some_result_returning_fn;

return_of_some_result_returning_fn = some_result_returning_fn();
if (return_of_some_result_returning_fn == some_error_value) {
    result_of_match = value_if_error;
} else {
    result_of_match = return_of_some_result_returning_fn
}
```

One last thing before we move on, the variables DEFAULT_HOST and DEFAULT_PORT
are just global consts setup like:

```Rust
const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: &str = "8080";
```

Phew! Unpacked a lot in here 😅

The next step is to properly bind a TCP socket to the aquired host:port:

```Rust
    let listener = match tokio::net::TcpListener::bind(&host_port).await {
        Ok(binded_listener) => binded_listener,
        Err(err) => {
            eprintln!("Error: Failed to bind on {host_port}: {err}");
            return;
        }
    };
```

We have a TcpListener setup according to our needs, now we have to create an axum [Router](https://docs.rs/axum/latest/axum/struct.Router.html) and specify which routes with which verbs execute which function, as such:

```Rust
    let app = axum::Router::new().route(
        "/healthcheck",
        axum::routing::get(|| async { "We're up and running!" }),
    );
```

The final step is to just ask axum to serve the newly created app,
using the specified listener:

```Rust
    if let Err(err) = axum::serve(listener, app).await {
        eprint!("Error on axum::serve: {}", err);
    }
```
