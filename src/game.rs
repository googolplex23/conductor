use std::Vec;
use std::fs::File;
use json;

pub struct board 

pub fn load_board(filepath: String) -> board {
    let mut file = File.open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let parsed = json::parse(contents);
    println!("{}", parsed.pretty(4);
}