use std::io::Stdout;

use crossterm::style::Color;
use crossterm::{QueueableCommand, Result};
use crossterm::terminal::size;
use crossterm::cursor::MoveTo;

use super::{Label, Widget, Border};
use crate::Pos;

pub struct StatusWidget {
    pub hitpoints: Hitpoints,
}

impl Widget for StatusWidget {
    fn draw(&mut self, stdout: &mut Stdout) -> Result<()> {
        let (w, _) = size()?;
        let mut border = Border::new(Pos::zero(), w, 3, ",-,|'-`|");
        border.draw(stdout)?;
        stdout.queue(MoveTo(1, 1))?;
        self.hitpoints.draw(stdout)?;

        Ok(())
    }
}

pub struct Hitpoints {
    pub current: u16,
    pub max: u16,
}

impl Widget for Hitpoints {
    fn draw(&mut self, stdout: &mut Stdout) -> Result<()> {
        let p = (self.current as f32 / self.max as f32) * 100.0;

        let color = if p < 10.0 {
            // almost dead
            Color::Red
        } else if p < 40.0 {
            Color::Yellow
        } else {
            Color::White
        };

        let text = format!("{}/{}", self.current, self.max);
        let mut label = Label::new(&text, color);
        label.draw(stdout)?;

        Ok(())
    }
}
