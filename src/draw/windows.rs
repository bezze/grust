use std::char;
use std::ops::{Add,Sub};
use std::path::PathBuf;
use std::fmt::Debug;
use std::collections::HashMap;
use std::cell::RefCell;

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


#[derive(Copy,Clone,Debug)]
pub struct Shape {
    pub size: YX,
    pub pos: YX
}

#[derive(Debug)]
pub enum WindowError {
    Boxx,
    WBorder,
    WClear,
    WRefresh,
    WNoutRefresh,
    RedrawWin,
    WHline,
    WVline,
    MvWHline,
    MvWVline,
    MvHline,
    MvVline,
    WPrintw,
    MvWPrintw,
    WAttrOn,
    WAttrOff,
    WResize,
    MvWin,
    Touchwin,
    FindLayer,
    DeleteSubWin,
}

#[derive(Debug)]
pub enum NcError {
    DrawError(WindowError),
    Error,
}

pub type NcResult = Result<i32, NcError>;

fn __call_wrapper(code: i32, error: NcError) -> NcResult {
    if code < 0 { Err(error) } else { Ok(code) }
}

pub fn nc_refresh() -> NcResult {
    __call_wrapper(refresh(), NcError::Error)
}

pub struct Window {
    main: WINDOW,
    shape: Shape,
    child_list: Vec<WindowId>,
    child_hash: HashMap<WindowId, Window>,
    id: WindowId,
}

impl Window {

