use std::mem::needs_drop;

pub  mod  types;

fn main() {
    let  book = types::Book {
      title: String::from("smth"),
      author: String::from("meow")
    };
    println!("{:#?}", book);
}
