# Tutorial

we don't have to add anything else for this step!

## How To

Before diving head first into the extractors, let's build something to extract!

Our health route is cool and all, but we can do better.
Let's create a struct to store some relevant info to be returned
by our healthcheck route:

```rust
pub struct AppInfo {
    pub uptime: Instant,
    pub host_port: String,
}
```

Much better! Now our healthcheck route can return some relevant info.

Since we're not fans of raw inlining functions, we extract the route's logic
into a nice separate function, such as:

```rust
pub async fn healthcheck(info: State<Arc<AppInfo>>) -> String {
    // some logging because we're not insane
    println!("Info: hit on /healthcheck");
    format!(
        "api uptime: {}\nserving on: {}",
        info.uptime.elapsed().as_secs(),
        &info.host_port
    )
}
```

The function is cool and all, but... how do we get to read some instance of
an AppInfo struct?
Well, that's where the good 'ol [Axum extranctors](https://docs.rs/axum/latest/axum/extract/index.html) come into play! More specifically the [State extractor](https://docs.rs/axum/latest/axum/extract/struct.State.html)!

To add a state we have to:

1: Create an object that will be our state, in our case,
an instance of an AppInfo struct. The object must implement Clone!

```rust
    let app_info = Arc::new(AppInfo {
        uptime: Instant::now(),
        host_port: host_port.clone(),
    });
```

2: Add the state into our Router using the [with_state](https://docs.rs/axum/latest/axum/routing/struct.Router.html#method.with_state) method.

3: We nest the router that has a healthcheck route into our bigger Router.
If you've ever done some express JS, you'll be familiar with this.
If not, picture that our application router is composed of smaller routers,
each representing a REST resource.

```rust
    let app = axum::Router::new()
        .nest("/", get_router(app_info.clone()))
        // we call .with_state to add a state to this router
        .with_state(app_info);
```
