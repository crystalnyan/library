pub(super) fn default() {
    println!(
        "Welcome to meow library!~\n\
        What you wanna do?\n\
        1. Enter library as a customer\n\
        2. Enter as a librarian\n\
        3. Leave");
}

pub(super) fn customer() {
    println!(
        "Customer!~\n\
        What you wanna do?\n\
        1. See all available books!\n\
        2. Reserve a book\n\
        3. Leave"
    );
}

pub(super) fn librarian() {
    println!(
        "Librarian!~\n\
        What you wanna do?\n\
        1. See all books!\n\
        2. Add a book\n\
        3. Remove a book\n\
        4. Leave"
    );
}