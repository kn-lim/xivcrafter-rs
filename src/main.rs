use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use serde::{Deserialize, Serialize};

use std::{
    error::Error,
    io,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod crafter;
mod ui;

// Constants
const TICK_RATE: Duration = Duration::from_millis(250);

/// Config file
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    id: i32,
    last_used: bool,
    name: String,
    amount: i32,
    food: String,
    food_duration: i64,
    potion: String,
    macro1: String,
    macro1_duration: i32,
    macro2: String,
    macro2_duration: i32,
    macro3: String,
    macro3_duration: i32,
    start_pause: String,
    stop: String,
    confirm: String,
    cancel: String,
}

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
    let mut app = crafter::app::App::new(path);
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: crafter::app::App) -> io::Result<()> {
    // Setup XIVCrafter
    // TODO

    let mut last_tick = Instant::now();

    let message = Arc::new(Mutex::new(String::from("Starting...")));
    let _message_thread = crafter::app::run(&app, message.clone());

    loop {
        terminal.draw(|f| ui::ui::ui(f, &app, &message))?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Right => app.next(),
                    KeyCode::Left => app.previous(),
                    KeyCode::Char('r') => {
                        app.program_running = true;

                        if app.running {
                            app.running = false;
                        } else {
                            app.running = true;
                        }
                    }
                    KeyCode::Char('o') => {
                        app.program_running = false;
                        app.running = false;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            app.update();

            if app.program_running {
                if app.running {
                    if app.current_amount < app.max_amount {
                        app.increment_amount();
                    } else {
                        // let mut enigo = Enigo::new();
                        // enigo.key_click(Key::Layout('v'));
                        app.current_amount = 0;
                    }
                }
            }

            last_tick = Instant::now();
        }
    }
}
