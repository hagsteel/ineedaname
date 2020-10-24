#![warn(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{self, Stdout, Write};

#[cfg(target_os = "windows")]
use crossterm::event::EnableMouseCapture;

#[cfg(not(target_os = "windows"))]
use crossterm::event::DisableMouseCapture;

use crossterm::cursor;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, ExecutableCommand, Result};
use crossterm::style::Color;

mod draw;
mod events;
mod map;
mod rect;
mod debug;

pub use draw::{Context, Widget, Character};
use events::{Event, Key};
pub use rect::{Pos, Rect, Size};
pub use debug::DebugLines;

fn do_the_raw_mode_thing() -> Result<Stdout> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    #[cfg(target_os = "windows")]
    execute!(stdout, EnableMouseCapture)?;

    #[cfg(not(target_os = "windows"))]
    execute!(stdout, DisableMouseCapture,)?;

    stdout.execute(cursor::Hide)?;
    stdout.execute(Clear(ClearType::All))?;
    Ok(stdout)
}

pub fn run() -> Result<()> {
    let mut stdout = do_the_raw_mode_thing()?;

    draw::draw_status(&mut stdout)?;

    // Debug jazz
    let mut debug = DebugLines::new(30);

    let mut context = Context::with_capacity(11 * 7);
    let render_offset = Pos::new(1, 4);
    let visible_area = Rect::new(Pos::zero(), Size::new(15, 7));
    let mut map = map::Map::from_path(context, visible_area, render_offset, "maps/start.map").unwrap();

    let mut map_border = draw::Border::new(Pos::new(0, 3), 17, 9, "╭─╮│╯─╰│");
    map_border.draw(&mut stdout)?;

    let mut player_pos = map.spawn_point().unwrap();
    let mut player = Character::new('', player_pos);
    player.fg = Some(Color::Green);

    for event in events::events() {
        if let Event::Key(Key::Char('q')) = event {
            break;
        }

        match event {
            Event::Left if player_pos.x > 0 => player_pos.x -= 1,
            Event::Right => player_pos.x += 1,
            Event::Up if player_pos.y > 0 => player_pos.y -= 1,
            Event::Down => player_pos.y += 1,
            Event::Tick(frame) => { }
            _ => {}
        }

        debug.draw(&mut stdout);

        map.visible_area.origin = player_pos;
        map.draw(&mut stdout)?;
        // debug.push(format!("{:?}", map.context.chars.len()));
        map.context.draw(&mut stdout)?;

        // player.set_pos(map.translate(player_pos));
        // player.draw(&mut stdout)?;
        stdout.flush()?;
    }

    disable_raw_mode()
}
