use super::Context;
use std::fs;
use crate::utils::{load_skya, dump_skya, Skya};

pub fn exec(ctx: Context) {
    let contents = fs::read_to_string("D:/Documents/Code/Personal/tyokora/data/dico.skya").expect("Error reading file");
    let mut data = load_skya(&contents);
    if ctx.subcommands.get(0).expect("Expected at least one subcommand") == &String::from("add") {
        add(ctx, data);
    } else if ctx.subcommands.get(0).expect("Expected at least one subcommand") == &String::from("get") {
        get(ctx, data);
    }
}

pub fn add(ctx: Context, mut data: Skya) {
    let params = ctx.parameters.clone();
    for param in params {
        data.add( param.key(), param.value());
    }
    let string = dump_skya(data);
    fs::write("D:/Documents/Code/Personal/tyokora/data/dico.skya", &string).unwrap();
    println!("{}", "Successful");
}

pub fn get(ctx: Context, data: Skya) {
    let found = data.get(ctx.get_param_from_key("key").unwrap().value());
    println!("{}", found.unwrap());
}