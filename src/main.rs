use std::env;
use std::fs;
use std::io;

use serde::Deserialize;

#[allow(non_upper_case_globals)]
static UsageMsg: &str =
  "[Error]: wrong number of parameters.
  Usage: vim-driver <input-file> <output-file>";

enum Key {
    LowH,
    LowJ,
    LowK,
    LowL
}

struct State {
    text: String,
    cursor_col: u16,
    cursor_row: u16,
    pressed: Vec<Key>
}

impl State {
    fn from_cfg(cfg: &Config) -> Result<State, io::Error> {
        Ok(State {
            text: fs::read_to_string(&cfg.input_file)?,
            cursor_col: cfg. start_col,
            cursor_row: cfg. start_row,
            pressed: vec![]
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

fn run(cfg: &Config) -> Vec<Key> {
    let state = State::from_cfg(cfg);
    vec![]
}

fn main() -> Result<(), io::Error> {
    let binding: Vec<String> = env::args().collect();
    let args: &[String] = binding.as_slice();
    let json_text = read_input(&args)?;
    let cfg: Config = serde_json::from_str(&json_text)?;

    run(&cfg);

    Ok(())
}
