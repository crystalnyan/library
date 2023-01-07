use std::fs::File;
use std::io::Read;

use crate::types::book::Book;
use crate::utils;

pub(crate) fn customer_actions() {
    let choice = utils::get_choice();

    match choice {
        1 => show_books(),
        _ => ()
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