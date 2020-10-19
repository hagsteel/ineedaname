use std::io::{self, Write as _, Stdout};
use std::fmt::{self, Write as _ };

use crossterm::style::Print;
use crossterm::{Result, QueueableCommand};
use crossterm::cursor::MoveTo;
use crossterm::terminal::size;

struct Border {
    x: u16,
    y: u16,
    width: u16,
    height: u16,

    top_left: char,
    top_right: char,
    top: char,
    bottom_left: char,
    bottom_right: char,
    bottom: char,
    left: char,
    right: char,
}

impl Border {
    pub fn new(x: u16, y: u16, width: u16, height: u16, border: &str) -> Self {
        if border.len() < 8 {
            panic!("Not enough chars to make a border");
        }

        let mut chars = border.chars();

        Self {
            x,
            y,
            width,
            height, 

            top_left: chars.next().unwrap(),
            top_right: chars.next().unwrap(),
            top: chars.next().unwrap(),
            bottom_left: chars.next().unwrap(),
            bottom_right: chars.next().unwrap(),
            bottom: chars.next().unwrap(),
            left: chars.next().unwrap(),
            right: chars.next().unwrap(),
        }
    }

    fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        // Top left
        stdout.queue(MoveTo(self.x, self.y))?;
        stdout.queue(Print(self.top_left))?;

        // Top
        stdout.queue(MoveTo(self.x + 1, self.y))?;
        let width = (self.width - 2) as usize;
        stdout.queue(Print(self.top.to_string().repeat(width)))?;

        // Top right
        stdout.queue(MoveTo(self.x + self.width - 1, self.y))?;
        stdout.queue(Print(self.top_right))?;

        // Bottom left
        stdout.queue(MoveTo(self.x, self.y + self.height - 1))?;
        stdout.queue(Print(self.bottom_left))?;

        // Bottom
        stdout.queue(MoveTo(self.x + 1, self.y + self.height - 1))?;
        let width = (self.width - 2) as usize;
        stdout.queue(Print(self.bottom.to_string().repeat(width)))?;

        // Bottom right
        stdout.queue(MoveTo(self.x + self.width - 1, self.y + self.height - 1))?;
        stdout.queue(Print(self.bottom_right))?;

        (1..self.height - 1).for_each(|i| {
            let _ = stdout.queue(MoveTo(self.x, self.y + i));
            let _ = stdout.queue(Print(self.left));
            let _ = stdout.queue(MoveTo(self.x + self.width - 1, self.y + i));
            let _ = stdout.queue(Print(self.right));
        });


        Ok(())
    }
}

pub fn draw_something(stdout: &mut Stdout) -> Result<()> {
    let (width, height) = size()?;
    let (width, height) = (5, 3);
    let border = Border::new(0, 0, width, height, "raxyzli");
    border.draw(stdout)?;
    stdout.flush()?;
    
    Ok(())
}
