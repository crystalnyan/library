use std::fs::File;
use std::io::Read;
use std::process::exit;

use crate::types::book::Book;
use crate::{menu, utils};

pub(crate) fn customer_actions() {
    loop {
        utils::clear_screen();
        menu::customer();
        let choice = utils::get_choice();

        match choice {
            1 => {
                utils::clear_screen();
                show_books();
                println!("Press Enter to continue: ");
                let _ = utils::get_choice();
            }
            3 => exit(1),
            _ => {
                println!("No such option! Try again.");
                continue;
            }
        }
        println!("What you wanna do?");
    }

}

fn show_books() {
    let books = get_available();
    Book::print_collection(books);
}

fn get_available() -> Vec<Book> {
    let mut file = match File::open("library.txt"){
        Ok(f) => f,
        Err(_) => {
            println!("No books existing!");
            return vec![]
        }
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut books = vec![];
    let (mut title, mut author, mut quantity) = (String::new(), String::new(), String::new());

    let mut count = 1;
    for line in buffer.to_string().lines() {
        match count % 3 {
            1 => title = line.to_string(),
            2 => author = line.to_string(),
            0 => {
                quantity = line.to_string();
                if quantity.trim().parse::<u8>().unwrap() > 0 {
                    books.push(Book::from_tuple(&title,&author,quantity))
                }
            },
            _ => ()
        }
        count = count + 1;
    }

    books
}