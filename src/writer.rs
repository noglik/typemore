extern crate termion;

use std::io::{stdin, stdout, Write, Stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::cursor::DetectCursorPos;
use termion::{clear, color, cursor, event};

use crate::story::{Story, Status};

const CORRECT: color::Green = color::Green;
const DANGER: color::Red = color::Red;
const DEFAULT: color::Reset = color::Reset;

pub struct View {
    stdout: RawTerminal<Stdout>,
    story:  Story,
    position: (u16, u16),
}

impl View {
    pub fn new(story: Story) -> View {
        let out = stdout().into_raw_mode().unwrap();
        let mut view = View { stdout: out, story, position: (1, 1) };

        // move to the  start for now
        write!(view.stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), view.story.get_text()).unwrap();
        write!(view.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        view.stdout.flush().unwrap();

        view
    }

    // TODO: add stats about accuracy and speed
    pub fn read(&mut self) -> () {
        for c in stdin().events() {
            let key = c.unwrap();

            match key {
                event::Event::Key(Key::Char(c)) => {
                    self.write_next(c);
                },
                event::Event::Key(Key::Backspace) => {
                    self.reset_prev();        
                },
                event::Event::Key(Key::Ctrl('c')) => break,
                _ => {}
            }
            self.stdout.flush().unwrap();

            if self.story.status == Status::Done {
                break;
            }

            // update position
            self.position = self.stdout.cursor_pos().unwrap();
        }
    }

    pub fn cleanup(&mut self) -> () {
        self.stdout.suspend_raw_mode().unwrap()
    }

    fn write_next(&mut self, c: char) -> () {
        match self.story.check_cur_char(c) {
            true => write!(self.stdout, "{}{}", color::Fg(CORRECT), c).unwrap(),
            false => write!(self.stdout, "{}{}", color::Fg(DANGER), c).unwrap(),
        };
        self.story.move_forward();
    }

    fn reset_prev(&mut self) -> () {
        match self.story.get_prev_char() {
            Some(c) => {
                write!(self.stdout, "{pos}{color}{c}{pos}", pos = cursor::Goto(self.position.0 - 1, self.position.1), color = color::Fg(DEFAULT), c = c).unwrap(); 
                self.story.move_backward();

            },
            None => (),
        }
    }
}
