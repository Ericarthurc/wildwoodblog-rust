use super::{get_file, meta::Meta};
use comrak::{markdown_to_html, ComrakOptions};

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
