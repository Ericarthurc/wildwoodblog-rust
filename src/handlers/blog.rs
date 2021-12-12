use std::collections::HashMap;

use askama::Template;
use axum::{extract::Path, response::IntoResponse};

use crate::parsers::{
    get_blog_index_vec, get_meta_and_markdown,
    meta::Meta,
    parsers::{markdown_parser, meta_parser},
};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path = "blog_index.html")]
struct BlogIndexTemplate {
    blog_index: Vec<Meta>,
}

pub async fn blog_index_handler() -> impl IntoResponse {
    let meta_vec = get_blog_index_vec().await;
    let template = BlogIndexTemplate {
        blog_index: meta_vec,
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    markdown: String,
    meta: Meta,
}

pub async fn blog_handler(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let blog = params.get("blog").unwrap();

    let (meta, mark) = get_meta_and_markdown(blog).await;
    let template = BlogTemplate {
        markdown: mark,
        meta,
    };
    HtmlTemplate(template)
}
