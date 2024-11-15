use termion::{color, event::Key, input::TermRead, raw::IntoRawMode, style, terminal_size};
use std::io::{stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut stdout= stdout().into_raw_mode().unwrap();
    write!(stdout, "Hello there").unwrap(); 
    println!("{}{}q to exit. Type stuff, use alt, and so on.{}", termion::clear::All, termion::cursor::Goto(1,1), termion::cursor::Hide);

    let mut todos = vec!["Learn Rust", "Learn Termion"];

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('c') => println!("typed C"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
