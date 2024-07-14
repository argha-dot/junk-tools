use clap::Args;
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

#[derive(Debug, Args)]
pub struct MangaDownArgs {
    /// The Title of the manga
    #[arg(short = 't', long = "title")]
    pub title: String,
    /// The Chapters to download
    #[arg(short = 'c', long = "chapter", value_delimiter = ',')]
    pub chapter: Vec<String>,
    /// The Link
    #[arg(short = 'l', long = "link", value_parser = clap::value_parser!(Url))]
    pub link: Url,
}
