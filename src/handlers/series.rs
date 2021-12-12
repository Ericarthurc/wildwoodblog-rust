use std::collections::HashMap;

use askama::Template;
use axum::{extract::Path, response::IntoResponse};

use crate::parsers::{
    meta::Meta,
    parsers::{markdown_parser, meta_parser},
};

use super::HtmlTemplate;

#[derive(Template)]
#[template(path = "series_index.html")]
struct SeriesIndexTemplate {}
pub async fn series_index_handler() -> impl IntoResponse {
    let template = SeriesIndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "series.html")]
struct SeriesTemplate {}
pub async fn series_handler(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let template = SeriesTemplate {};
    HtmlTemplate(template)
}
