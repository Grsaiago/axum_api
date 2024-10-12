use std::env::VarError;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: &str = "8080";

fn get_host_port() -> String {
    // we didn't need this function.
    // This is only here for a matter ot 'separation of concerns'

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

    let port = match std::env::var("SERVER_PORT") {
        Ok(port) => port,
        Err(err) => {
            match err {
                VarError::NotPresent => {
                    eprintln!("Warn: SERVER_PORT not setup, fallback to {}", DEFAULT_PORT);
                }
                VarError::NotUnicode(_) => {
                    eprintln!(
                        "Warn: SERVER_PORT contains non unicode data, fallback to {}",
                        DEFAULT_PORT
                    );
                }
            }
            DEFAULT_PORT.to_string()
        }
    };
    format!("{host}:{port}")
}

#[tokio::main]
async fn main() {
    // This loads the env files from the .env file in the root dir
    let _ = dotenvy::dotenv();

    // this checks the host_port and then tries to bind the TcpSocket
    let host_port = get_host_port();
    let listener = match tokio::net::TcpListener::bind(&host_port).await {
        Ok(binded_listener) => binded_listener,
        Err(err) => {
            eprintln!("Error: Failed to bind on {host_port}: {err}");
            return;
        }
    };

    // This creates the router
    let app = axum::Router::new().route(
        "/healthcheck",
        axum::routing::get(|| async { "We're up and running!" }),
    );

    println!("Info: Serving on {}", &host_port);
    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("Error: axum::serve: {}", err);
    }
}
