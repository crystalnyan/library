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