use crate::calc::Calc;
use std::io;
use tui::{
    Terminal,
    layout::{Layout, Direction, Rect, Constraint, Corner},
    backend::TermionBackend,
    widgets::{Block, Widget, Borders}
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

    pub fn render(&mut self, calc: &Calc) {
        self.term.clear().unwrap();
        self.term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(f.size());
            let block = Block::default()
                .title("Stackalc")
                .borders(Borders::all());

            f.render_widget(block, f.size());
        });
    }
}
