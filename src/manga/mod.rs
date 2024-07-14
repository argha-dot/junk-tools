mod args;
mod utils;

pub use self::args::*;
pub use self::utils::*;
use std::error::Error;

pub async fn manga_download(args: MangaDownArgs) -> Result<(), Box<dyn Error>> {
    let chapters = match parse_chapters(args.chapter) {
        Ok(chapters) => chapters,
        Err(err) => return Err(err.into()),
    };

    let first_chapter = chapters.get(0).copied();

    let url = get_chapter_link(&args.link, first_chapter)?;
    println!("{}", url.to_string());

    let (url, page) = get_chapter_info(url.to_string()).await?;

    println!("{:?} {}", url.to_string(), page);

    Ok(())
}
