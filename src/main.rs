use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::env;
use std::fs;
use std::io;

use serde::Deserialize;

use queues::*;

use vim_driver::*;

#[allow(non_upper_case_globals)]
static UsageMsg: &str = "[Error]: wrong number of parameters.
  Usage: vim-driver <input-file> <output-file>";

#[derive(Clone)]
enum Key {
    Char(char),
    Digit(u8),
    Left,
    Right,
    Up,
    Down,
    BS
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match self {
            Key::Char(chr)  => chr.to_string(),
            Key::Digit(dig) => dig.to_string(),
            Key::Left       => String::from("<Left>"),
            Key::Right      => String::from("<Right>"),
            Key::Up         => String::from("<Up>"),
            Key::Down       => String::from("<Down>"),
            Key::BS         => String::from("<BS>"),
        }
    }
}

struct State {
    history: Vec<Key>,
    buff: *mut file_buffer
}

impl State {
    fn from_cfg(cfg: &Config) -> State {
        let pos = pos_T {
            lnum: cfg.start_row as i64,
            col: cfg.start_col as i32,
            coladd: 0
        };
        let buff_file_name = cfg.input_file.clone().as_mut_ptr();
        unsafe {
            let buff = vimBufferOpen(buff_file_name, 0, 0);
            vimCursorSetPosition(pos);
            State { history: vec![]
                  , buff
                  }
        }
    }

    fn get_text(&self) -> String {
        vimBufferGetAllText(self.buff)
    }
}

impl Clone for State {
    fn clone(&self) -> State {
        unsafe {
            let mut file_buffer_clone = *self.buff.clone();
            let clone_pointer: *mut file_buffer = &mut file_buffer_clone;
            State { history: self.history.clone()
                  , buff: clone_pointer
                  }
        }
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let txt = vimBufferGetAllText(self.buff);
        txt.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let txt = vimBufferGetAllText(self.buff);
        let other_txt = vimBufferGetAllText(other.buff);
        String::eq(&txt, &other_txt)
    }
}

impl Eq for State { }

#[derive(Debug, Deserialize)]
struct Config {
    input_file: String,
    output_file: String,
    start_col: u16,
    start_row: u16,
}

fn read_input(args: &[String]) -> Result<Config, io::Error> {
    match args {
        [_, json_file] => {
            let json_str = fs::read_to_string(&json_file)?;
            let json: Config = serde_json::from_str(&json_str)?;
            Ok(json)
        }
        _ => panic!("{}", UsageMsg), // should be Err, but must find a way to convert string to
                                     // io::Error
    }
}

fn update_state(state: &State, key: &Key) {
    unsafe {
        vimBufferSetCurrent(state.buff);
        let key_string = key.to_string().as_mut_ptr();
        vimKey(key_string);
    }
}

fn run(state: State, goal_txt: String) -> Vec<Key> {
    let mut queue: Queue<State> = queue![state.clone()];
    // always the state AFTER update_state
    let mut seen: HashSet<State> = HashSet::from([state]);

    let all_lower: Vec<Key>  = ('a'..='z').map(|chr| Key::Char(chr)).collect();
    let all_upper: Vec<Key>  = ('A'..='Z').map(|chr| Key::Char(chr)).collect();
    let all_digits: Vec<Key> = (0..=9).map(|dig| Key::Digit(dig)).collect();
    let dirs                 = vec![Key::Left, Key::Right, Key::Up, Key::Down];
    let other                = vec![Key::BS];
    let all_keys = [ all_lower
                   , all_upper
                   , all_digits
                   , dirs
                   , other
                   ].concat();

    while queue.size() > 0 {
        let curr_state = queue.remove().unwrap();
        if curr_state.get_text() == goal_txt {
            return curr_state.history;
        }
        for key in &all_keys {
            let new_state = curr_state.clone();
            update_state(&new_state, &key);
            if !seen.contains(&new_state) {
                seen.insert(new_state.clone());
                queue.add(new_state).unwrap();
            }
        }
    }
    panic!("couldn't find a path to the desire result");
}

fn start_vim()
{
    let argc = 1;
    let mut argv_string = String::from("vim");
    let mut argv_deref = argv_string.as_mut_ptr() as *mut i8;
    let argv = &mut argv_deref;
    unsafe {
        vimInit(argc, argv);
    }
}

fn main() -> Result<(), io::Error> {
    start_vim();
    let binding: Vec<String> = env::args().collect();
    let args: &[String] = binding.as_slice();
    let cfg: Config = read_input(&args)?;
    let state = State::from_cfg(&cfg);
    let goal_txt = fs::read_to_string(&cfg.output_file)?;
    run(state, goal_txt);
    Ok(())
}
