/*
json/board format:
Color Codes:
	0 - gray
	1 - red
	2 - orange
	3 - yellow
	4 - green
	5 - blue
	6 - pink
	7 - white
	8 - black
*/



use std::vec;
use std::fs;
use std::string::String;
use json;

#[repr(C)]
pub struct ticket {
	start: u8,
	end: u8,
	points: u16,
}

#[no_mangle]
pub extern "C" fn load_board(filepath: &str){
    let contents = std::fs::read_to_string(filepath);
	let mut parsed = json::JsonValue::new_object();
	match contents {
		Ok(content) => parsed = json::parse(content.as_str()).unwrap(),
		_ => panic!("File not found in {}", std::env::current_dir().expect("no working directory").display()),
	}
	let mut tickets = std::vec::Vec::new();
	for ticket in parsed["tickets"].iter() {
		()
	}
	
	
	println!("{}",parsed.pretty(4));
	
	
}