use camino::Utf8PathBuf;
use clap::{builder::ValueParser, Args};
use regex::Regex;
use url::Url;

pub fn parse_chapters(chapters: Vec<String>) -> Result<Vec<f64>, String> {
    let mut parsed_chapters: Vec<f64> = vec![];
    for split in chapters {
        if split.contains("..") {
            let nums = split
                .split("..")
                .map(|s| s.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
                .map_err(|err| format!("[ERROR] Splitting Strings: {:?}", err))?;

            if nums.len() != 2 {
                return Err(format!("More than 3 values in iterator: {}", split));
            }

            if nums.first() > nums.last() {
                return Err(format!("Iterator values should be ascending: {}", split));
            }
            let first = *nums.first().expect("Couldn't find first");
            let last = 1 + *nums.last().expect("Couldn't find last");

            parsed_chapters.append(&mut (first..last).map(|i| i as f64).collect());
        } else {
            parsed_chapters.push(
                split
                    .parse::<f64>()
                    .map_err(|err| format!("[ERROR] Can't Parse Float {:?}", err))?,
            )
        }
    }

    Ok(parsed_chapters)
}

pub fn validate_url() -> ValueParser {
    ValueParser::from(move |s: &str| -> Result<Url, String> {
        let url = match Url::parse(s) {
            Ok(url) => url,
            Err(err) => return Err(err.to_string()),
        };

        let re =
            Regex::new(r"https:\/\/manga4life.com/read-online/([\w+-]+)chapter-([\d\.]+).html")
                .expect("a valid regex");
        match re.is_match(s) {
            true => Ok(url),
            false => Err("Please Give a Manga4Life Link".into()),
        }
    })
}

#[derive(Debug, Args)]
pub struct MangaDownArgs {
    /// The Title of the manga
    #[arg(short = 't', long = "title")]
    pub title: Option<String>,
    /// The Chapters to download
    #[arg(short = 'c', long = "chapter", value_delimiter = ',')]
    pub chapters: Option<Vec<String>>,
    /// The Link
    #[arg(short = 'l', long = "link", value_parser = validate_url())]
    pub link: Url,
    /// Output path
    #[arg(short = 'o', long, default_value = "./")]
    pub path: Utf8PathBuf,
}
