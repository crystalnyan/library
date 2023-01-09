use std::io;
use console::Term;

pub(crate) fn get_choice() -> u8 {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Can't read the choice TT");

    choice.trim().parse().unwrap_or(0)
}

pub(crate) fn clear_screen() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}

pub(crate) fn continue_screen() {
    println!("Press Enter to continue... ");
    let _ = get_choice();
}