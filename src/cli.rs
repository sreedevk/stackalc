use crate::calc::Calc;
use std::io;
use tui::{
    Terminal,
    backend::TermionBackend,
};

use termion::raw::{IntoRawMode, RawTerminal};

pub struct Cli {
    term: Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
}

impl Cli {
    pub fn init() -> Result<Cli, io::Error> {
        let stdout       = io::stdout().into_raw_mode()?;
        let backend      = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        Ok(Cli { term: terminal })
    }

    pub fn render_stack(&mut self, calc: &Calc) {
        for frame in calc.frames.to_owned() {
            println!("frame: {}", frame);
        }
    }
}
