use joke::{get_joke, JokeArgs};
use manga::{manga_download, MangaDownArgs};
use screen_scroll::{render_matrix, render_snow, show_cursor, Scroller, Scrollers};

use clap::{Parser, Subcommand};
mod joke;
mod manga;
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
    /// Fetch a random joke from the internet
    #[command(name = "joke")]
    Joke(JokeArgs),
    /// Start a screensaver thingy for your terminal
    #[command(name = "scroller")]
    Scrollers(Scroller),
    /// Download Manga
    #[command(name = "md")]
    MangaDown(MangaDownArgs),
    /// Show the cursor if it's hidden
    #[command(name = "cursor")]
    Cursor,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Cursor => {
            show_cursor();
        }
        Commands::Joke(args) => {
            match get_joke(args).await {
                Ok(()) => {}
                Err(err) => eprintln!("[ERROR] Couldn't fetch joke. {} error", err),
            };
        }
        Commands::MangaDown(args) => {
            match manga_download(args).await {
                Ok(()) => {}
                Err(err) => eprintln!("[ERROR] Couldn't download manga. {}", err),
            };
        }
        Commands::Scrollers(args) => match args.r#type {
            Scrollers::Snow => render_snow(args.options),
            Scrollers::Matrix => render_matrix(args.options),
        },
    }
}
