use reqwest::Client;
use std::error::Error;
use url::Url;

mod joke_args;

pub use joke_args::*;

use crate::utils::JOKE_API_URL;

pub async fn get_joke(args: JokeArgs) -> Result<(), Box<dyn Error>> {
    let category = if args.category.is_empty() {
        "Any".to_string()
    } else {
        args.category
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    };

    let joke_type = args.joke_type.map(|x| x.to_string()).unwrap_or_default();

    let blacklist = if args.blacklist.is_empty() {
        "".to_string()
    } else {
        args.blacklist
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    };

    let response_format = args.response_format.to_string();

    let mut api_url = Url::parse(JOKE_API_URL)?;

    api_url
        .path_segments_mut()
        .map_err(|_| url::ParseError::InvalidIpv4Address)?
        .push("joke")
        .push(category.as_str());

    api_url
        .query_pairs_mut()
        .append_pair("format", response_format.as_str())
        .append_pair("blacklistFlags", blacklist.as_str())
        .append_pair("type", joke_type.as_str())
        .append_pair("amount", args.count_jokes.to_string().as_str());

    let client = Client::new();
    let res = client.get(api_url).send().await?.text().await?;

    println!("{}", res);

    Ok(())
}
