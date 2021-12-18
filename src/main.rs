mod calc;
mod cli;
mod app;
mod events;

use calc::Calc;
use calc::OptIdent;
use cli::Cli;
use app::App;

fn main() -> Result<(), std::io::Error> {
    let mut app = App::init();
    app.run();
    Ok(())
}
