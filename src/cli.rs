use crate::app::App;
use std::io;
use tui::{
    Terminal,
    layout::{Layout, Direction, Rect, Constraint, Corner, Alignment},
    backend::TermionBackend,
    text::{Span, Spans},
    widgets::{Block, Widget, Borders, List, ListItem, Paragraph, Wrap},
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

    pub fn render(&mut self, app: &App) {
        self.term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(1)
                .horizontal_margin(1)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(f.size());
            let sub_right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(chunks[1]);

            let block = Block::default()
                .title("Stackalc")
                .borders(Borders::all());

            let stackframes: Vec<ListItem> = app
                .app_calc
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

            let formatted_buffer = Spans::from(Span::styled(app.formatted_input_buffer(), Style::default().fg(Color::Blue)));
            let input_buffer_span = Paragraph::new(vec![formatted_buffer])
                .block(Block::default().title("Input").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });

            f.render_widget(block, f.size());
            f.render_widget(calc_stack, sub_right_chunks[0]);
            f.render_widget(input_buffer_span, sub_right_chunks[1]);
            f.render_widget(operations_list, chunks[0]);
        }).unwrap();
    }
}
