
use crate::draw;
use crate::interact;

use interact::interact;
use draw::{DrawResult,DrawError};


pub fn start_interface() -> DrawResult {

    while draw_interface().is_ok() { }
    Err(DrawError::CleanExit)

}

pub fn draw_interface() -> DrawResult {

    // if let Some(cwd) = state.current() {
    //     if let Some(ref mut s) = cwd.to_str() {
    //         mvprintw(0,0,&format!("{} {:?}\n",s, screen_size()));
    //     }
    //     else{
    //         printw("Can't parse dir name");
    //     }
    // }

    let mut main_w = draw::main_window();
    draw::sub_window(&mut main_w.window);
    interact()

}

