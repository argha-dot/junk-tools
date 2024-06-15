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

#[derive(Display, ValueEnum, Debug, Clone, Copy)]
pub enum TextStyle {
    #[strum(serialize = "1")]
    Bold,
    #[strum(serialize = "2")]
    Dim,
    #[strum(serialize = "3")]
    Italic,
    #[strum(serialize = "4")]
    Underline,
    #[strum(serialize = "5")]
    Blinking,
    #[strum(serialize = "7")]
    Inverse,
    #[strum(serialize = "8")]
    Hidde,
    #[strum(serialize = "9")]
    StrikeThrough,
}

pub fn set_colors(color: Colors) {
    print!("\x1b[{}m", color);
}
