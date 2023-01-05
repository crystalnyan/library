use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};

use crate::utils;

pub(crate) fn show_books() {
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

pub(crate) fn add_books() {
    println!("How many different book titles you want to add?");
    let count = utils::get_choice();

    for _ in 0..count {
        let (mut title, mut author, mut quantity) = (String::new(), String::new(), String::new());
        println!("Enter title: ");
        io::stdin().read_line(&mut title).expect("Something went wrong...");
        println!("Enter author name: ");
        io::stdin().read_line(&mut author).expect("Something went wrong...");
        println!("Enter quantity: ");
        io::stdin().read_line(&mut quantity).expect("Something went wrong...");

        // TODO: check quantity to be valid int

        let record = title + &author + &quantity;

        let mut file = match OpenOptions::new().append(true).open("library.txt") {
            Ok(file) => file,
            Err(_) => File::create("library.txt").unwrap()
        };

        file.write(record.as_ref()).expect("Write operation to file failed");
    }
}