use std::io::Stdout;

use crossterm::style::Print;
use crossterm::{Result, QueueableCommand};
use crossterm::cursor::MoveTo;

use crate::draw::Widget;

pub struct DebugLines {
    lines: Vec<String>,
    offset_y: u16,
}

impl DebugLines {
    pub fn new(offset_y: u16) -> Self {
        Self {
            lines: Vec::new(),
            offset_y,
        }
    }

    pub fn push(&mut self, s: String) {
        self.lines.push(s);
        while self.lines.len() > 10 {
            self.lines.remove(0);
        }
    }
}

impl Widget for DebugLines {
    fn draw(&mut self, stdout: &mut Stdout) -> Result<()> {
        self.lines.iter().enumerate().for_each(|(y, line)| {
            stdout.queue(MoveTo(0, y as u16 + self.offset_y)).unwrap();
            stdout.queue(Print(line)).unwrap();
        });
        Ok(())
    }
}
