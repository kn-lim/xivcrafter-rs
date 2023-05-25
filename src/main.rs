mod app;
mod crafter;
mod ui;
mod utils;

use crate::app::{run_app, App};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{error::Error, io};

use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    // Get home directory path
    let home_dir = dirs::home_dir();
    if home_dir.is_none() {
        panic!("ERROR cannot locate home directory")
    }

    // Get .xivcrafter.json file path
    let path = home_dir.as_ref().unwrap().join(".xivcrafter.json");

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, App::new(path));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
