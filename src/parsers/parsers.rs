use crate::errors::AppError;

use super::{get_file, meta::Meta};
use comrak::{markdown_to_html, ComrakOptions};

pub async fn markdown_parser(file_name: &str) -> Result<String, AppError> {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.description_lists = true;
    options.extension.superscript = true;
    options.extension.strikethrough = true;
    options.extension.footnotes = true;
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.render.unsafe_ = true;

    let file = get_file(file_name).await?;

    Ok(markdown_to_html(&file, &options))
}

pub async fn meta_parser(file_name: &str) -> Result<Meta, AppError> {
    let file = get_file(file_name).await?;

    Ok(Meta::new(&file, file_name))
}
