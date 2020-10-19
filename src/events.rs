use std::thread;
use std::sync::mpsc::{self, Receiver};

use crossterm::event::{read, KeyEvent, KeyCode, Event as CrossTermEvent};

pub type Rx = Receiver<Event>;

pub enum Event {
    Key(Key),
    // Resize(u16, u16),
}

pub enum Key {
    Char(char),
}

pub struct Events {
    rx: Rx
}

impl Iterator for Events {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.recv().ok()
    }
}

pub fn events() -> Events {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            if let Ok(event) = read() {
                if let CrossTermEvent::Key(KeyEvent { code, .. }) = event {
                    match code {
                        KeyCode::Char(c) => { let _ = tx.send(Event::Key(Key::Char(c))); }
                        _ => {
                        }
                    }

                }
            }
        }
    });

    Events { rx }
}

