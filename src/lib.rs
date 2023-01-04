mod menu;
mod user;
mod utils;

pub fn run() {
    menu::default();

    let mut choice = utils::get_choice();

    loop {
        match choice {
            1 => {
                menu::customer();
                user::customer::customer_actions();
                choice = utils::get_choice();
                break;
            }
            2 => {
                println!("You are a librarian");
                user::librarian::add_books();
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