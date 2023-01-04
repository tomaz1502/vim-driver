use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

use serde::Deserialize;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use queues::*;

#[allow(non_upper_case_globals)]
static UsageMsg: &str =
  "[Error]: wrong number of parameters.
  Usage: vim-driver <input-file> <output-file>";

#[derive(PartialEq, Eq, Debug, EnumIter, Clone, Hash)]
enum Key {
    LowH,
    LowJ,
    LowK,
    LowL
}

#[derive(PartialEq, Eq, Clone, Hash)]
enum Mode {
    Normal,
    Insert,
    Visual
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct State {
    text: String,
    cursor_col: u16,
    cursor_row: u16,
    pressed: Vec<Key>,
    history: Vec<Key>,
    mode: Mode
}

impl State {
    fn from_cfg(cfg: &Config) -> Result<State, io::Error> {
        Ok(State {
            text: fs::read_to_string(&cfg.input_file)?,
            cursor_col: cfg. start_col,
            cursor_row: cfg. start_row,
            pressed: vec![],
            history: vec![],
            mode: Mode::Normal
        })
    }
}


#[derive(Debug, Deserialize)]
struct Config {
    input_file: String,
    output_file: String,
    start_col: u16,
    start_row: u16
}

fn read_input(args: &[String]) -> Result<String, io::Error> {
    match args {
        [_, json_file] =>
            Ok(fs::read_to_string(&json_file)?),
        _ => panic!("{}", UsageMsg) // should be err, but must find a way to convert string to
                                    // io::Error
    }
}

// reads the keys pressed and checks if it will perform an action
fn update_state(state: &mut State) {
    unimplemented!()
}

fn run(state: State, goal_txt: String) -> Vec<Key> {
    let mut queue: Queue<State> = queue![state.clone()];
    // always the state AFTER update_state
    let mut seen: HashSet<State> = HashSet::from([state]);

    while queue.size() > 0 {
        let curr_state = queue.remove().unwrap();
        if curr_state.text == goal_txt {
            assert!(curr_state.pressed.is_empty());
            return curr_state.history;
        }
        for key in Key::iter() {
            let mut new_state = curr_state.clone();
            new_state.pressed.push(key);
            update_state(&mut new_state);
            if !seen.contains(&new_state) {
                seen.insert(new_state.clone());
                queue.add(new_state).unwrap();
            }
        }
    }
    panic!("couldn't find a path to the desire result");
}

fn main() -> Result<(), io::Error> {
    let binding: Vec<String> = env::args().collect();
    let args: &[String] = binding.as_slice();
    let json_text = read_input(&args)?;
    let cfg: Config = serde_json::from_str(&json_text)?;
    let mut state = State::from_cfg(&cfg)?;
    let goal_txt = fs::read_to_string(&cfg.output_file)?;
    run(state, goal_txt);

    for key in Key::iter() {
        println!("{:?}", key);
    }


    Ok(())
}
