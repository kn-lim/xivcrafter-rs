use crate::app::App;
use crate::utils::{self, Config};

use enigo::{Enigo, KeyboardControllable};

use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, mpsc,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// delay per key press (milliseconds)
const KEY_DELAY: u64 = 500;

/// delay per action (seconds)
const ACTION_DELAY: u64 = 2;

/// potion duration (seconds)
const POTION_DURATION: i64 = 900;

#[derive(Clone)]
pub struct Crafter {
    // Crafter Settings
    pub current_amount: i32,
    pub max_amount: i32,

    // Consumables
    pub food: String,
    pub food_count: i32,
    pub food_duration: i64,
    pub food_start_time: i64,
    pub potion: String,
    pub potion_count: i32,
    pub potion_start_time: i64,

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

impl Crafter {
    pub fn new(path: &PathBuf, last_used: i32) -> Crafter {
        let file = fs::read_to_string(path).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("Unable to parse JSON");
        let configs: Vec<Config> = serde_json::from_value(json).unwrap();

        Crafter {
            // Settings
            current_amount: 0,
            max_amount: configs[last_used as usize].amount,

            // Consumables
            food: configs[last_used as usize].food.clone(),
            food_count: 0,
            food_duration: configs[last_used as usize].food_duration,
            food_start_time: 0,
            potion: configs[last_used as usize].potion.clone(),
            potion_count: 0,
            potion_start_time: 0,

            // In-Game Hotkeys
            confirm: configs[last_used as usize].confirm.clone(),
            cancel: configs[last_used as usize].cancel.clone(),
            macro1: configs[last_used as usize].macro1.clone(),
            macro1_duration: configs[last_used as usize].macro1_duration,
            macro2: configs[last_used as usize].macro2.clone(),
            macro2_duration: configs[last_used as usize].macro2_duration,
            macro3: configs[last_used as usize].macro3.clone(),
            macro3_duration: configs[last_used as usize].macro3_duration,
        }
    }

    /// update changes app's values to match the config file
    pub fn update(&mut self, path: &PathBuf, last_used: i32) {
        let file = fs::read_to_string(path).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("Unable to parse JSON");
        let configs: Vec<Config> = serde_json::from_value(json).unwrap();

        // Settings
        self.max_amount = configs[last_used as usize].amount;

        // Consumables
        self.food = configs[last_used as usize].food.clone();
        self.food_duration = configs[last_used as usize].food_duration;
        self.potion = configs[last_used as usize].potion.clone();

        // In-Game Hotkeys
        self.macro1 = configs[last_used as usize].macro1.clone();
        self.macro1_duration = configs[last_used as usize].macro1_duration;
        self.macro2 = configs[last_used as usize].macro2.clone();
        self.macro2_duration = configs[last_used as usize].macro2_duration;
        self.macro3 = configs[last_used as usize].macro3.clone();
        self.macro3_duration = configs[last_used as usize].macro3_duration;
        self.confirm = configs[last_used as usize].confirm.clone();
        self.cancel = configs[last_used as usize].cancel.clone();
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

        let confirm = utils::get_enigo_key_code(&self.confirm);

        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));

        thread::sleep(Duration::from_secs(ACTION_DELAY));
    }

    /// stop_craft closes the crafting action
    fn stop_craft(&mut self) {
        let mut enigo = Enigo::new();

        let confirm = utils::get_enigo_key_code(&self.confirm);
        let cancel = utils::get_enigo_key_code(&self.cancel);

        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
        enigo.key_click(cancel.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));
        enigo.key_click(confirm.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));

        thread::sleep(Duration::from_secs(ACTION_DELAY));
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

    /// consume_food renews the food buff
    fn consume_food(&mut self) {
        let mut enigo = Enigo::new();

        self.stop_craft();

        let food = utils::get_enigo_key_code(&self.food);
        self.food_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64;
        enigo.key_click(food.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));

        thread::sleep(Duration::from_secs(ACTION_DELAY));

        self.increment_food();

        self.start_craft();
    }

    /// check_potion checks to see whether the potion buff needs to be renewed
    fn check_potion(&mut self) {
        if self.potion_start_time > 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as i64;

            let difference = now - self.food_start_time;

            if difference > POTION_DURATION {
                self.consume_potion();
            }
        } else {
            self.consume_potion();
        }
    }

    /// consume_potion renews the potion buff
    fn consume_potion(&mut self) {
        let mut enigo = Enigo::new();

        self.stop_craft();

        let potion = utils::get_enigo_key_code(&self.potion);
        self.potion_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64;
        enigo.key_click(potion.unwrap());
        thread::sleep(Duration::from_millis(KEY_DELAY));

        thread::sleep(Duration::from_secs(ACTION_DELAY));

        self.increment_potion();

        self.start_craft();
    }
}

