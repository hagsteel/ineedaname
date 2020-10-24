use std::io::Stdout;

use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{QueueableCommand, Result};
use crossterm::cursor::MoveTo;

use super::Widget;
use crate::Pos;

#[derive(Debug)]
pub struct Context {
    pub chars: Vec<(char, Pos, Option<Color>, Option<Color>)>,
}

impl Context {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            chars: Vec::with_capacity(cap),
        }
    }

    pub fn put(&mut self, c: char, pos: Pos, fg: Option<Color>, bg: Option<Color>) {
        let (fg, bg) = match self.chars.last() {
            Some((_, _, prev_fg, prev_bg)) => {
                let fg = match (prev_fg, fg) {
                    (Some(prev_fg), Some(fg)) if *prev_fg == fg => None,
                    (Some(prev_fg), None) => Some(*prev_fg),
                    (None, Some(fg)) => Some(fg),
                    _ => None,
                };

                let bg = match (prev_bg, bg) {
                    (Some(prev_bg), Some(bg)) if *prev_bg == bg => None,
                    (Some(prev_bg), None) => Some(*prev_bg),
                    (None, Some(bg)) => Some(bg),
                    _ => None,
                };

                (fg, bg)
            }
            None => (fg, bg),
        };

        self.chars.push((c, pos, fg, bg));
    }
}

impl Widget for Context {
    fn draw(&mut self, stdout: &mut Stdout) -> Result<()> {
        self.chars.drain(..).for_each(|(c, pos, fg, bg)| {
            stdout.queue(MoveTo(pos.x, pos.y)).expect("Failed to position cursor");

            if let Some(col) = fg {
                stdout.queue(SetForegroundColor(col)).expect("Failed to set foreground colour");
            }

            if let Some(col) = bg {
                stdout.queue(SetBackgroundColor(col)).expect("Failed to set foreground colour");
            }

            stdout.queue(Print(c)).expect("Failed to print character");
        });

        stdout.queue(ResetColor)?;
        Ok(())
    }
}
