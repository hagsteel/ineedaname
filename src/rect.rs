use crossterm::cursor::MoveTo;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub origin: Pos,
    pub size: Size,
}

impl Rect {
    pub fn new(origin: Pos, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn contains(&self, pos: Pos) -> bool {
        self.origin.x <= pos.x
            && pos.x < self.origin.x + self.size.width
            && self.origin.y <= pos.y
            && pos.y < self.origin.y + self.size.height
    }
}

// -----------------------------------------------------------------------------
//     - Position -
// -----------------------------------------------------------------------------
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
        Self { x, y }
    }

    pub fn add(&self, add: Pos) -> Pos {
        Pos::new(self.x + add.x, self.y + add.y)
    }

    pub fn sub(&self, add: Pos) -> Pos {
        Pos::new(self.x - add.x, self.y - add.y)
    }

    pub fn into_move_to(self: Self) -> MoveTo {
        MoveTo(self.x, self.y)
    }

    pub fn to_tuple(self) -> (u16, u16) {
        (self.x, self.y)
    }
}

impl From<(usize, usize)> for Pos {
    fn from(p: (usize, usize)) -> Pos {
        Pos::new(p.0 as u16, p.1 as u16)
    }
}

// -----------------------------------------------------------------------------
//     - Size -
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn to_tuple(self) -> (u16, u16) {
        (self.width, self.height)
    }
}
