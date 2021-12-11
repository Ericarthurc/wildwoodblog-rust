use askama::Template;
use axum::{
    body::{self, Full},
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, get_service},
    Router,
};
use comrak::{markdown_to_html, ComrakOptions};
use std::net::SocketAddr;
use std::{collections::HashMap, fs};
use tower_http::services::ServeDir;

#[derive(Debug, Default)]
struct Meta {
    file_name: String,
    title: String,
    date: String,
    tags: Vec<String>,
    series: String,
}

impl Meta {
    fn new(file: &str) -> Self {
        let mut meta = Meta::default();

        let raw_meta: Vec<String> = file
            .split("---")
            .nth(1)
            .unwrap()
            .split("\n")
            .map(|x| x.replace('\r', ""))
            .collect();

        for line in raw_meta {
            match line.split(":").nth(0).unwrap() {
                "file_name" => meta.file_name = line,
                "title" => meta.title = line.split(":").nth(1).unwrap().trim().to_string(),
                "date" => meta.date = line.split(":").nth(1).unwrap().trim().to_string(),
                "tags" => {
                    meta.tags = line
                        .split(":")
                        .nth(1)
                        .unwrap()
                        .split(",")
                        .map(|s| s.trim().to_owned())
                        .collect()
                }
                "series" => meta.series = line.split(":").nth(1).unwrap().trim().to_string(),
                _ => (),
            }
        }
        return meta;
    }
}

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
            // get(|| async { Redirect::permanent("/blog".parse().unwrap()) }),
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

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    markdown: String,
    meta: Meta,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body::boxed(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                ))))
                .unwrap(),
        }
    }
}

async fn root() -> impl IntoResponse {
    let mark = markdown_parser("test");
    let meta = meta_parser("test");
    let template = IndexTemplate {
        markdown: mark,
        meta,
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct BlogTemplate {
    markdown: String,
    meta: Meta,
}

async fn blog(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let blog = params.get("blog").unwrap();

    let mark = markdown_parser(blog);
    let meta = meta_parser(blog);
    let template = BlogTemplate {
        markdown: mark,
        meta,
    };
    HtmlTemplate(template)
}

fn get_file(file_name: &str) -> String {
    fs::read_to_string(format!("./markdown/{}.markdown", file_name)).unwrap()
}

fn markdown_parser(file_name: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.description_lists = true;
    options.extension.superscript = true;
    options.extension.strikethrough = true;
    options.extension.footnotes = true;
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.render.unsafe_ = true;

    let file = get_file(file_name);

    markdown_to_html(&file, &options)
}

fn meta_parser(file_name: &str) -> Meta {
    let file = get_file(file_name);

    Meta::new(&file)
}
