use ncurses::*;
use std::char;
use std::str::{FromStr};
use std::fmt::Debug;

pub mod windows;
pub mod colors;

use windows::{*};
use windows::{NcursesWindow, NcursesWindowParent};

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
    let mut w = windows::ScaledWindow::new(YX(0,2), screen - YX(1,2), None);
    let style = Style::from(0, ' ' as chtype,
                            ' ' as chtype, 0,
                            ' ' as chtype, ' ' as chtype,
                            0, ' ' as chtype);
    w.sw.wborder(style);
    w.sw.wrefresh();
    w
}

pub fn create_subwindow<'a>(w: &'a mut SimpleWindow) { //-> &mut SimpleWindow {

    w.subwin(YX(10,10), YX(5,5), "Hola", |child| {
        let style = Style::default();
        child.wborder(style);
        child.wrefresh();
    });

    w.subwin(YX(10,10), YX(5,5), "Hola", |child| {
        let style = Style::default();
        child.wborder(style);
        child.wrefresh();
    });

}

pub fn sub_window<'a>(w: &'a mut SimpleWindow) { //-> &mut SimpleWindow {
    let screen = windows::screen_size();
    create_subwindow(w);
}
