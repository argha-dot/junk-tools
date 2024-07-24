mod args;
mod error_type;
mod m4l;
mod utils;

use m4l::download_all_chapters;
use std::error::Error;

pub use self::args::*;

pub async fn manga_download(args: MangaDownArgs) -> Result<(), Box<dyn Error>> {
    let chapters = match parse_chapters(args.chapter) {
        Ok(chapters) => chapters,
        Err(err) => return Err(err.into()),
    };

    let _ = download_all_chapters(chapters, args.title, args.link.to_string()).await;
    Ok(())
}
