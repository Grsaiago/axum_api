use axum::extract::State;
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
