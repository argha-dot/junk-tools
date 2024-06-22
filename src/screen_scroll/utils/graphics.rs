use clap::ValueEnum;
use strum::Display;

#[derive(Display, ValueEnum, Debug, Clone, Copy)]
pub enum Colors {
    #[strum(serialize = "30")]
    Black,
    #[strum(serialize = "31")]
    Red,
    #[strum(serialize = "32")]
    Green,
    #[strum(serialize = "33")]
    Yellow,
    #[strum(serialize = "34")]
    Blue,
    #[strum(serialize = "35")]
    Magenta,
    #[strum(serialize = "36")]
    Cyan,
    #[strum(serialize = "38")]
    White,
    #[strum(serialize = "39")]
    Default,
    #[strum(serialize = "90")]
    BrightBlack,
    #[strum(serialize = "91")]
    BrightRed,
    #[strum(serialize = "92")]
    BrightGreen,
    #[strum(serialize = "93")]
    BrightYellow,
    #[strum(serialize = "94")]
    BrightBlue,
    #[strum(serialize = "95")]
    BrightMagenta,
    #[strum(serialize = "96")]
    BrightCyan,
    #[strum(serialize = "97")]
    BrightWhite,
}

/// Set the color
pub fn set_color(color: Option<Colors>) {
    if let Some(color) = color {
        print!("\x1b[{}m", color);
    }
}
