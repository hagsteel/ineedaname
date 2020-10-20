#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub top_left: Pos,
    pub bottom_right: Pos,
}

impl Rect {
    pub fn new(top_left: Pos, bottom_right: Pos) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn contains(&self, pos: Pos) -> bool {
        if pos.x < self.top_left.x {
            false
        } else if pos.x > self.bottom_right.x {
            false
        } else if pos.y < self.top_left.y {
            false
        } else if pos.y > self.bottom_right.y {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y
        }
    }
}
