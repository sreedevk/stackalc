use std::{
    io,
    sync::mpsc,
    thread,
    time::Duration
};

use termion::{
    event::Key,
    input::TermRead
};

pub enum Event<T> {
    Input(T),
    Tick
}

pub struct EventsManager {
    rx: mpsc::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>
}

impl EventsManager {
    pub fn init() -> EventsManager {
        let (tx, rx) = mpsc::channel();

        // Input Events
        let input_tx = tx.clone();
        let input_handle = {
            thread::spawn(move || {
                let stdin = std::io::stdin();
                for event in stdin.keys() {
                    if let Ok(key) = event {
                        if let Err(err) = input_tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                }
            })
        };

        // Tick Events
        let tick_handle = {
            thread::spawn(move || {
                if let Err(err) = tx.send(Event::Tick) {
                    eprintln!("{}", err);
                    return;
                }
                thread::sleep(Duration::from_millis(250));
            })
        };

        EventsManager {
            rx,
            input_handle,
            tick_handle
        }
    }
}
