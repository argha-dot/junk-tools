use core::time;
use std::thread;

use super::{
    draw_grid, generate_snow_row, hide_cursor, set_color, show_cursor, terminal_size,
    ScrollerOptionsArgs,
};

pub fn render_matrix(options: ScrollerOptionsArgs) {
    let terminal_size = terminal_size();

    let mut terminal_grid =
        vec![vec![" "; terminal_size.cols as usize]; terminal_size.rows as usize];

    show_cursor();
    hide_cursor();

    loop {
        set_color(options.color);

        let density = options.density.unwrap_or(0.025);
        let speed = options.speed.unwrap_or(200);

        draw_grid(&terminal_grid);

        let row = generate_snow_row(&terminal_size, density);

        terminal_grid.insert(0, row);
        terminal_grid.pop();

        thread::sleep(time::Duration::from_millis(speed));
    }
}
