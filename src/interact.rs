use ncurses::getch;
use ncurses::*;
use std::collections::HashMap;
use std::fs;
use std::path;
use std::char::{self};
use std::str::FromStr;
use std::fmt::Debug;

use crate::draw;

// use back::{*, Mode, State};
use draw::{*}; //DrawResult;
// use draw::DrawResult;
use draw::windows::{*};

// mod rawkeys;
// pub mod action_mapper;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Key(pub char);

mod vars {
    use std::env;
    pub fn HOME() -> String {
        env::var("HOME").unwrap()
    }
} /* vars */


pub fn interact() -> DrawResult {

    refresh();
    let ch = getch() as u8 as char;
    let YX(height, width) = screen_size() - YX(2,0);

    match ch {
        'q' => {println!("EXIT"); Err(DrawError::CleanExit)},
         _  => {mvprintw(height+1i32,width-20i32, &format!("{:?}\n",ch)); Ok(())},
        // _ => Ok(())
    }

}


#[derive(Debug)]
pub enum Fun {
    F1, F2, F3, F4,
    F5, F6, F7, F8,
    F9, F10, F11, F12,
}

impl Fun {
    pub fn from(ch: i32) -> Option<Fun> {
        match ch {
            KEY_F1 => Some(Fun::F1),
            KEY_F2  => Some(Fun::F2),
            KEY_F3  => Some(Fun::F3),
            KEY_F4  => Some(Fun::F4),
            KEY_F5  => Some(Fun::F5),
            KEY_F6  => Some(Fun::F6),
            KEY_F7  => Some(Fun::F7),
            KEY_F8  => Some(Fun::F8),
            KEY_F9  => Some(Fun::F9),
            KEY_F10 => Some(Fun::F10),
            KEY_F11 => Some(Fun::F11),
            KEY_F12 => Some(Fun::F12),
                _ => None
        }
    }
}

#[derive(Debug)]
pub enum Mod {
    Ctrl,
    Shift,
    Alt,
    Super
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
pub enum Symbol {
}

// pub struct Mapping{
//     mode: Mode,
//     keyseq: Vec<String>,
//     action: Action
// }

// impl Mapping {
//     pub fn new(mode: Mode, keyseq: Vec<String>, action: Action) -> Mapping {
//         Mapping {
//             mode,
//             keyseq,
//             action
//         }
//     }
// }

pub enum Action {
    Move(Direction),
    Copy_,
    Cut,
    Paste,
    Rename,
}

fn parse_command_file() -> HashMap<String,String> {

    let home = vars::HOME();
    let home_path = path::Path::new(&home);
    let config_path = home_path.join(".config/effeme");
    let file_text = fs::read_to_string(config_path.join("effeme.conf"))
        .expect("Can't read file")
        .to_string();

    let mut command : HashMap<_,_> = HashMap::new();

    let lines = file_text.lines(); // splits &str into newlines
    for mut line in lines {

        let collection : Vec<&str> = line.split(" ").collect();
        let mut coll = collection.to_vec();
        command.insert(coll[0].to_string(),coll[1].to_string());
    };

    command
}
