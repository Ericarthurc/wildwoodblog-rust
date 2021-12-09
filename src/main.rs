use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::net::SocketAddr;

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
    // options.extension.front_matter_delimiter = Some("---".to_owned());

    options.render.unsafe_ = true;

    let file = fs::read_to_string("./markdown/test.markdown").unwrap();
    // let new: Vec<&str> = file.split("---").collect();
    // println!("{:#?}", new);

    // let converted = markdown_to_html(new[2], &options);
    // Html(converted);

    let mark = markdown_to_html(&file, &options);

    println!("{}", mark);

    let meta = Meta::new(&file);

    Html(meta.title)
}
