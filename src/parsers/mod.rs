use async_fs;
use std::fs;
use std::io::Error;

use self::meta::Meta;
use self::parsers::{markdown_parser, meta_parser};

pub mod meta;
pub mod parsers;

pub async fn get_file(file_name: &str) -> Result<String, Error> {
    async_fs::read_to_string(format!("./markdown/{}.markdown", file_name)).await
}

pub async fn get_blog_index_vec() -> Vec<Meta> {
    let mut meta_vec: Vec<Meta> = vec![];

    let files = fs::read_dir("./markdown").unwrap();

    for file in files {
        meta_vec.push(
            meta_parser(
                file.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .split(".markdown")
                    .collect::<Vec<&str>>()[0],
            )
            .await,
        );
    }

    meta_vec.sort_by(|a, b| a.date.cmp(&b.date));

    meta_vec
}

pub async fn get_series_index_vec() -> Vec<Meta> {
    let mut meta_vec: Vec<Meta> = vec![];
    meta_vec
}

pub async fn get_meta_by_series_vec() -> Vec<Meta> {
    let mut meta_vec: Vec<Meta> = vec![];
    meta_vec
}

pub async fn get_meta_and_markdown(file_name: &str) -> (Meta, String) {
    let meta = meta_parser(file_name).await;
    let mark = markdown_parser(file_name).await;
    return (meta, mark);
}
