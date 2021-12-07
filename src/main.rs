use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Server: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.description_lists = true;
    options.extension.superscript = true;
    options.extension.strikethrough = true;
    options.extension.footnotes = true;

    options.render.unsafe_ = true;

    let file = fs::read_to_string("./markdown/test.markdown").unwrap();
    let new: Vec<&str> = file.split("---").collect();
    println!("{:#?}", new);

    let converted = markdown_to_html(new[2], &options);
    Html(converted)
}
