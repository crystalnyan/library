use std::io;

pub mod types;
pub mod utils;

fn main() {
  //menu with entering/leaving library
  //1-enter as a user 2-enter as a librarian 3-exit
  utils::show_default_menu();

  let mut choice = utils::get_choice();

  loop {

    match  choice {
      Some(num) => {
        if num == 1 { };
        if num ==2  { };
        if num ==3 { break; } ;
      },
      None => ()
    }

  }

  println!("Goodbye TT");
}
