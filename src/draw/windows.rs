use std::char;
use std::ops::{Add,Sub};
use std::path::PathBuf;
use std::fmt::Debug;
use std::collections::HashMap;

use ncurses::*;
use unicode_segmentation::UnicodeSegmentation;

use super::colors::{*};

pub fn truncate(string : &String, limit : usize) -> String {
    let title_str = &string[..];
    let title_vec = UnicodeSegmentation::graphemes(title_str, true).collect::<Vec<&str>>();
    if title_vec.len() >= limit {
        let mut trunc = title_vec[..(limit-1)].join("");
        trunc.push_str("~");
        trunc
    }
    else {
        title_vec.join("")
    }
}

pub fn screen_size() -> YX {
    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    YX(max_y,max_x)
}

#[derive(Copy,Clone,Debug)]
pub struct YX(pub i32, pub i32);
impl YX {
    pub fn from_u(y: usize, x: usize) -> YX {
        YX(y as i32, x as i32)
    }
    pub fn from_i32(y: i32, x: i32) -> YX {
        YX(y, x)
    }
}

impl Add for YX {
    type Output = YX;
    fn add(self, rhs: YX) -> YX {
        YX( self.0 + rhs.0,
            self.1 + rhs.1
        )
    }
}

impl Sub for YX {
    type Output = YX;
    fn sub(self, rhs: YX) -> YX {
        YX( self.0 - rhs.0,
            self.1 - rhs.1
        )
    }
}

pub struct Window {
    w: WINDOW,
}

impl Window {
    fn w(&mut self) -> WINDOW { self.w }
}

impl Drop for Window {
    fn drop(&mut self) {
        delwin(self.w);
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub struct WindowId<'a> (&'a str);

pub struct SimpleWindow<'w>{
    w: Window,
    pos: YX,
    size: YX,
    subw: HashMap<WindowId<'w>, SimpleWindow<'w>>
}

impl <'a> SimpleWindow<'a> {

    pub fn init(pos: YX, size: YX, style: Option<(chtype, chtype)>) -> SimpleWindow<'a> {
        let mut w = Window { w: newwin(size.0, size.1, pos.0, pos.1) };

        if let Some((att1, att2)) = style {
            box_(w.w(), att1, att2);
        }
        else {
            box_(w.w(), 0, 0);
        }

        let subw = HashMap::new();

        SimpleWindow {
            w,
            pos,
            size,
            subw,
        }
    }

    pub fn reshape(&mut self, pos: YX, size: YX) {
        self.pos = pos;
        self.size = size;
        // We clear the borders
        self.box_(NCURSES_ACS(' '), NCURSES_ACS(' '));
        // Resize them
        self.wresize(size.0, size.1);
        // Move them
        self.mvwin(pos.0, pos.1);
        // Write them again
        self.box_(0, 0);
    }

    pub fn write_text(&mut self, i: i32, text: &str, color: NCURSES_ATTR_T) {

        // Aux data
        let yx0 = self.pos;// + YX(0i32, 2i32);
        let yxS = self.size;// + YX(0i32, 2i32);

        // truncate text
        let truncname = truncate(&text.to_string(), yxS.1 as usize -2);

        // Write directories
        self.wattron(color);
        self.mvwprintw(YX(i,1), &truncname);
        self.wattroff(color);
        // self.mvwprintw(YX(1,1), &format!("{:?}", yx0));
    }

    pub fn clean(&mut self) {

        // Aux data
        let yx_size = self.size;// + YX(0i32, 2i32);

        // Construct empty line
        let mut string = String::new();
        for _i in 1..yx_size.1-1 {
            string.push_str(" ")
        }

        // Write every line
        for i in 1..yx_size.0-1 {
            self.mvwprintw(YX(i,1), &string);
        }

    }


}

