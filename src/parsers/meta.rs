#[derive(Debug, Default)]
pub struct Meta {
    pub file_name: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub series: String,
}

impl Meta {
    pub fn new(file: &str, file_name: &str) -> Self {
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
        meta.file_name = file_name.to_string();
        return meta;
    }
}
