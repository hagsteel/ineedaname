use std::io::{self, Write};

#[cfg(target_os = "windows")]
use crossterm::event::EnableMouseCapture;

#[cfg(not(target_os = "windows"))]
use crossterm::event::DisableMouseCapture;

use crossterm::cursor;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, Result};

mod events;
mod draw;

use events::{Event, Key};


pub fn run() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    #[cfg(target_os = "windows")]
    execute!(stdout, EnableMouseCapture)?;

    #[cfg(not(target_os = "windows"))]
    execute!(
        stdout,
        DisableMouseCapture,
        Clear(ClearType::All),
        cursor::Hide
    )?;

    draw::draw_something(&mut stdout)?;

    for event in events::events() {
        if let Event::Key(Key::Char('q')) = event {
            break;
        }
    }

    disable_raw_mode()
}
