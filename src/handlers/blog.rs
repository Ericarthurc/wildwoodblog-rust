use std::collections::HashMap;

use askama::Template;
use axum::{extract::Path, response::IntoResponse};

use crate::{
    errors::AppError,
    parsers::{get_blog_index_vec, get_meta_and_markdown, meta::Meta},
};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path = "blog_index.html")]
struct BlogIndexTemplate {
    blog_index: Vec<Meta>,
}

pub async fn blog_index_handler() -> Result<impl IntoResponse, AppError> {
    let meta_vec = get_blog_index_vec().await?;
    let template = BlogIndexTemplate {
        blog_index: meta_vec,
    };
    Ok(HtmlTemplate(template))
}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    markdown: String,
    meta: Meta,
}

pub async fn blog_handler(
    Path(params): Path<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let blog = params.get("blog").unwrap();

    let (meta, mark) = get_meta_and_markdown(blog).await?;
    let template = BlogTemplate {
        markdown: mark,
        meta,
    };
    Ok(HtmlTemplate(template))
}
