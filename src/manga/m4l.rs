use crate::manga::{
    error_type::DownloadChapterError,
    utils::{get_chapter_string, zip_rename_delete, SPINNER_LONG, SPINNER_SHORT},
};
use console::{style, Term};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::fs;

use super::utils::{create_folder, is_int, write_file};

use camino::Utf8Path;
use futures::{stream, StreamExt};
use headless_chrome::Browser;
use reqwest::Client;

use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};

use url::Url;

pub async fn download_all_chapters(
    chapters: Vec<f64>,
    manga_name: String,
    link: String,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let (mut url, _) = get_chapter_info(link.clone()).await?;

    for chapter in chapters {
        let chapter_download = download_chapter(chapter, &client, manga_name.clone(), &url).await;

        if chapter_download.is_err_and(|x| x == DownloadChapterError::ChaterMayNotExist) {
            println!(
                "{} Possible Image Link Change, Getting New Link for {}",
                style("[WARN]").yellow().bold(),
                style(chapter).italic(),
            );
            let new_link =
                get_chapter_link(link.clone(), chapter).map_err(|err| err.to_string())?;

            let (new_url, _) = get_chapter_info(new_link.clone()).await.or_else(|err| {
                println!(
                    "{} Chapter {} may not exist",
                    style("[ERROR]").red().bold(),
                    style(chapter).italic().bold()
                );
                Err(err)
            })?;
            // let (new_url, _) = get_chapter_info(new_link.clone()).await?;

            let chapter_download =
                download_chapter(chapter, &client, manga_name.clone(), &new_url).await;

            if chapter_download.is_ok() {
                url = new_url
            } else {
                println!(
                    "{} Chapter {} may not exist",
                    style("[ERROR]").red().bold(),
                    style(chapter).italic().bold()
                );
            }
        }
    }
    Ok(())
}

pub fn get_chapter_link(link: String, chapter: f64) -> Result<String, DownloadChapterError> {
    let re = Regex::new(r"chapter-\d")
        .map_err(|err| DownloadChapterError::CouldntParseRegex(err.to_string()))?;
    let chapter_string = get_chapter_string(chapter);

    Ok(re
        .replace(link.as_str(), format!("chapter-{chapter_string}").as_str())
        .to_string())
}

pub async fn get_chapter_info(url: String) -> Result<(Url, i32), Box<dyn Error>> {
    let browser = Browser::default()?;
    let term = Term::stdout();
    let bar = ProgressBar::new_spinner();

    bar.set_style(
        ProgressStyle::with_template("{prefix:.bold.dim} {spinner:.blue}")
            .unwrap()
            .tick_strings(SPINNER_SHORT),
    );

    let tab = browser.new_tab()?;
    println!("{} Opened Browser", style("[1/3]").bold().dim());

    tab.set_default_timeout(Duration::from_secs(15));
    println!("{} Opening Tab", style("[2/3]").bold().dim());
    tab.navigate_to(url.as_str())?;
    term.clear_last_lines(1)?;

    tab.wait_until_navigated()?;
    println!("{} Navigated to Tab", style("[2/3]").bold().dim());
    bar.enable_steady_tick(Duration::from_millis(120));
    bar.set_prefix("[3/3]");

    let last_image = tab
        .wait_for_element(
            "div.ImageGallery > :nth-last-child(1 of .ng-scope) > div.ng-scope > img.img-fluid",
        )
        .or_else(|err| {
            bar.finish_and_clear();
            term.clear_last_lines(2)?;
            Err(err)
        })
        .map_err(|err| format!("Cannot find the image element {:?}", err.to_string()))?;

    let image = last_image
        .get_attribute_value("src")
        .unwrap()
        .ok_or("Image Not Found!".to_string())?;

    bar.finish_and_clear();
    term.clear_last_lines(3)?;

    let url = Url::parse(image.as_str())?;
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

pub async fn download_page(
    file_path: &Utf8Path,
    client: &Client,
    url: Url,
) -> Result<(), Box<dyn Error>> {
    let res = client.get(url).send().await?;
    res.error_for_status_ref()?;

    let file_content = res.bytes().await?;
    write_file(file_path, file_content)?;

    Ok(())
}

pub async fn download_chapter(
    chapter: f64,
    client: &Client,
    manga_name: String,
    base_url: &Url,
) -> Result<(), DownloadChapterError> {
    let chapter_folder_name = if is_int(chapter) {
        format!("{} {}", manga_name, chapter.floor())
    } else {
        format!("{} {}", manga_name, chapter)
    };

    let chapter_path = Utf8Path::new("./")
        .join(manga_name.as_str())
        .join(chapter_folder_name.as_str());

    create_folder(&chapter_path)
        .map_err(|err| DownloadChapterError::CannotCreateFolder(err.to_string()))?;

    let pages: Vec<i32> = (1..1001).collect();

    let url_chapter_string = get_chapter_string(chapter);
    let should_continue = Arc::new(Mutex::new(true));
    let counter = Arc::new(Mutex::new(1));

    let bar = ProgressBar::new_spinner();
    bar.set_style(
        ProgressStyle::with_template("{prefix:.blue.bold} {spinner:.blue} {wide_msg}")
            .unwrap()
            .tick_strings(SPINNER_LONG),
    );

    bar.set_prefix("[DOWNLOADING]");
    stream::iter(pages)
        .map(|i| {
            let url_file_string = format!("{}-{:0>3}.png", url_chapter_string, i);

            let url = match base_url.join(&url_file_string) {
                Ok(url) => url,
                Err(err) => panic!("[ERROR] Can't Get URL {:?}", err),
            };
            let file_path = chapter_path.join(format!("{:0>3}.png", i));

            let should_continue = should_continue.clone();
            let counter = counter.clone();
            let bar = bar.clone();

            async move {
                if *should_continue.lock().unwrap() {
                    match download_page(&file_path, client, url).await {
                        Ok(_) => {
                            let mut counter = counter.lock().unwrap();
                            bar.inc(1);
                            bar.set_message(format!("{}", *counter));
                            *counter += 1;
                        }
                        Err(_) => {
                            let mut state = should_continue.lock().unwrap();
                            *state = false;
                        }
                    }
                }
            }
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    zip_rename_delete(&chapter_path, &chapter_folder_name);

    if *counter.lock().unwrap() == 1 {
        let zip_path = &chapter_path
            .parent()
            .unwrap()
            .join(format!("{}.cbz", &chapter_folder_name));

        fs::remove_file(zip_path)
            .map_err(|err| DownloadChapterError::CouldntRemoveFile(err.to_string()))?;
        return Err(DownloadChapterError::ChaterMayNotExist);
    }

    bar.finish_and_clear();
    println!(
        "{} Chapter {: <4} ({: >4}) Pages",
        style("[FINISHED]").green().bold(),
        style(chapter).green(),
        style(*counter.lock().unwrap()).green()
    );

    Ok(())
}
