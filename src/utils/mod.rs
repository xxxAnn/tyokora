use std::io;
use std::io::*;
use std::{thread, time};

    pub fn input(prompt: Vec<String>) -> io::Result<String> {
    for fragment in prompt.iter() {
        thread::sleep(time::Duration::from_millis(200));
        print!("{}", fragment.as_str());
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