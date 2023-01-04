use std::fs::File;
use std::io::Read;

use crate::utils;

pub(crate) fn customer_actions() {
    let choice = utils::get_choice();

    match choice {
        1 => show_books(),
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