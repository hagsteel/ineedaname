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
mod viewport;

use draw::{Widget, Context};
use events::{Event, Key};
pub use rect::{Pos, Size, Rect};

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
    let mut viewport = viewport::Viewport::new(5, 5);
    viewport.move_by(Pos::new(1, 1));

    let offset = Pos::new(0, 3);
    map.draw(&mut context, &mut stdout, &viewport, offset)?;

    // let mut viewport_border = draw::Border::new(offset, viewport.width, viewport.height, "********");
    // viewport_border.draw(&mut stdout)?;

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
