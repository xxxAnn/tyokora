#![allow(dead_code)]
mod utils;
mod text;
mod parser;
mod commands;

use utils::input;
use text::welcome_message;
use parser::parse;

fn main() {
    let resp = input(welcome_message()).expect("Could not read user input");
    parse(&resp);
}