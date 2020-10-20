use std::io::Stdout;

use crossterm::style::{Color, Print, SetForegroundColor, ResetColor};
use crossterm::{Result, QueueableCommand};
use crossterm::cursor::MoveTo;

mod status;
mod context;

pub use context::Context;
use status::{StatusWidget, Hitpoints};
use crate::Pos;

pub trait Widget {
    fn draw(&self, stdout: &mut Stdout) -> Result<()>;
}

// -----------------------------------------------------------------------------
//     - Widgets -
// -----------------------------------------------------------------------------
pub struct Border {
    pos: Pos,
    width: u16,
    height: u16,

    top_left: char,
    top: char,
    top_right: char,
    right: char,
    bottom_right: char,
    bottom: char,
    bottom_left: char,
    left: char,
}

impl Border {
    pub fn new(pos: Pos, width: u16, height: u16, border: &str) -> Self {
        if border.len() < 8 {
            panic!("Not enough chars to make a border");
        }

        let mut chars = border.chars();

        Self {
            pos,
            width,
            height, 

            top_left: chars.next().unwrap(),
            top: chars.next().unwrap(),
            top_right: chars.next().unwrap(),
            right: chars.next().unwrap(),
            bottom_right: chars.next().unwrap(),
            bottom: chars.next().unwrap(),
            bottom_left: chars.next().unwrap(),
            left: chars.next().unwrap(),
        }
    }
}

impl Widget for Border {
    fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        // Top left
        stdout.queue(MoveTo(self.pos.x, self.pos.y))?;
        stdout.queue(Print(self.top_left))?;

        // Top
        stdout.queue(MoveTo(self.pos.x + 1, self.pos.y))?;
        let width = (self.width - 2) as usize;
        stdout.queue(Print(self.top.to_string().repeat(width)))?;

        // Top right
        stdout.queue(MoveTo(self.pos.x + self.width - 1, self.pos.y))?;
        stdout.queue(Print(self.top_right))?;

        // Bottom left
        stdout.queue(MoveTo(self.pos.x, self.pos.y + self.height - 1))?;
        stdout.queue(Print(self.bottom_left))?;

        // Bottom
        stdout.queue(MoveTo(self.pos.x + 1, self.pos.y + self.height - 1))?;
        let width = (self.width - 2) as usize;
        stdout.queue(Print(self.bottom.to_string().repeat(width)))?;

        // Bottom right
        stdout.queue(MoveTo(self.pos.x + self.width - 1, self.pos.y + self.height - 1))?;
        stdout.queue(Print(self.bottom_right))?;

        (1..self.height - 1).for_each(|i| {
            let _ = stdout.queue(MoveTo(self.pos.x, self.pos.y + i));
            let _ = stdout.queue(Print(self.left));
            let _ = stdout.queue(MoveTo(self.pos.x + self.width - 1, self.pos.y + i));
            let _ = stdout.queue(Print(self.right));
        });

        Ok(())
    }
}

pub struct Label<'a> {
    text: &'a str,
    color: Color,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str, color: Color) -> Self {
        Self {
            text,
            color,
        }
    }
}

impl<'a> Widget for Label<'a> {
    fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        stdout.queue(SetForegroundColor(self.color))?;
        stdout.queue(Print(self.text))?;
        stdout.queue(ResetColor)?;
        Ok(())
    }
}

pub fn draw_something(stdout: &mut Stdout) -> Result<()> {
    let status = StatusWidget {
        hitpoints: Hitpoints { current: 52, max: 200 }
    };
    status.draw(stdout)?;
    
    Ok(())
}
