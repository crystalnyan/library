use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use crate::types::book::Book;

use crate::utils;

pub(crate) fn librarian_actions() {
    let choice = utils::get_choice();

    match choice {
        1 => show_books(),
        2 => add_books(),
        _ => ()
    }
}

fn show_books() {
    let books = get_all();
    Book::print_collection(books);
}

fn get_all() -> Vec<Book> {
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
                books.push(Book::from_tuple(&title,&author,quantity))
            },
            _ => ()
        }
        count = count + 1;
    }

    books
}

fn add_books() {
    println!("How many different book titles you want to add?");
    let count = utils::get_choice();

    for _ in 0..count {
        let (mut title, mut author) = (String::new(), String::new());
        println!("Enter title: ");
        io::stdin().read_line(&mut title).expect("Something went wrong...");
        println!("Enter author name: ");
        io::stdin().read_line(&mut author).expect("Something went wrong...");

        let mut quantity: u8 = 0;
        loop {
            println!("Enter quantity: ");
            let mut quantity_string= String::new();
            io::stdin().read_line(&mut quantity_string).expect("Something went wrong...");
            match quantity_string.trim().parse() {
                Ok(q) => {
                    match q {
                        q if q > 0 => {quantity = q;},
                        _ => {
                            println!("Quantity should be greater than 0!");
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("Could not parse. Please enter a number: ");
                    continue;
                }
            };
            break;
        }

        let book = Book::new(title, author, quantity);
        let record = book.to_string();

        let mut file = match OpenOptions::new().append(true).open("library.txt") {
            Ok(file) => file,
            Err(_) => File::create("library.txt").unwrap()
        };

        file.write(record.as_ref()).expect("Write operation to file failed");
    }
}