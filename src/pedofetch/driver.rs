use crate::pedofetch::entities::*;
use ncurses::{
    addstr, clear, curs_set, endwin, getch, getmaxyx, initscr, mv, resetty, CURSOR_VISIBILITY,
};

/// Runs driver code in your terminal.
/// # Examples
/// ```
///     pedofetch::driver();
/// ```
pub fn run() {
    initscr();
    let mut ts = TermCoord { x: 0, y: 0 };
    let mut ch = 1;
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    while ch != 'q' as i32 {
        clear();
        getmaxyx(ncurses::stdscr(), &mut ts.y, &mut ts.x);
        match ts.calculate_corners() {
            Ok(_corners) => {
                mv(ts.y / 2, ts.x / 2);
                addstr(format!("{}x{}", ts.x, ts.y).as_str());
            }
            Err(warning) => {
                mv(ts.y / 2, ts.x / 2 - warning.to_string().len() as i32 / 2);
                addstr(warning.to_string().as_str());
            }
        }
        ch = getch();
    }
    resetty();
    endwin();
}
