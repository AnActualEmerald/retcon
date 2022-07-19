use std::io::{self, Write};

use anyhow::Result;
use crossterm::{cursor, execute, style::Print, terminal};

mod client;
mod proto;

fn main() {
    let mut stdout = io::stdout();
    if let Err(e) = run(&mut stdout) {
        println!("{}", e);
    }
}

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(
        w,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    execute!(
        w,
        Print(format!("Welcome to RetCon v{}!", env!("CARGO_PKG_VERSION"))),
        cursor::MoveToNextLine(1)
    )?;

    //let the terminal exit the screen rather than return early on error
    let ret = client::start();

    execute!(w, terminal::LeaveAlternateScreen)?;

    ret
}
