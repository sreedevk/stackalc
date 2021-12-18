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

use crate::app::{App, AppState};
use crate::calc::{OptIdent};

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

    fn dequeue(&mut self) -> Result<Event<Key>, mpsc::TryRecvError> {
        self.rx.try_recv()
    }

    pub fn handle_events(&mut self, app: &mut App) -> Result<(), mpsc::TryRecvError> {
        let event = self.dequeue();
        match event {
            Ok(Event::Input(key)) => {
                match key {
                    Key::Char('+') => { app.app_calc.reduce(OptIdent::Add); Ok(()) },
                    Key::Char('-') => { app.app_calc.reduce(OptIdent::Sub); Ok(()) },
                    Key::Char('/') => { app.app_calc.reduce(OptIdent::Div); Ok(()) },
                    Key::Char('*') => { app.app_calc.reduce(OptIdent::Mul); Ok(()) },
                    Key::Char('q') => { app.app_state = AppState::Halted; Ok(()) },
                    Key::Char('0') => { app.input_buffer.push(String::from("0")); Ok(()) },
                    Key::Char('1') => { app.input_buffer.push(String::from("1")); Ok(()) },
                    Key::Char('2') => { app.input_buffer.push(String::from("2")); Ok(()) },
                    Key::Char('3') => { app.input_buffer.push(String::from("3")); Ok(()) },
                    Key::Char('4') => { app.input_buffer.push(String::from("4")); Ok(()) },
                    Key::Char('5') => { app.input_buffer.push(String::from("5")); Ok(()) },
                    Key::Char('6') => { app.input_buffer.push(String::from("6")); Ok(()) },
                    Key::Char('7') => { app.input_buffer.push(String::from("7")); Ok(()) },
                    Key::Char('8') => { app.input_buffer.push(String::from("8")); Ok(()) },
                    Key::Char('9') => { app.input_buffer.push(String::from("9")); Ok(()) },
                    Key::Char('\n') => { app.parse_input(); Ok(()) },
                    _ => Ok(())
                }
            },
            Ok(Event::Tick) => { Ok(()) },
            Err(_) => Ok(())
        }
    }
}