    pub fn new(shape: Shape, style: Option<(chtype, chtype)>) -> Window {

        let Shape { pos: YX(y, x), size: YX(lines, cols) } = shape;

        let mut w = Window {
            main: newwin(lines, cols, y, x),
            shape: shape,
            child_list: Vec::new(),
            child_hash: HashMap::new(),
            id: WindowId::from("main"),
        };

        if let Some((att1, att2)) = style {
            w.box_(att1, att2);
            // box_(w.main, att1, att2);
        }
        else {
            w.box_(0, 0);
            // box_(w.main, 0, 0);
        }

        w

    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    fn __call_wrapper(code: i32, error: WindowError) -> NcResult {
        if code < 0 { Err(NcError::DrawError(error)) } else { Ok(code) }
    }

    pub fn box_(&mut self, att1: chtype, att2: chtype) -> NcResult {
        Window::__call_wrapper(box_(self.main, att1, att2), WindowError::Boxx)
    }

    pub fn wborder(&mut self, style: Style) -> NcResult {

        let Style {
            ls, rs,
            ts, bs,
            tlc, trc,
            blc, brc,
        } = style;

        let code = wborder(self.main,
            ls, rs,
            ts, bs,
            tlc, trc,
            blc, brc,
        );
        Window::__call_wrapper(code, WindowError::WBorder)
    }

    pub fn wclear(&mut self) -> NcResult {
        Window::__call_wrapper(wclear(self.main), WindowError::WClear)
    }

    pub fn wrefresh(&mut self) -> NcResult {
        Window::__call_wrapper(wrefresh(self.main), WindowError::WRefresh)
    }

    pub fn wnoutrefresh(&mut self) -> NcResult {
        Window::__call_wrapper(wnoutrefresh(self.main), WindowError::WNoutRefresh)
    }

    pub fn redrawwin(&mut self) -> NcResult {
        Window::__call_wrapper(redrawwin(self.main), WindowError::RedrawWin)
    }

    pub fn whline(&mut self, ch: chtype, n: i32) -> NcResult {
        Window::__call_wrapper(whline(self.main, ch, n), WindowError::WHline)
    }

    pub fn wvline(&mut self, ch: chtype, n: i32) -> NcResult {
        Window::__call_wrapper(wvline(self.main, ch, n), WindowError::WVline)
    }

    pub fn mvwhline(&mut self, yx: YX, ch: chtype, n: i32) -> NcResult {
        let YX(y,x) = yx;
        Window::__call_wrapper(mvwhline(self.main, y, x, ch, n), WindowError::MvWHline)
    }

    pub fn mvwvline(&mut self, yx: YX, ch: chtype, n: i32) -> NcResult {
        let YX(y,x) = yx;
        Window::__call_wrapper(mvwvline(self.main, y, x, ch, n), WindowError::MvWVline)
    }

    pub fn mvhline(yx: YX, ch: chtype, n: i32) -> NcResult {
        let YX(y,x) = yx;
        Window::__call_wrapper(mvhline(y,x,ch,n), WindowError::MvHline)
    }

    pub fn mvvline(yx: YX, ch: chtype, n: i32) -> NcResult {
        let YX(y,x) = yx;
        Window::__call_wrapper(mvvline(y,x,ch,n), WindowError::MvVline)
    }

    pub fn wprintw(&mut self, s: &str) -> NcResult {
        Window::__call_wrapper(wprintw(self.main, s), WindowError::WPrintw)
    }

    pub fn mvwprintw(&mut self, yx: YX, s: &str) -> NcResult {
        let YX(y,x) = yx;
        Window::__call_wrapper(mvwprintw(self.main, y, x, s), WindowError::MvWPrintw)
    }

    // fn split_vline(&mut self, x: i32) {
    //     let YX(h,w) = self.size();
    //     let YX(y0,x0) = self.pos();
    //     self.mvwvline(YX(0,x0+x), ACS_TTEE(), 1);
    //     self.mvwvline(YX(1,x0+x), 0, h-2);
    //     self.mvwvline(YX(h-1,x0+x), ACS_BTEE(), 1);
    // }

    // fn split_hline(&mut self, y: i32) {
    //     let YX(h,w) = self.size();
    //     let YX(y0,x0) = self.pos();
    //     self.mvwhline(YX(y0+y,0), ACS_LTEE(), 1);
    //     self.mvwhline(YX(y0+y,1), 0, w-2);
    //     self.mvwhline(YX(y0+y, w-1), ACS_RTEE(), 1);
    // }

    pub fn wattron(&mut self, color_pair: NCURSES_ATTR_T) -> NcResult {
        Window::__call_wrapper(wattron(self.main, color_pair), WindowError::WAttrOn)
    }

    pub fn wattroff(&mut self, color_pair: NCURSES_ATTR_T) -> NcResult {
        Window::__call_wrapper(wattroff(self.main, color_pair), WindowError::WAttrOff)
    }

    // fn wattr_set(&mut self, attr: NCURSES_ATTR_T, cpair: NCURSES_ATTR_T) {
    //     wattr_set(self.main, attr, cpair);
    // }

    pub fn wresize(&mut self, lines: i32, cols: i32) -> NcResult {
        let code = wresize(self.main, lines, cols);
        if code > 0 {
            self.shape.size = YX(lines, cols);
        }
        Window::__call_wrapper(code, WindowError::WResize)
    }

    pub fn mvwin(&mut self, y: i32, x: i32) -> NcResult {
        let code = mvwin(self.main, y, x);
        if code > 0 {
            self.shape.pos = YX(y, x);
        }
        Window::__call_wrapper(code, WindowError::MvWin)
    }

    pub fn subwin(&mut self, shape: &Shape, id: &str) -> Option<Window> {
        let Shape { pos: YX(y, x), size: YX(lines, cols) } = shape;
        let wid = WindowId::from(id);
        let sw = Window{
            main: subwin(self.main, *lines, *cols, *y, *x),
            shape: shape.clone(),
            child_list: Vec::new(),
            child_hash: HashMap::new(),
            id: wid.clone(),
        };
        self.child_list.push(wid.clone());
        self.child_hash.insert(wid, sw)
    }

    pub fn draw_<F>(&mut self, mut draw: F) -> NcResult
    where F: FnMut(&mut Window) -> NcResult
    {
        self.wclear()?;
        draw(self)?;
        self.wrefresh()
    }

    pub fn draw_sw<F>(&mut self, id: &str, draw: F) -> NcResult
        where F: FnMut(&mut Window) -> NcResult
        {
            if let Some(sw) = self.child_hash.get_mut(&WindowId::from(id)) {
                sw.draw_(draw)?;
                sw.wrefresh()?;
                self.touchwin()?;
                self.wrefresh()
            }
            else {
                Ok(0i32)
            }
        }

    fn touchwin(&mut self) -> NcResult {
        Window::__call_wrapper(touchwin(self.main), WindowError::Touchwin)
    }

    fn find_layer(&self, id: &str) -> NcResult {
        let wid = WindowId::from(id);
        let mut index = Err(NcError::DrawError(WindowError::FindLayer));
        for (i, window_id) in self.child_list.iter().enumerate() {
            if *window_id == wid {
                // This is a bad idea that will eventually bite me in the ass
                index = Ok(i as i32)
            }
        }
        index
    }

    pub fn delete_sw(&mut self, id: &str) -> NcResult {
        let wid = WindowId::from(id);
        let wid_position = self.find_layer(id)?;
        self.child_hash.remove(&wid);
        self.child_list.remove(wid_position as usize);
        self.wrefresh()?;
        self.redrawwin()?;
        nc_refresh()
    }

}

impl Drop for Window {
    fn drop(&mut self) {
        delwin(self.main);
    }
}


#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub struct WindowId (String);

impl WindowId {
    pub fn from(s: &str) -> WindowId {
        WindowId(s.to_string())
    }
}


pub struct ScaledWindow {
    pub window: Window,
    vscale: f32,
    voffset: f32,
    hscale: f32,
    hoffset: f32,
}

impl <'a> ScaledWindow {

    pub fn new(shape: Shape, style: Option<(chtype, chtype)>) -> ScaledWindow {

        ScaledWindow {
            window: Window::new(shape, style),
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
    fn pos(&self) -> YX;
    fn size(&self) -> YX;

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
