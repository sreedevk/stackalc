mod calc;
mod cli;

use calc::Calc;
use calc::OptIdent;
use cli::Cli;

fn main() -> Result<(), std::io::Error> {
    let mut c = Calc::init();
    let mut cli = Cli::init()?;
    c.push(10.0);
    c.push(20.0);
    c.reduce(OptIdent::Add);
    cli.render_stack(&c);
    Ok(())
}
