use std::io::Error;

use super::meta::Meta;
use async_fs;
use comrak::{markdown_to_html, ComrakOptions};

pub async fn get_file(file_name: &str) -> Result<String, Error> {
    async_fs::read_to_string(format!("./markdown/{}.markdown", file_name)).await
}

pub async fn markdown_parser(file_name: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.description_lists = true;
    options.extension.superscript = true;
    options.extension.strikethrough = true;
    options.extension.footnotes = true;
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.render.unsafe_ = true;

    let file = get_file(file_name).await.unwrap();

    markdown_to_html(&file, &options)
}

pub async fn meta_parser(file_name: &str) -> Meta {
    let file = get_file(file_name).await.unwrap();

    Meta::new(&file)
}
