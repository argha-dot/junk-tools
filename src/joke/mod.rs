use crate::utils::JOKE_API_URL;
use reqwest::Client;

mod joke_args;

pub use joke_args::*;

pub async fn get_joke(args: JokeArgs) -> Result<(), reqwest::Error> {
    let client = Client::new();

    println!("{:?}", args);
    let res = client
        .get(format!("{}/joke/Any?format=txt", JOKE_API_URL))
        .send()
        .await?
        .text()
        .await?;

    println!("{}", res);

    Ok(())
}
