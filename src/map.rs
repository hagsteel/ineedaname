use std::fs::read_to_string;
use std::io::{self, Stdout};
use std::path::Path;

use crossterm::Result;
use tilemap::TileMap;

use crate::draw::{Context, Widget};
use crate::{Pos, Rect};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Flag {
    Spawn,
}

#[derive(Debug, Clone)]
pub struct Tile {
    c: char,
    flags: Vec<Flag>,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        let flags = if c == 'S' { vec![Flag::Spawn] } else { vec![] };
        Tile { c, flags }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            c: ' ',
            flags: Vec::new(),
        }
    }
}

pub struct Map {
    tilemap: TileMap<Tile>,
    pub context: Context,
    pub visible_area: Rect,
    pub render_offset: Pos,
}

impl Map {
    pub fn from_path(
        context: Context,
        visible_area: Rect,
        render_offset: Pos,
        path: impl AsRef<Path>,
    ) -> io::Result<Self> {
        let raw_data = read_to_string(path)?;

        let mut width = 0;
        let mut height = 0;

        let mut chars = Vec::new();

        for (y, line) in raw_data.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if x > width {
                    width = x;
                }
                if y > height {
                    height = y;
                }
                chars.push((c, (x, y)));
            }
        }

        let mut tilemap = TileMap::new(width, height);

        for (c, pos) in chars {
            tilemap.insert(
                c.into(),
                pos,
            );
        }

        Ok(Self {
            tilemap,
            context,
            visible_area,
            render_offset,
        })
    }

    pub fn spawn_point(&self) -> Option<Pos> {
        self.tilemap
            .tiles
            .iter()
            .enumerate()
            .find_map(|(index, tile)| match tile {
                Some(t) if t.flags.iter().any(|&f| f == Flag::Spawn) => {
                    Some(self.tilemap.from_index(index).into())
                }
                _ => None,
            })
    }

    pub fn translate(&self, pos: Pos) -> Pos {
        pos.sub(self.visible_area.origin).add(self.render_offset)
    }

    pub fn centre(&self, pos: Pos) -> Pos {
        let centre = Pos::new(
            self.visible_area.size.width / 2,
            self.visible_area.size.height / 2,
        );
        pos.sub(centre)
    }
}

impl Widget for Map {
    fn draw(&mut self, stdout: &mut Stdout) -> Result<()> {
        let mut count = 0;
        let max = self.visible_area.size.width * self.visible_area.size.height;

        let start = self.visible_area.origin.to_tuple();
        let end = {
            let (mut x, mut y) = self.visible_area.size.to_tuple();
            x += start.0;
            y += start.1;
            (x as usize, y as usize)
        };

        let start = (start.0 as usize, start.1 as usize);

        self.tilemap.coords_in_area(start, end).for_each(|(x, y)| {
            let tile = self.tilemap.by_coords((x, y));
            if let Some(tile) = tile {
                self.context.put(
                    tile.c,
                    Pos::new(
                        x as u16 + self.render_offset.x,
                        y as u16 + self.render_offset.y,
                    ),
                    None,
                    None,
                );
            }
        });

        Ok(())
    }
}
