use screen_scroll::{render_matrix, render_snow, show_cursor, Scroller, Scrollers};

use clap::{Parser, Subcommand};
mod screen_scroll;
mod utils;

/// Random commands that are made for fun and stuff
#[derive(Parser, Debug)]
#[command(about = "Random commands that are made for fun and out of sheer boredom")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start a screensaver thingy for your terminal
    #[command(name = "scroller")]
    Scrollers(Scroller),
    /// Show the cursor if it's hidden
    #[command(name = "cursor")]
    Cursor,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Cursor => {
            show_cursor();
        },
        Commands::Scrollers(args) => {
            match args.r#type {
                Scrollers::Snow => render_snow(args.options),
                Scrollers::Matrix => render_matrix(args.options),
            }
        }
    }
}
