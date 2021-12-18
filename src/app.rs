use crate::cli;
use crate::calc;
use crate::events;

#[derive(PartialEq)]
pub enum AppState {
    Running,
    Halted
}

pub struct App {
    pub app_cli: cli::Cli,
    pub app_calc: calc::Calc,
    pub app_state: AppState,
    pub input_buffer: Vec<String>
}

impl App {
    pub fn init() -> App {
        App {
            app_cli: cli::Cli::init().unwrap(),
            app_calc: calc::Calc::init(),
            app_state: AppState::Running,
            input_buffer: Vec::with_capacity(64)
        }
    }

    pub fn parse_input(&mut self) {
        let input = self.input_buffer.join("");
        let parsed_input: f64 = input.as_str().parse().unwrap();
        self.app_calc.push(parsed_input);
        self.input_buffer = Vec::with_capacity(64);
    }

    pub fn run(&mut self) {
        let mut events_manager = events::EventsManager::init();
        while self.app_state == AppState::Running {
            events_manager.handle_events(self).unwrap();
            self.app_cli.render(&mut self.app_calc);
        }
    }
}