// pub fn test(
//     app: &App,
//     program_signal: Arc<AtomicBool>,
//     crafter_signal: Arc<AtomicBool>,
// ) -> mpsc::Receiver<(i32, i32, i32, String)> {
//     let path = app.config.clone();
//     let last_used = app.last_used.clone();

//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         loop {
//             let mut crafter = Crafter::new(&path, last_used);

//             while program_signal.load(Ordering::Relaxed) {
//                 while crafter_signal.load(Ordering::Relaxed) {
//                     tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, String::from("hello 1"))).unwrap();
//                     thread::sleep(Duration::from_millis(500));
//                     tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, String::from("hello 2"))).unwrap();
//                     thread::sleep(Duration::from_millis(500));
//                     tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, String::from("hello 3"))).unwrap();
//                     thread::sleep(Duration::from_millis(500));
                    
//                     crafter.increment_amount();

//                     if crafter.current_amount > crafter.max_amount {
//                         crafter.current_amount = 0;
//                     }
//                 }
//             }
//         }
//     });

//     rx
// }

pub fn craft(
    app: &App,
    program_signal: Arc<AtomicBool>,
    crafter_signal: Arc<AtomicBool>,
) -> mpsc::Receiver<(i32, i32, i32, String)> {
    let path = app.config.clone();
    let last_used = app.last_used.clone();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let mut paused = false;
            let mut crafter = Crafter::new(&path, last_used);

            while program_signal.load(Ordering::Relaxed) {
                // Countdown to allow time for user to focus FFXIV
                if crafter.current_amount == 0 {
                    let msg = String::from("Starting in 5...");
                    tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                    thread::sleep(Duration::from_secs(1));
                    let msg = String::from("Starting in 4...");
                    tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                    thread::sleep(Duration::from_secs(1));
                    let msg = String::from("Starting in 3...");
                    tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                    thread::sleep(Duration::from_secs(1));
                    let msg = String::from("Starting in 2...");
                    tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                    thread::sleep(Duration::from_secs(1));
                    let msg = String::from("Starting in 1...");
                    tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }

                while crafter_signal.load(Ordering::Relaxed) {
                    if paused {
                        // Countdown to allow time for user to focus FFXIV
                        let msg = String::from("Starting in 5...");
                        tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                        thread::sleep(Duration::from_secs(1));
                        let msg = String::from("Starting in 4...");
                        tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                        thread::sleep(Duration::from_secs(1));
                        let msg = String::from("Starting in 3...");
                        tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                        thread::sleep(Duration::from_secs(1));
                        let msg = String::from("Starting in 2...");
                        tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                        thread::sleep(Duration::from_secs(1));
                        let msg = String::from("Starting in 1...");
                        tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                        thread::sleep(Duration::from_secs(1));

                        paused = false;
                    }

                    let mut enigo = Enigo::new();

                    let msg = String::from("Starting craft...");
                    tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();

                    crafter.start_craft();

                    // check food
                    if crafter.food != "" {
                        let msg = String::from("Checking food...");
                        tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();

                        crafter.check_food();
                    }

                    // check potion
                    if crafter.potion != "" {
                        let msg: String = String::from("Checking potion...");
                        tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();

                        crafter.check_potion();
                    }

                    // activate macro 1
                    let msg = String::from("Activating Macro 1...");
                    tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                    let macro1 = utils::get_enigo_key_code(&crafter.macro1);
                    enigo.key_click(macro1.unwrap());
                    thread::sleep(Duration::from_millis(KEY_DELAY));
                    thread::sleep(Duration::from_secs(crafter.macro1_duration));

                    // activate macro 2
                    if crafter.macro2 != "" {
                        let msg = String::from("Activating Macro 2...");
                        tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                        let macro2 = utils::get_enigo_key_code(&crafter.macro2);
                        enigo.key_click(macro2.unwrap());
                        thread::sleep(Duration::from_millis(KEY_DELAY));
                        thread::sleep(Duration::from_secs(crafter.macro2_duration));
                    }

                    // activate macro 3
                    if crafter.macro3 != "" {
                        let msg = String::from("Activating Macro 3...");
                        tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                        let macro3 = utils::get_enigo_key_code(&crafter.macro3);
                        enigo.key_click(macro3.unwrap());
                        thread::sleep(Duration::from_millis(KEY_DELAY));
                        thread::sleep(Duration::from_secs(crafter.macro3_duration));
                    }

                    crafter.increment_amount();
                    if crafter.current_amount >= crafter.max_amount {
                        program_signal.store(false, Ordering::Relaxed);
                        crafter_signal.store(false, Ordering::Relaxed);
                    }

                    thread::sleep(Duration::from_secs(ACTION_DELAY));
                }

                let msg = String::from("Waiting...");
                tx.send((crafter.current_amount, crafter.food_count, crafter.potion_count, msg)).unwrap();
                if !paused {
                    crafter.update(&path, last_used);
                }
                paused = true;

            }
        }
    });

    rx
}