pub trait  NcursesWindowParent<'c>: NcursesWindow {
    type Child: NcursesWindow;
    fn subwindows(&'c mut self) -> &mut HashMap<WindowId<'c>, Self::Child>;
    fn subwin<F>(&'c mut self, size: YX, pos: YX, name: &'c str, mut draw: F)
        where F: FnMut(&mut Self::Child) -> ();
    fn get(&'c mut self, name: &'c str) -> Option<&Self::Child>;
    fn get_mut(&'c mut self, name: &'c str) -> Option<&mut Self::Child>;

    fn draw_subwin<F>(&'c mut self, name: &'c str, mut draw: F) -> ()
        where F: FnMut(&mut Self::Child) -> () {
            if let Some(child) = self.get_mut(name) {
                draw(child);
                child.wrefresh();
            };
        }

}

impl <'w> NcursesWindowParent<'w> for SimpleWindow<'w> {
    type Child = Self;

    fn subwindows(&'w mut self) -> &mut HashMap<WindowId<'w>, SimpleWindow<'w>> { &mut self.subw }

    fn subwin<F>(&'w mut self, size: YX, pos: YX, name: &'w str, mut draw: F)
        where F: FnMut(&mut Self::Child) -> () {
        let YX(lines, cols) = size;
        let YX(y, x) = pos;
        let sw = Window{ w: subwin(self.window(), lines, cols, y, x) };

        let wid = WindowId(name);
        let mut sub_window = SimpleWindow {
            w: sw,
            pos: pos,
            size: size,
            subw: HashMap::new(),
        };

        draw(&mut sub_window);
        sub_window.wrefresh();

        self.subwindows().insert(wid, sub_window);

    }

    fn get(&'w mut self, name: &'w str) -> Option<&'w SimpleWindow> {
        let wid = WindowId(name);
        self.subwindows().get(&wid)
    }

    fn get_mut(&'w mut self, name: &'w str) -> Option<&'w mut SimpleWindow> {
        let wid = WindowId(name);
        self.subwindows().get_mut(&wid)
    }

}

impl <'b> NcursesWindow for SimpleWindow<'b> {
    fn window(&mut self) -> WINDOW { self.w.w() }
    fn pos(&mut self) -> YX { self.pos }
    fn size(&mut self) -> YX { self.size }
}


pub struct ScaledWindow<'a> {
    pub sw: SimpleWindow<'a>,
    vscale: f32,
    voffset: f32,
    hscale: f32,
    hoffset: f32,
}

impl <'a> ScaledWindow<'a> {

    pub fn new(pos: YX, size: YX, style: Option<(chtype, chtype)>) -> ScaledWindow<'a> {
        ScaledWindow {
            sw: SimpleWindow::init(pos, size, style),
            vscale: 1.,
            voffset: 0.,
            hscale: 1.,
            hoffset: 0.,
        }
    }

    pub fn set_vscale(&mut self, scale: f32) {
        self.vscale = scale;
    }

    pub fn set_hscale(&mut self, scale: f32) {
        self.hscale = scale;
    }


    pub fn set_voffset(&mut self, offset: f32) {
        self.voffset = offset;
    }


    pub fn set_hoffset(&mut self, offset: f32) {
        self.hoffset = offset;
    }

    pub fn set_scale_offset(&mut self, vscale: f32, voffset: f32, hscale: f32, hoffset: f32) {
        self.set_vscale(vscale);
        self.set_voffset(voffset);
        self.set_hscale(hscale);
        self.set_hoffset(hoffset);
    }

}




pub trait NcursesWindow {
    fn window(&mut self) -> WINDOW;
    fn pos(&mut self) -> YX;
    fn size(&mut self) -> YX;

    fn box_(&mut self, att1: chtype, att2: chtype) {
        box_(self.window(), att1, att2);
    }

    fn wborder(&mut self, style: Style) {

        let Style {
            ls, rs,
            ts, bs,
            tlc, trc,
            blc, brc,
        } = style;

        wborder(self.window(),
            ls, rs,
            ts, bs,
            tlc, trc,
            blc, brc,
        );
    }

    fn wclear(&mut self) {
        wclear(self.window());
    }

    fn wrefresh(&mut self) {
        wrefresh(self.window());
    }

    fn wnoutrefresh(&mut self) {
        wnoutrefresh(self.window());
    }

    fn redrawwin(&mut self) {
        redrawwin(self.window());
    }

