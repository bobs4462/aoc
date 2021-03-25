use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::thread::JoinHandle;
use std::{io, time::Duration};
use termion::input::TermRead;

use termion::event::Key;

pub enum Event {
    KeyPress(Key),
    Tick,
}

pub struct EventProvider {
    pub rx: Receiver<Event>,
    pub tick_thread_handle: JoinHandle<()>,
    pub key_thread_handle: JoinHandle<()>,
}

impl EventProvider {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let key_thread_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::KeyPress(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                }
            })
        };
        let tick_rate = Duration::from_millis(50);
        let tick_thread_handle = {
            thread::spawn(move || loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(tick_rate);
            })
        };
        EventProvider {
            rx,
            key_thread_handle,
            tick_thread_handle,
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
