use std::io;
use std::string::ParseError;

pub fn show_default_menu() {
  println!(
	"Welcome to meow library!~\n\
        What you wanna do?\n\
        1. Enter library as a customer\n\
        2. Enter as a librarian\n\
        3. Leave");
}

pub fn get_choice() -> u8 {
  let mut choice = String::new();

  io::stdin()
	  .read_line(&mut choice)
	  .expect("Can't read the choice TT");

  choice.trim().parse().unwrap_or(0)
}