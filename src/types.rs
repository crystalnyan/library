#[derive(Debug)]
struct  User {
	pub name: String,
	pub role: Roles,
	pub book_taken: Book
}

#[derive(Debug)]
pub  struct Book {
	pub  title: String,
	pub  author: String,
}

enum Roles{
	customer,
	librarian
}