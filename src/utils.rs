use crossterm::event::KeyCode;

use enigo::Key;

use serde::{Deserialize, Serialize};

/// Config file
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub id: i32,
    pub last_used: bool,
    pub name: String,
    pub amount: i32,
    pub food: String,
    pub food_duration: i64,
    pub potion: String,
    pub macro1: String,
    pub macro1_duration: u64,
    pub macro2: String,
    pub macro2_duration: u64,
    pub macro3: String,
    pub macro3_duration: u64,
    pub start_pause: String,
    pub stop: String,
    pub confirm: String,
    pub cancel: String,
}

pub fn get_crossterm_key_code(key: &str) -> Option<KeyCode> {
    let key = key.to_lowercase();
    if key.len() == 1 {
        let c = key.chars().next().unwrap();
        return Some(KeyCode::Char(c));
    }
    match key.as_str() {
        "home" => Some(KeyCode::Home),
        "end" => Some(KeyCode::End),
        "pageup" => Some(KeyCode::PageUp),
        "pagedown" => Some(KeyCode::PageDown),
        "delete" => Some(KeyCode::Delete),
        _ => None,
    }
}

pub fn get_enigo_key_code(key: &str) -> Option<Key> {
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
