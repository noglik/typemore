extern crate termion;

use std::io::{stdout, Write};
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

fn main() {
    let mut stdin = async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}Hello world!", clear::All, cursor::Goto(1, 1)).unwrap();
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    loop {
        let input = stdin.next();

        if let Some(Ok(key)) = input {
            match key {
                Key::Char(c) => {
                    write!(stdout, "{}{}", color::Fg(color::Green), c).unwrap();
                    stdout.lock().flush().unwrap();
                }
                Key::Ctrl('c') => break,
                _ => {}
            }
        }

        stdout.flush().unwrap();
    }
}
