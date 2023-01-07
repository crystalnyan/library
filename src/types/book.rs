pub struct Book {
    title: String,
    author: String,
    quantity: u8,
}

impl Book {
    pub fn new(title: String, author: String, quantity: u8) -> Book {
        Book {
            title,
            author,
            quantity
        }
    }

    pub fn get_title (&self) -> String {self.title.clone()}
    pub fn get_author (&self) -> String {self.author.clone()}
    pub fn get_quantity (&self) -> String {self.quantity.to_string()}

    pub fn from_tuple(title: &String, author: &String, quantity: String) -> Book {
        let quantity = quantity.trim().parse().unwrap();
        Book::new(title.clone(), author.clone(), quantity)
    }

    pub fn print_collection(books: Vec<Book>) {
        for book in books {
            let title = book.get_title();
            let author = book.get_author();
            let quantity = book.get_quantity();
            print!("{} | {} | {}\n", title, author, quantity);
        }
    }
}

impl ToString for Book {
    fn to_string(&self) -> String {
        let record = self.title.clone() + &(self.author) + &(self.quantity.to_string()) + "\n";
        String::from(record)
    }
}