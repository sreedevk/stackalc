use crate::cli;
use crate::calc;

#[derive(PartialEq)]
pub enum AppState {
    Running,
    Halted
}

pub struct App {
    app_cli: cli::Cli,
    app_calc: calc::Calc,
    app_state: AppState
}

impl App {
    pub fn init() -> App {
        App {
            app_cli: cli::Cli::init().unwrap(),
            app_calc: calc::Calc::init(),
            app_state: AppState::Running
        }
    }

    pub fn run(&mut self) {
        while self.app_state == AppState::Running {
            self.app_cli.render(&mut self.app_calc);
        }
    }
}
