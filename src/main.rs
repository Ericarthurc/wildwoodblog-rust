use axum::{
    http::StatusCode,
    response::Redirect,
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use crate::handlers::blog::{blog, root};

mod handlers;
mod parsers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(get_service(ServeDir::new("./public")).handle_error(
            |error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            },
        ))
        .route(
            "/",
            get(|| async { Redirect::to("/blog".parse().unwrap()) }),
        )
        .route("/blog", get(root))
        .route("/blog/:blog", get(blog))
        .route("/series", get(root))
        .route("/series/:series", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Server: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
