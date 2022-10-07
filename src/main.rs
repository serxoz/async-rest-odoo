pub mod odoo;
pub mod views;

use crate::views::products::get_products;
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // routing
    let app = Router::new()
        .route("/", get(root))
        .route("/products", get(get_products));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic view...
async fn root() -> &'static str {
    "Nothing to see here ;)"
}
