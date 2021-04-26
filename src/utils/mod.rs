mod skya;

pub use skya::{dump_skya, load_skya, Pair, Skya};

use std::io;
use std::io::*;
use std::{thread, time};
use colored::{Color, ColoredString};

pub fn input(prompt: Vec<ColoredString>) -> io::Result<String> {
    for fragment in prompt.iter() {
        thread::sleep(time::Duration::from_millis(200));
        print!("{}", fragment);
        io::stdout().flush()?;
    }
    println!();
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}