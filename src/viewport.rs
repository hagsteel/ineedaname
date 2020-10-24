use std::io::Stdout;

use crossterm::Result;

use crate::{Rect, Pos, Size};
use crate::Widget;
use crate::map::Map;

// Viewport
// --------
// Have a fixed location for rendering the viewport (e.g 0, 3)
// Offset the map inside the viewport to draw the correct region.

pub struct Viewport {
    size: Size,
    pos: Pos,
    map: Map,
}

impl Viewport {
    pub fn rect(&self) -> Rect {
        Rect::new(Pos::zero(), self.size)
    }

    pub fn new(pos: Pos, width: u16, height: u16, map: Map) -> Self {
        Self {
            size: Size::new(width, height),
            pos,
            map,
        }
    }

    pub fn move_to(&mut self, pos: Pos) {
        self.map.centre = pos;
    }
}

impl Widget for Viewport {
    fn draw(&mut self, stdout: &mut Stdout) -> Result<()> {
        self.map.draw(stdout)?;
        Ok(())
    }
}
