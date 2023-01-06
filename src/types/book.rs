pub struct Book {
    title: String,
    author: String,
    quantity: i32,
}

impl Book {
    pub fn new(title: String, author: String, quantity: i32) -> Book {
        Book {
            title,
            author,
            quantity
        }
    }
}

impl ToString for Book {
    fn to_string(&self) -> String {
        String::from(self.title.clone() + &(self.author) + &(self.quantity.to_string()))
    }
}