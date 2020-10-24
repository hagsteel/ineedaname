use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Receiver, Sender};

use crossterm::event::{read, KeyEvent, KeyCode, Event as CrossTermEvent};

pub type Tx = Sender<Event>;
pub type Rx = Receiver<Event>;

pub enum Event {
    Key(Key),
    // Resize(u16, u16),
    Tick(usize),
    Left,
    Right, 
    Up,
    Down,
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

    tick(tx.clone());

    thread::spawn(move || {
        loop {
            if let Ok(event) = read() {
                if let CrossTermEvent::Key(KeyEvent { code, .. }) = event {
                    match code {
                        KeyCode::Char(c) => { let _ = tx.send(Event::Key(Key::Char(c))); }
                        KeyCode::Left => { let _ = tx.send(Event::Left); }
                        KeyCode::Right => { let _ = tx.send(Event::Right); }
                        KeyCode::Up => { let _ = tx.send(Event::Up); }
                        KeyCode::Down => { let _ = tx.send(Event::Down); }
                        _ => {
                        }
                    }

                }
            }
        }
    });

    Events { rx }
}


fn tick(tx: Tx) {
    thread::spawn(move || {
        let mut frame = 0;
        loop {
            let _ = tx.send(Event::Tick(frame));
            thread::sleep(Duration::from_millis(33));
            frame.wrapping_add(1);
        }
    });
}
