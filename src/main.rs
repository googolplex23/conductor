
mod game;

use game::load_board;


fn main() {
	//std::env::set_current_dir(std::env::current_exe().expect("no working directory"));

    println!("{}", std::env::current_dir().expect("no working directory").display());
	game::load_board("usa.json");
}
