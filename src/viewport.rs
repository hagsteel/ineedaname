use crate::{Rect, Pos, Size};

pub struct Viewport {
    pub size: Size,
    pub render_offset: Pos,
}

impl Viewport {
    pub fn rect(&self) -> Rect {
        Rect::new(Pos::zero(), self.size)
    }

    pub fn new(width: u16, height: u16) -> Self {
        Self {
            size: Size::new(width, height),
            render_offset: Pos::zero(),
        }
    }

    pub fn move_by(&mut self, pos: Pos) {
        self.render_offset = pos;
    }
}


