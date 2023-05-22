use enigo::{Enigo, Key, KeyboardControllable};

use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// delay per key press (milliseconds)
const KEY_DELAY: u64 = 500;

/// delay per action (seconds)
const ACTION_DELAY: u64 = 1;

/// potion duration (seconds)
const POTION_DURATION: i64 = 900;

pub struct App<'a> {
    pub tabs: Vec<&'a str>,
    pub index: usize,

    //// XIVCrafter
    // Settings
    pub config: PathBuf,
    pub last_used: i32,
    pub name: String,
    pub program_running: bool,
    pub running: bool,
    pub current_amount: i32,
    pub max_amount: i32,

    // Program Hotkeys
    pub start_pause: String,
    pub stop: String,

    // Food
    pub food: String,
    pub food_count: i32,
    pub food_duration: i64,
    pub food_start_time: i64,

    // Potion
    pub potion: String,
    pub potion_count: i32,
    pub potion_start_time: i64,

    // In-Game Hotkeys
    pub confirm: String,
    pub cancel: String,
    pub macro1: String,
    pub macro1_duration: i32,
    pub macro2: String,
    pub macro2_duration: i32,
    pub macro3: String,
    pub macro3_duration: i32,
}

impl<'a> App<'a> {
    pub fn new(path: PathBuf) -> App<'a> {
        let file = fs::read_to_string(&path).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("Unable to parse JSON");

        let configs: Vec<crate::Config> = serde_json::from_value(json).unwrap();

        App {
            // TUI
            tabs: vec!["Home", "Config"],
            index: 0,

            // Setup XIVCrafter
            config: path,
            last_used: configs[0].id,
            name: configs[0].name.clone(),
            program_running: false,
            running: false,
            current_amount: 0,
            max_amount: configs[0].amount,
            start_pause: configs[0].start_pause.clone(),
            stop: configs[0].stop.clone(),
            food: configs[0].food.clone(),
            food_count: 0,
            food_duration: configs[0].food_duration,
            food_start_time: 0,
            potion: configs[0].potion.clone(),
            potion_count: 0,
            potion_start_time: 0,
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

        let configs: Vec<crate::Config> = serde_json::from_value(json).unwrap();

        for config in configs.iter() {
            if config.last_used {
                self.last_used = config.id;
                self.name = config.name.clone();
                self.max_amount = config.amount;
                self.food = config.food.clone();
                self.food_duration = config.food_duration;
                self.potion = config.potion.clone();
                self.macro1 = config.macro1.clone();
                self.macro1_duration = config.macro1_duration;
                self.macro2 = config.macro2.clone();
                self.macro2_duration = config.macro2_duration;
                self.macro3 = config.macro3.clone();
                self.macro3_duration = config.macro3_duration;
                self.start_pause = config.start_pause.clone();
                self.stop = config.stop.clone();
                self.confirm = config.confirm.clone();
                self.cancel = config.cancel.clone();
            }
        }
    }

    // crafter related functions
    /// increments the total amount crafted
    pub fn increment_amount(&mut self) {
        self.current_amount += 1;
    }

    /// increments the total amount of food consumed
    fn increment_food(&mut self) {
        self.food_count += 1;
    }

    /// increments the total amount of potions consumed
    fn increment_potion(&mut self) {
        self.potion_count += 1;
    }

    /// start_craft sets up the crafting action
    fn start_craft(&mut self) {
        let mut enigo = Enigo::new();

        let confirm = get_key_code(&self.confirm);

        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
    }

    /// stop_craft closes the crafting action
    fn stop_craft(&mut self) {
        let mut enigo = Enigo::new();

        let confirm = get_key_code(&self.confirm);
        let cancel = get_key_code(&self.cancel);

        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
        enigo.key_click(cancel.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
    }

    /// check_food checks to see whether the food buff needs to be renewed
    fn check_food(&mut self) {
        if self.food_start_time > 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as i64;

            let difference = now - self.food_start_time;

            if difference > self.food_duration {
                self.consume_food();
            }
        } else {
            self.consume_food();
        }
    }

    fn consume_food(&mut self) {}
}

pub fn run(mut app: &App, message: Arc<Mutex<String>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let messages = vec![
            "Processing...",
            "Still going...",
            "Almost there...",
            "Finished!",
        ];

        for msg in messages {
            {
                // Update the message
                let mut message = message.lock().unwrap();
                *message = msg.to_string();
            }

            // Sleep for a while
            thread::sleep(Duration::from_secs(2));
        }
    })
}

pub fn get_key_code(key: &str) -> Option<Key> {
    let key = key.to_lowercase();
    if key.len() == 1 {
        let c = key.chars().next().unwrap();
        return Some(Key::Layout(c));
    }
    match key.as_str() {
        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "pageup" => Some(Key::PageUp),
        "pagedown" => Some(Key::PageDown),
        "delete" => Some(Key::Delete),
        _ => None,
    }
}
