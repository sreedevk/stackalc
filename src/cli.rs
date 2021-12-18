use crate::calc::Calc;
use std::io;
use tui::{
    Terminal,
    layout::{Layout, Direction, Rect, Constraint, Corner},
    backend::TermionBackend,
    widgets::{Block, Widget, Borders, List, ListItem},
    style::{Style, Color, Modifier}
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
        terminal.clear().unwrap();
        Ok(Cli { term: terminal })
    }

    pub fn render(&mut self, calcos: &Calc) {
        self.term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(1)
                .horizontal_margin(1)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(f.size());
            let block = Block::default()
                .title("Stackalc")
                .borders(Borders::all());

            let stackframes: Vec<ListItem> = calcos
                .frames
                .iter()
                .map(|x| ListItem::new(String::from(format!("{}", x))) )
                .collect();

            let calc_stack = List::new(stackframes)
                .block(Block::default().title("Stack").borders(Borders::all()))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            let operations = vec![
                ListItem::new("Add <+>"),
                ListItem::new("Subtract <->"),
                ListItem::new("Multiply <*>"),
                ListItem::new("Divide </>")
            ];

            let operations_list = List::new(operations)
                .block(Block::default().title("Operations").borders(Borders::all()))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            f.render_widget(block, f.size());
            f.render_widget(calc_stack, chunks[1]);
            f.render_widget(operations_list, chunks[0]);
        }).unwrap();
    }
}
