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

use std::{
    error::Error,
    fs::File,
    io::{stdout, Write},
};

use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    // get home directory path
    let home_dir = dirs::home_dir();
    if home_dir.is_none() {
        panic!("ERROR cannot locate home directory")
    }

    // get .xivcrafter.json file path
    let path = home_dir.as_ref().unwrap().join(".xivcrafter.json");

    // check if .xivcrafter.json already exists
    if !path.exists() {
        let config = [utils::Config {
            id: 0,
            last_used: true,
            name: String::from(""),
            amount: 0,
            food: String::from(""),
            food_duration: 0,
            potion: String::from(""),
            macro1: String::from(""),
            macro1_duration: 0,
            macro2: String::from(""),
            macro2_duration: 0,
            macro3: String::from(""),
            macro3_duration: 0,
            start_pause: String::from(""),
            stop: String::from(""),
            confirm: String::from(""),
            cancel: String::from(""),
        }];

        let json = serde_json::to_string_pretty(&config)?;
        let mut file = File::create(&path)?;
        file.write_all(json.as_bytes())?;
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, App::init(path));

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
