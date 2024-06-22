use clap::{Args, ValueEnum};

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy)]
pub enum JokeCategories {
    Any,
    Custom,
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy)]
pub enum CustomCategories {
    Programming,
    Misc,
    Dark,
    Pun,
    Spooky,
    Christmas,
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy)]
pub enum JokeBlacklists {
    NSFW,
    Religious,
    Political,
    Racist,
    Sexist,
    Explicit,
}

#[derive(Debug, Args)]
pub struct JokeArgs {
    /// Show All Jokes or Custom Jokes
    #[arg(long, default_value = "any")]
    pub r#type: JokeCategories,
    /// Custom Category
    #[arg(long, required_if_eq("type", "custom"), value_delimiter(','))]
    pub category: Option<Vec<CustomCategories>>,
    /// Don't show jokes containing these themes
    #[arg(long, value_delimiter(','))]
    pub blacklist: Option<Vec<JokeBlacklists>>,
}
