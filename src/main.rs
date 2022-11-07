pub mod types;
pub mod utils;

fn main() {
  //menu with entering/leaving library
  //1-enter as a user 2-enter as a librarian 3-exit
  utils::show_default_menu();

  let mut choice = utils::get_choice();

  loop {
	match choice {
	  1 => {
		println!("You are a customer");
		break;
	  }
	  2 => {
		println!("You are a librarian");
		break;
	  }
	  3 => {
		println!("Bye!");
		break;
	  }
	  _ => {
		println!("We dont have this Option :( Try again");
		choice = utils::get_choice();
	  }
	}
  }

  println!("Goodbye TT");
}
