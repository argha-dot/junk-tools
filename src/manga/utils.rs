use headless_chrome::Browser;
use regex::Regex;
use std::error::Error;
use std::time::Duration;
use url::Url;

pub fn get_chapter_link(url: &Url, chapter: Option<f64>) -> Result<Url, Box<dyn Error>> {
    let re = Regex::new(r"(?<name>[\w-]*-)(?<chapter>\d+)(?<ext>.html)")?;

    let url_end = url
        .path_segments()
        .ok_or_else(|| "cannot be base")?
        .last()
        .ok_or("couldn't find last")?;

    let cap = re.replace(
        url_end,
        if let Some(chapter) = chapter {
            format!("${{name}}{}${{ext}}", chapter.to_string())
        } else {
            "${name}${chapter}${ext}".to_string()
        }
        .as_str(),
    );

    let mut url_mut = url.clone();
    url_mut
        .path_segments_mut()
        .map_err(|_| "cannot be base")?
        .pop()
        .push(cap.as_ref());

    Ok(url_mut)
}

pub fn get_all_chapter_links(url: &Url, chapters: &Vec<f64>) -> Result<Vec<Url>, Box<dyn Error>> {
    let url = url.clone();

    let re = Regex::new(r"(?<name>[\w-]*-)(?<chapter>\d+)(?<ext>.html)")?;
    let url_binding = url.to_string();

    let urls = chapters
        .iter()
        .map(|ch| {
            let cap = re.replace(
                &url_binding.as_str(),
                format!("${{name}}{}${{ext}}", ch.to_string()).as_str(),
            );
            Url::parse(cap.as_ref())
        })
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(urls)
}

pub async fn get_chapter_info(url: String) -> Result<(Url, i32), Box<dyn Error>> {
    let browser = Browser::default()?;

    println!("start");
    let tab = browser.new_tab()?;

    tab.set_default_timeout(Duration::from_secs(4));
    tab.navigate_to(url.as_str())?;

    tab.wait_until_navigated()?;
    println!("navigated");

    let last_image = tab
        .wait_for_element(
            "div.ImageGallery > :nth-last-child(1 of .ng-scope) > div.ng-scope > img.img-fluid",
        )
        .map_err(|err| format!("Cannot find the image element {:?}", err.to_string()))?;

    let image = last_image
        .get_attribute_value("src")
        .unwrap()
        .ok_or("Image Not Found!".to_string())?;
    println!("found image");

    let url = Url::parse(image.as_str())?;
    let path_segments = &url.path_segments().ok_or("Cannot be base")?;

    let last_chapter = path_segments
        .clone()
        .last()
        .ok_or("Last Page Not Found")?
        .rsplit_once(&['.', '-'])
        .ok_or("Couldn't find the chapter split")?
        .0
        .split("-")
        .nth(1)
        .ok_or("Couldn't find the chapter split")?
        .parse::<i32>()?;
    println!("found last_chapter");

    Ok((url, last_chapter))
}
