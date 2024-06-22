use clap::Args;
use clap::Subcommand;

mod matrix;
mod snow;
mod utils;

pub use self::{matrix::*, snow::*, utils::*};

#[derive(Debug, Args)]
pub struct Scroller {
    /// The Type of Scroller to render
    #[command(subcommand)]
    pub r#type: Scrollers,

    #[command(flatten)]
    pub options: ScrollerOptionsArgs,
}

// #[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy)]
#[derive(Debug, Subcommand)]
pub enum Scrollers {
    /// Render snow
    Snow,
    /// Render Matrix
    Matrix,
}

#[derive(Debug, Args)]
pub struct ScrollerOptionsArgs {
    /// The Color to display the elements in
    #[arg(short, long)]
    pub color: Option<Colors>,
    /// The Density of the elements. This is a float value between 0 and 1, defualts to 0.025
    #[arg(long)]
    pub density: Option<f64>,
    /// The Speed of the elements. This is in milliseconds, defaults to 200
    #[arg(long)]
    pub speed: Option<u64>,
}
