mod args;
use headless_chrome::Browser;
use std::error::Error;
use std::time::Duration;
use url::Url;

pub use self::args::*;

pub async fn get_chapter_info(url: String) -> Result<(Url, i32), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    tab.set_default_timeout(Duration::from_secs(4));
    tab.navigate_to(url.as_str())?;

    tab.wait_until_navigated()?;

    let last_image = tab
        .wait_for_element(
            "div.ImageGallery > :nth-last-child(1 of .ng-scope) > div.ng-scope > img.img-fluid",
        )
        .map_err(|err| format!("Cannot find the image element {:?}", err.to_string()))?;

    let image = last_image
        .get_attribute_value("src")
        .unwrap()
        .ok_or("Image Not Found!".to_string())?;

    let url = Url::parse(image.as_str())?;
    println!("{:?}", url);
    let path_segments = &url.path_segments().ok_or("Cannot be base")?;

    let last_chapter = path_segments
        .clone()
        .last()
        .ok_or("Last Page Not Found")?
        .split(&['.', '-'])
        .nth(1)
        .ok_or("Couldn't find the chapter split")?
        .parse::<i32>()?;

    Ok((url, last_chapter))
}

pub async fn manga_download(args: MangaDownArgs) -> Result<(), Box<dyn Error>> {
    let chapters = match parse_chapters(args.chapter) {
        Ok(chapters) => chapters,
        Err(err) => return Err(err.into()),
    };

    let (url, last_chapter) = get_chapter_info(args.link).await?;
    println!("{} {}", url.to_string(), last_chapter.to_string());

    Ok(())
}
