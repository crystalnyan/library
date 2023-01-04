pub  mod  types;
use crate::types::Book;

use std::io;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn show_default_menu() {
  println!(
    "Welcome to meow library!~\n\
        What you wanna do?\n\
        1. Enter library as a customer\n\
        2. Enter as a librarian\n\
        3. Leave");
}

pub fn show_customer_menu() {
  println!(
    "Customer!~\n\
        What you wanna do?\n\
        1. See all available books!\n\
        2. \n\
        3. Leave"
  );
}

pub  fn  customer_actions() {
  let  choice = get_choice();

  match choice {
    1 => show_books(),
    _ => ()
  }
}

fn add_books() {
  println!("How many different book titles you want to add?");
  let mut count = get_choice();

  for i in 0..count {
    let (mut title, mut author, mut quantity) = (String::new(), String::new(), String ::new());
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

pub  fn  show_books() {
  let  mut books: Vec<Book> = vec![
    Book{
      title: String::from("Harry Potter"),
      author: String::from("Joan K. Rowling"),
      quantity: 3},
  ];

  // closure!
  books.sort_by_key(|b| b.quantity);

  for b in books {
    println!("{} by {}: {}",b.title, b.author, b.quantity);
  }
}

pub fn get_choice() -> u8 {
  let mut choice = String::new();

  io::stdin()
          .read_line(&mut choice)
          .expect("Can't read the choice TT");

  choice.trim().parse().unwrap_or(0)
}

pub fn run() {
  //menu with entering/leaving library
  //1-enter as a user 2-enter as a librarian 3-exit
  show_default_menu();

  let mut choice = get_choice();

  loop {
    match choice {
      1 => {
        show_customer_menu();
        customer_actions();
        choice = get_choice();
        break;
      }
      2 => {
        println!("You are a librarian");
        add_books();
        break;
      }
      3 => {
        println!("Bye!");
        break;
      }
      _ => {
        println!("We dont have this Option :( Try again");
        choice = get_choice();
      }
    }
  }

  println!("Goodbye TT");
}