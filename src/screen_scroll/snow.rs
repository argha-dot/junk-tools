use core::time;
use rand;
use termsize::Size;
use std::thread;

use crate::screen_scroll::draw_grid;
use crate::screen_scroll::terminal_size;
use crate::utils::rand_choice;
use crate::utils::SNOWFLAKES;

use super::hide_cursor;
use super::set_colors;
use super::show_cursor;
use super::ScrollerOptionsArgs;

pub fn generate_snow_row<'a>(terminal_size: &Size) -> Vec<&'a str> {
    let mut row: Vec<&str> = vec![];
    for _ in 0..terminal_size.cols {
        let char: &str;
        if rand::random::<f64>() < 0.025 {
            char = *rand_choice(SNOWFLAKES);

        } else {
            char = " ";
        }

        row.push(char);
    }

    row
}

pub fn render_snow(options: ScrollerOptionsArgs) {
    let terminal_size = terminal_size();

    let mut terminal_grid =
        vec![vec![" "; terminal_size.cols as usize]; terminal_size.rows as usize];

    show_cursor();
    hide_cursor();

    loop {
        if let Some(color) = options.color {
            set_colors(color);
        }

        draw_grid(&terminal_grid);

        let row = generate_snow_row(&terminal_size);

        terminal_grid.insert(0, row);
        terminal_grid.pop();

        thread::sleep(time::Duration::from_millis(200));
    }
}
