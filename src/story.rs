#[derive(Debug,PartialEq)]
pub enum Status {
    InProgress,
    Done,
}

#[derive(Debug)]
pub struct Story {
    text: String,
    length: usize,
    position: usize,
    pub status: Status
}

impl Story {
    pub fn new(text: &str) -> Story {
        Story { text: String::from(text), length: text.len(), position: 0, status: Status::InProgress }
    }

    fn move_pos(&mut self, new_position: usize) -> usize {
        if new_position >= self.length {
            self.position = self.length;
            self.status = Status::Done;
        }

        self.position = new_position;
        self.position
    }

    pub fn move_forward(&mut self) -> usize  {
        // position should never be bigger than length, so default to it here
        self.move_pos(self.position.checked_add(1).unwrap_or(self.length))
    }

    pub fn move_backward(&mut self) -> usize {
        self.move_pos(self.position.checked_sub(1).unwrap_or(0))
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    fn get_cur_char(&self) -> Option<char> {
        self.text.chars().nth(self.position)
    }

    pub fn get_prev_char(&self) -> Option<char> {
        self.text.chars().nth(self.position.checked_sub(1).unwrap_or(0))
    }

    pub fn check_cur_char(&self, cur_char: char) -> bool {
        match self.get_cur_char() {
            Some(c) if c == cur_char => true,
            Some(_) => false,
            // no next char -> whole text is finished
            None => true, 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cur_char_for_random() {
        let mut story = Story::new("hello");
        story.position = 3;

        assert!(story.check_cur_char('l'));
    }

    #[test]
    fn get_prev_char_first_char() {
        let story = Story::new("hello");
        assert_eq!(story.get_prev_char().unwrap(), 'h');
    }
}

