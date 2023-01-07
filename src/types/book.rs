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
}

impl ToString for Book {
    fn to_string(&self) -> String {
        let record =
            "\n".to_string()
            + &self.title + &(self.author) + &(self.quantity.to_string());
        String::from(record)
    }
}