    fn whline(&mut self, ch: chtype, n: i32) {
        whline(self.window(), ch, n);
    }

    fn wvline(&mut self, ch: chtype, n: i32) {
        wvline(self.window(), ch, n);
        // wrefresh(self.window());
    }

    fn mvwhline(&mut self, yx: YX, ch: chtype, n: i32) {
        let YX(y,x) = yx;
        mvwhline(self.window(), y,x,ch,n);
        // wrefresh(self.window());
    }

    fn mvwvline(&mut self, yx: YX, ch: chtype, n: i32) {
        let YX(y,x) = yx;
        mvwvline(self.window(), y, x, ch, n);
        // wrefresh(self.window());
    }

    fn mvhline(yx: YX, ch: chtype, n: i32) {
        let YX(y,x) = yx;
        mvhline(y,x,ch,n);
    }

    fn mvvline(yx: YX, ch: chtype, n: i32) {
        let YX(y,x) = yx;
        mvvline(y,x,ch,n);
    }

    fn wprintw(&mut self, s: &str) -> i32 {
        wprintw(self.window(), s)
    }

    fn mvwprintw(&mut self, yx: YX, s: &str) -> i32 {
        let YX(y,x) = yx;
        mvwprintw(self.window(), y, x, s)
    }

    fn split_vline(&mut self, x: i32) {
        let YX(h,w) = self.size();
        let YX(y0,x0) = self.pos();
        self.mvwvline(YX(0,x0+x), ACS_TTEE(), 1);
        self.mvwvline(YX(1,x0+x), 0, h-2);
        self.mvwvline(YX(h-1,x0+x), ACS_BTEE(), 1);
    }

    fn split_hline(&mut self, y: i32) {
        let YX(h,w) = self.size();
        let YX(y0,x0) = self.pos();
        self.mvwhline(YX(y0+y,0), ACS_LTEE(), 1);
        self.mvwhline(YX(y0+y,1), 0, w-2);
        self.mvwhline(YX(y0+y, w-1), ACS_RTEE(), 1);
    }

    fn wattron(&mut self, color_pair: NCURSES_ATTR_T) {
        wattron(self.window(), color_pair);
    }

    fn wattroff(&mut self, color_pair: NCURSES_ATTR_T) {
        wattroff(self.window(), color_pair);
    }

    // fn wattr_set(&mut self, attr: NCURSES_ATTR_T, cpair: NCURSES_ATTR_T) {
    //     wattr_set(self.window(), attr, cpair);
    // }

    fn wresize(&mut self, lines: i32, cols: i32) {
        wresize(self.window(), lines, cols);
    }

    fn mvwin(&mut self, lines: i32, cols: i32) {
        mvwin(self.window(), lines, cols);
    }


    fn touchwin(&mut self) {
        touchwin(self.window());
    }

}

pub struct Style {
     ls: chtype,
     rs: chtype,
     ts: chtype,
     bs: chtype,
    tlc: chtype,
    trc: chtype,
    blc: chtype,
    brc: chtype,
}

impl Style {

    pub fn new(ls: char, rs: char, ts: char, bs: char, tlc: char, trc: char, blc: char, brc: char,) -> Self {
        Self {
            ls: ls as chtype,
            rs: rs as chtype,
            ts: ts as chtype,
            bs: bs as chtype,
            tlc: tlc as chtype,
            trc: trc as chtype,
            blc: blc as chtype,
            brc: brc as chtype,
        }
    }

    pub fn from(ls: chtype, rs: chtype, ts: chtype, bs: chtype, tlc: chtype, trc: chtype, blc: chtype, brc: chtype,) -> Self {
        Self {
            ls: ls as chtype,
            rs: rs as chtype,
            ts: ts as chtype,
            bs: bs as chtype,
            tlc: tlc as chtype,
            trc: trc as chtype,
            blc: blc as chtype,
            brc: brc as chtype,
        }
    }

    pub fn default() -> Self {
        Self {
            ls: 0,
            rs: 0,
            ts: 0,
            bs: 0,
            tlc: 0,
            trc: 0,
            blc: 0,
            brc: 0,
        }
    }

}
