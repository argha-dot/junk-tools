use termsize::{self, Size};

mod graphics;

pub use self::graphics::*;

pub fn draw_grid(grid: &Vec<Vec<&str>>) {
    println!("\x1b[2J"); // Clear the Screen

    let mut output = "".to_string();

    grid.iter().for_each(|row| {
        output.push_str(row.join("").as_str());
        output.push_str("\n");
    });

    print!("{}", output);
}

pub fn terminal_size() -> Size {
    match termsize::get() {
        Some(size) => size,
        None => panic!("[ERROR] Cannot determine terminal size"),
    }
}

pub fn hide_cursor() {
    print!("\x1b[?25l"); // Get rid of the cursor
}

pub fn show_cursor() {
    print!("\x1b[?25h"); // Show the cursor again
}