extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, event};

const CORRECT: color::Green = color::Green;
const DANGER: color::Red = color::Red;
const DEFAULT: color::Reset = color::Reset;

pub fn read_keys(text: String) {
    let length = text.chars().count();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{clear}{pos}{text}",
        clear = clear::All,
        pos = cursor::Goto(1, 1),
        text = text
    )
    .unwrap();

    write!(stdout, "{pos}", pos = cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let (x, _y) = stdout.cursor_pos().unwrap();
        let key = c.unwrap();

        match key {
            event::Event::Key(Key::Char(c)) => {
                if c == text.chars().nth((x - 1).into()).unwrap() {
                    write!(stdout, "{color}{}", c, color = color::Fg(CORRECT)).unwrap();
                } else {
                    write!(stdout, "{color}{}", c, color = color::Fg(DANGER)).unwrap();
                }
                stdout.flush().unwrap();
            }
            event::Event::Key(Key::Backspace) => {
                if x != 1 {
                    write!(
                        stdout,
                        "{pos}{color}{}{pos}",
                        text.chars().nth((x - 2).into()).unwrap(),
                        pos = cursor::Goto(x - 1, 1),
                        color = color::Fg(DEFAULT)
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                }
            }
            event::Event::Key(Key::Ctrl('c')) => break,
            _ => {}
        }

        if usize::from(x) > length - 1 {
            write!(
                stdout,
                "{pos}{color}Thanks!",
                pos = cursor::Goto(1, 2),
                color = color::Fg(DEFAULT)
            )
            .unwrap();
            stdout.flush().unwrap();
            break;
        }

        stdout.flush().unwrap();
    }
}
