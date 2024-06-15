mod snow;
mod matrix;
mod utils;

use clap::Args;
use clap::Subcommand;

pub use self::snow::*;
pub use self::matrix::*;
pub use self::utils::*;


#[derive(Debug, Args)]
pub struct Scroller {
    /// The Type of Scroller to render
    #[command(subcommand)]
    pub r#type: Scrollers,
    
    #[command(flatten)]
    pub options: ScrollerOptionsArgs
}

// #[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Copy)]
#[derive(Debug, Subcommand)]
pub enum Scrollers {
    /// Render snow 
    Snow,
    /// Render Matrix
    Matrix
}

#[derive(Debug, Args)]
pub struct ScrollerOptionsArgs {
    /// The Color to display the elements in
    #[arg(short, long)]
    pub color: Option<Colors>

}