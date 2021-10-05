// Run with "cargo run --all-features --example tls_rustls"

use axum::{handler::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, world" }));

    axum_server::bind_rustls("127.0.0.1:3000")
        .private_key_file("examples/self-signed-certs/key.pem")
        .certificate_file("examples/self-signed-certs/cert.pem")
        .serve(app)
        .await
        .unwrap();
}