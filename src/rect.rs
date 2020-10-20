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

    pub fn contains_with_offset(&self, pos: Pos, offset: Pos) -> bool {
        if pos.x as i32 - (offset.x as i32) < 0 {
            return false;
        }

        if pos.y as i32 - (offset.y as i32) < 0 {
            return false;
        }

        let pos = pos.sub(offset);
        self.origin.x <= pos.x
            && pos.x < self.origin.x + self.size.width
            && self.origin.y <= pos.y
            && pos.y < self.origin.y + self.size.height
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
        Self { x, y }
    }

    pub fn sub(&self, add: Pos) -> Pos {
        Pos::new(self.x - add.x, self.y - add.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}
