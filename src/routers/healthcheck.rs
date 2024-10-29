use axum::extract::State;
use axum::routing::get;
use std::sync::Arc;
use std::time::Instant;

pub struct AppInfo {
    pub uptime: Instant,
    pub host_port: String,
}

// and then we inspect the state like this
pub async fn healthcheck(info: State<Arc<AppInfo>>) -> String {
    // some logging because we're not insane
    println!("Info: hit on /healthcheck");
    format!(
        "api uptime: {}\nserving on: {}",
        info.uptime.elapsed().as_secs(),
        &info.host_port
    )
}

// all nested routers must be of the same type.
// Since rust can't infer the type this router has to be, because the .with_state
// is called in a different scope. So we have to type annotate the return like this
pub fn get_router(state: Arc<AppInfo>) -> axum::Router<Arc<AppInfo>> {
    axum::Router::new().route("/healthcheck", get(healthcheck))
}
