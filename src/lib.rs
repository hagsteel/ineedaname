use std::io::{self, Write, Stdout};

#[cfg(target_os = "windows")]
use crossterm::event::EnableMouseCapture;

#[cfg(not(target_os = "windows"))]
use crossterm::event::DisableMouseCapture;

use crossterm::cursor;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, ExecutableCommand, Result};

mod draw;
mod events;
mod map;
mod rect;

use draw::{Widget, Context};
use events::{Event, Key};
use rect::{Pos, Rect};

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

    // TODO: this just draws status thing for now
    draw::draw_something(&mut stdout)?;

    let map = map::Map::from_path("maps/start.map").unwrap();
    let mut context = Context::with_capacity(80 * 20);
    let mut viewport = map::Viewport::new(15, 7);
    if let Some(s) = map.spawn_point() {
        viewport.set_pos(s);
    }
    map.draw(&mut context, &mut stdout, 0, 3, &viewport)?;
    let mut viewport_border = draw::Border::new(viewport.min_pos(), viewport.width, viewport.height, "********");
    viewport_border.expand(2);
    viewport_border.draw(&mut stdout)?;

    stdout.flush()?;

    for event in events::events() {
        if let Event::Key(Key::Char('q')) = event {
            break;
        }
    }

    disable_raw_mode()
}

// pub fn fun() -> Result<()> {
//     let map = map::Map::from_path("maps/start.map").unwrap();
//     Ok(())
// }
