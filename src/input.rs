use std::io;

use crossterm::{
    cursor::MoveLeft,
    event::{Event, KeyCode},
    queue,
    style::Print,
};

use crate::Widget;

/// Helper struct to emulate traditional
/// user input in a raw-mode terminal environment
pub struct RawInput {
    value: String,
    cursor: u16,
}

impl RawInput {
    pub fn new() -> Self {
        RawInput {
            value: String::new(),
            cursor: 0,
        }
    }
    pub fn with_value(&mut self, value: &str) -> &mut Self {
        self.value = value.to_owned();
        self
    }
}

impl RawInput {
    /// Sets the value of the input and sets the cursor
    /// position to the end of the string
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_owned();
        self.cursor = self.value.len() as u16;
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl RawInput {
    /// Handles an incoming event and updates internal state accordingly
    pub fn handle_event(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            match key.code {
                // Input
                KeyCode::Char(c) => {
                    self.value.insert(self.cursor as usize, c);
                    self.cursor += 1;
                }
                // Blanket navigation
                KeyCode::Home => self.cursor = 0,
                KeyCode::End => self.cursor = self.value.len() as u16,

                // Scrubbing
                KeyCode::Left if self.cursor > 0 => self.cursor -= 1,
                KeyCode::Right if (self.cursor as usize) < self.value.len() => self.cursor += 1,

                // Deletion
                KeyCode::Backspace if self.cursor > 0 => {
                    self.value.remove(self.cursor as usize - 1);
                    self.cursor -= 1;
                }
                KeyCode::Delete if (self.cursor as usize) < self.value.len() => {
                    self.value.remove(self.cursor as usize - 1);
                }
                _ => {}
            }
        }
    }
}

impl Widget for RawInput {
    fn draw<W: io::Write>(&self, term: &mut W) -> io::Result<()> {
        let move_left: u16 = (self.value.len() - self.cursor as usize)
            .try_into()
            .expect("failed to move cursor");

        queue!(term, Print(&self.value), MoveLeft(move_left))
    }
}
