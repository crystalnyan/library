pub  mod  types;
use crate::types::Book;

use std::io;

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