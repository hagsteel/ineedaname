use std::fs::read_to_string;
use std::io::{self, Stdout};
use std::path::PathBuf;

use crossterm::Result;

use crate::draw::Context;
use crate::draw::Widget;
use crate::{Rect, Pos};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Flag {
    Spawn,
}

pub struct Viewport {
    pub pos: Pos,
    pub width: u16,
    pub height: u16,
}

impl Viewport {
    fn rect(&self) -> Rect {
        Rect::new(self.pos, Pos::new(self.pos.x + self.width, self.pos.y + self.width))
    }

    pub fn new(width: u16, height: u16) -> Self {
        Self {
            pos: Pos::zero(),
            width,
            height,
        }
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }

    pub fn min_pos(&self) -> Pos {
        let min_x = self.pos.x - self.width / 2;
        let min_y = self.pos.y - self.height / 2;
        Pos::new(min_x, min_y)
    }
}


pub struct Tile {
    pos: Pos,
    c: char,
    flags: Vec<Flag>,
}

pub struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub fn from_path(path: impl Into<PathBuf>) -> io::Result<Self> {
        let raw_data = read_to_string(path.into())?;

        let tiles = raw_data
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().map(|(x, c)| {
                    let mut flags = Vec::new();

                    if c == 'S' {
                        flags.push(Flag::Spawn);
                    }

                    Tile {
                        pos: Pos::new(x as u16, y as u16),
                        c,
                        flags,
                    }
                }).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect();

        Ok(Self { tiles })
    }

    pub fn spawn_point(&self) -> Option<Pos> {
        self.tiles.iter().find_map(|tile| {
            if tile.flags.iter().any(|&f| f == Flag::Spawn) {
                Some(tile.pos)
            } else {
                None
            }
        })
    }

    pub fn draw(
        &self,
        context: &mut Context,
        stdout: &mut Stdout,
        offset_x: u16,
        offset_y: u16,
        viewport: &Viewport,
    ) -> Result<()> {

        // Centre of viewport has X, Y
        // only draw tiles that are less than viewport.width / 2 + X and greater
        //                               than viewport.width / 2 - X

        let rect = viewport.rect();

        for tile in &self.tiles {
            if rect.contains(tile.pos) {
                context.put(
                    tile.c,
                    Pos::new(tile.pos.x + offset_x, tile.pos.y + offset_y),
                    None,
                    None,
                );
            }
        }

        context.draw(stdout)?;
        Ok(())
    }
}
