//! axum-server is a hyper server implementation designed to be used with axum framework.
//!
//! # Example
//!
//! Axum "Hello, World!" example can be run like:
//!
//! ```rust,no_run
//! use axum::{
//!     handler::get,
//!     Router,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     let app = Router::new().route("/", get(|| async { "Hello, World!" }));
//!
//!     axum_server::bind("127.0.0.1:3000")
//!         .serve(app)
//!         .await
//!         .unwrap();
//! }
//! ```

#![forbid(unsafe_code)]

mod server;

pub use server::{bind, Server};

#[cfg(feature = "rustls")]
pub use server::bind_rustls;
