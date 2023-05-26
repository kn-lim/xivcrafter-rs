use crate::crafter;
use crate::ui::ui;
use crate::utils::{self, Config};

use crossterm::event::{self, Event, KeyCode};

use std::{
    fs, io,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    time::{Duration, Instant},
};

use tui::{backend::Backend, Terminal};

// Constants
/// XIVCrafter tick rate
const TICK_RATE: Duration = Duration::from_millis(500);

#[derive(Clone)]
pub struct App<'a> {
    pub tabs: Vec<&'a str>,
    pub index: usize,

    //// XIVCrafter
    // Settings
    pub config: PathBuf,
    pub name: String,
    pub last_used: i32,

    // Program Settings
    pub current_amount: i32,
    pub max_amount: i32,
    pub start_pause: String,
    pub stop: String,

    // Consumables
    pub food: String,
    pub food_count: i32,
    pub food_duration: i64,
    pub potion: String,
    pub potion_count: i32,

    // In-Game Hotkeys
    pub confirm: String,
    pub cancel: String,
    pub macro1: String,
    pub macro1_duration: u64,
    pub macro2: String,
    pub macro2_duration: u64,
    pub macro3: String,
    pub macro3_duration: u64,
}

impl<'a> App<'a> {
    pub fn new(path: PathBuf) -> App<'a> {
        let file = fs::read_to_string(&path).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("Unable to parse JSON");
        let configs: Vec<Config> = serde_json::from_value(json).unwrap();

        App {
            // TUI
            tabs: vec!["Home", "Config"],
            index: 0,

            // Settings
            config: path,
            name: configs[0].name.clone(),
            last_used: configs[0].id,
            current_amount: 0,
            max_amount: configs[0].amount,

            // Program Hotkeys
            start_pause: configs[0].start_pause.clone(),
            stop: configs[0].stop.clone(),

            // Consumables
            food: configs[0].food.clone(),
            food_count: 0,
            food_duration: configs[0].food_duration,
            potion: configs[0].potion.clone(),
            potion_count: 0,

            // In-Game Hotkeys
            confirm: configs[0].confirm.clone(),
            cancel: configs[0].cancel.clone(),
            macro1: configs[0].macro1.clone(),
            macro1_duration: configs[0].macro1_duration,
            macro2: configs[0].macro2.clone(),
            macro2_duration: configs[0].macro2_duration,
            macro3: configs[0].macro3.clone(),
            macro3_duration: configs[0].macro3_duration,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tabs.len() - 1;
        }
    }

    /// update changes app's values to match the config file
    pub fn update(&mut self) {
        let file = fs::read_to_string(self.config.clone()).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("Unable to parse JSON");
        let configs: Vec<Config> = serde_json::from_value(json).unwrap();

        let mut index = None;
        for (i, config) in configs.iter().enumerate() {
            if config.last_used {
                index = Some(i);
                break;
            }
        }

        match index {
            Some(index) => {
                // Settings
                self.name = configs[index].name.clone();
                self.last_used = configs[index].id;
                self.max_amount = configs[index].amount;

                // Program Hotkeys
                self.start_pause = configs[index].start_pause.clone();
                self.stop = configs[index].stop.clone();

                // Consumables
                self.food = configs[index].food.clone();
                self.food_duration = configs[index].food_duration;
                self.potion = configs[index].potion.clone();

                // In-Game Hotkeys
                self.macro1 = configs[index].macro1.clone();
                self.macro1_duration = configs[index].macro1_duration;
                self.macro2 = configs[index].macro2.clone();
                self.macro2_duration = configs[index].macro2_duration;
                self.macro3 = configs[index].macro3.clone();
                self.macro3_duration = configs[index].macro3_duration;
                self.confirm = configs[index].confirm.clone();
                self.cancel = configs[index].cancel.clone();
            }
            None => {
                // TODO: Handle error if last_used doesn't exist in any
            }
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    app.update();

    let mut last_tick = Instant::now();

    let program_signal = Arc::new(AtomicBool::new(false));
    let crafter_signal = Arc::new(AtomicBool::new(false));

    let receiver = crafter::craft(&app, program_signal.clone(), crafter_signal.clone());

    let mut message = String::from("Waiting...");

    loop {
        terminal.draw(|f| ui(f, &app, &message, &program_signal, &crafter_signal))?;

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        let start_pause_hotkey = utils::get_crossterm_key_code(&app.start_pause).unwrap();
        let stop_hotkey = utils::get_crossterm_key_code(&app.stop).unwrap();

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    program_signal.store(false, Ordering::Relaxed);
                    crafter_signal.store(false, Ordering::Relaxed);
                    return Ok(());
                } else if key.code == KeyCode::Right {
                    app.next();
                } else if key.code == KeyCode::Left {
                    app.previous();
                } else if key.code == start_pause_hotkey {
                    program_signal.store(true, Ordering::Relaxed);

                    if crafter_signal.load(Ordering::Relaxed) {
                        crafter_signal.store(false, Ordering::Relaxed);
                    } else {
                        crafter_signal.store(true, Ordering::Relaxed);
                    }
                } else if key.code == stop_hotkey {
                    program_signal.store(false, Ordering::Relaxed);
                    crafter_signal.store(false, Ordering::Relaxed);
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            app.update();

            match receiver.recv_timeout(Duration::from_millis(100)) {
                Ok(val) => {
                    let (crafted, food, potion, msg) = val;
                    app.current_amount = crafted;
                    app.food_count = food;
                    app.potion_count = potion;
                    message = msg;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }

            last_tick = Instant::now();
        }
    }
}
