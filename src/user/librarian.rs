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
    let mut file = File::open("library.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut count = 1;
    for line in buffer.to_string().lines() {
        if count % 3 != 0 {
            print!("{} | ", line);
        } else {
            print!("{}\n", line);
        }
        count = count + 1;
    }
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
                    quantity = q;
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