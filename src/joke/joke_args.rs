use clap::{Args, ValueEnum};
use strum::{Display, EnumString};

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "PascalCase")]
pub enum CustomCategories {
    Programming,
    #[clap(name = "misc")]
    Miscellaneous,
    Dark,
    Pun,
    Spooky,
    Christmas,
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum JokeBlacklists {
    NSFW,
    Religious,
    Political,
    Racist,
    Sexist,
    Explicit,
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum JokeType {
    Single,
    TwoPart,
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum ResponseFormat {
    Json,
    YAML,
    Txt,
}

#[derive(Debug, Args)]
pub struct JokeArgs {
    /// Custom Category, if no categories are selected,
    /// then we take a joke is taken form any of the categories
    #[arg(long, value_delimiter(','))]
    pub category: Vec<CustomCategories>,
    /// Don't show jokes containing these themes
    #[arg(long, value_delimiter(','))]
    pub blacklist: Vec<JokeBlacklists>,
    /// Whether the joke is a single part joke or a two part joke
    #[arg(long = "type")]
    pub joke_type: Option<JokeType>,
    /// The Response Format
    #[arg(long = "format", default_value = "txt")]
    pub response_format: ResponseFormat,
}
