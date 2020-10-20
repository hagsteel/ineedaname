use std::fs::read_to_string;
use std::io::{self, Stdout};
use std::path::PathBuf;

use crossterm::Result;

use crate::draw::Context;
use crate::draw::Widget;
use crate::Pos;
use crate::viewport::Viewport;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Flag {
    Spawn,
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
        viewport: &Viewport,
        offset: Pos,
    ) -> Result<()> {
        let rect = viewport.rect();

        let mut count = 0;
        let max = viewport.size.width * viewport.size.height;

        for tile in &self.tiles {
            if rect.contains_with_offset(tile.pos, viewport.render_offset) {
                context.put(
                    tile.c,
                    Pos::new(tile.pos.x + offset.x, tile.pos.y + offset.y),
                    None,
                    None,
                );

                count += 1;
                if count > max {
                    return Ok(());
                }
            }
        }

        context.draw(stdout)?;
        Ok(())
    }
}
