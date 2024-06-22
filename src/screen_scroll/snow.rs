use core::time;
use rand;
use std::thread;
use termsize::Size;

use crate::screen_scroll::{draw_grid, terminal_size};
use crate::utils::{rand_choice, SNOWFLAKES};

use super::{hide_cursor, set_color, ScrollerOptionsArgs};

pub fn generate_snow_row<'a>(terminal_size: &Size, _density: f64) -> Vec<&'a str> {
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

    hide_cursor();

    loop {
        set_color(options.color);

        let density = options.density.unwrap_or(0.025);
        let speed = options.speed.unwrap_or(500);

        draw_grid(&terminal_grid);

        let row = generate_snow_row(&terminal_size, density);

        terminal_grid.insert(0, row);
        terminal_grid.pop();

        thread::sleep(time::Duration::from_millis(speed));
    }
}
