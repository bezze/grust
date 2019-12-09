use ncurses::*;
use std::char;
use std::str::{FromStr};
use std::fmt::Debug;

pub mod windows;
pub mod colors;

use windows::{*};
// use windows::{NcursesWindow, NcursesWindowParent};

pub type DrawResult = Result<(),DrawError>;

pub enum DrawError {
    CleanExit
}

// impl Error {
//     pub fn new() -> Error {
//         Error{}
//     }
// }

pub fn start_ncurses_mode() {
    // utf-8 support
    setlocale(LcCategory::all, "");

    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    if has_colors() == true {
        // mvprintw(0,0,&format!("COLOR"));
        start_color();
        colors::init_color_set();
    }
}

pub fn end_ncurses_mode() {
    endwin();
}

pub fn main_window<'a>() -> ScaledWindow {
    // This should be a ScaleWindow, a SimpleWindow with additional attributes, like scale,
    // offset, units, tick frequency
    let screen = windows::screen_size();
    let shape = Shape { pos: YX(0, 2), size: screen - YX(1,2) };
    let mut w = windows::ScaledWindow::new(shape, None);
    let style = Style::from(0, ' ' as chtype,
                            ' ' as chtype, 0,
                            ' ' as chtype, ' ' as chtype,
                            0, ' ' as chtype);
    w.window.wborder(style);
    w.window.wrefresh();
    w
}

pub fn create_subwindow<'a>(w: &'a mut Window) -> windows::NcResult { //-> &mut SimpleWindow {

    let shape = Shape { pos: YX(10, 10), size: YX(5,5) };
    w.subwin(&shape, "Win1");
    w.draw_sw("Win1", |child| {
        let style = Style::default();
        child.wresize(5, 20)?;
        child.wborder(style)?;
        child.mvwprintw(YX(1,1), "Win1")
    })?;

    let shape = Shape { pos: YX(20, 10), size: YX(10,10) };
    w.subwin(&shape, "Win2");
    w.draw_sw("Win2", |child| {

        child.mvwprintw(YX(1,1), "Win2")?;
        child.wnoutrefresh()?;

        child.wresize(15, 40)?;
        child.wnoutrefresh()?;

        child.mvwin(20, 40)?;
        child.wnoutrefresh()?;

        child.wborder(Style::default())?;
        child.wnoutrefresh()
    })?;

    // let code = w.delete_sw("Win1");
    // println!("{:?}", code);
    // code
    Ok(0)

}

pub fn sub_window<'a>(w: &'a mut Window) { //-> &mut SimpleWindow {
    let screen = windows::screen_size();
    create_subwindow(w);
}
