use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Starts XIVCrafter
  Start(Start),

  /// Prints XIVCrafter's configuration
  Config(Config),
}

/// Start subcommand
#[derive(Parser)]
struct Start {
  /// Amount to craft
  #[arg(long)]
  amount: Option<u32>,

  /// Cancel hotkey
  #[arg(long, value_name = "HOTKEY")]
  cancel: Option<String>,

  /// Confirm hotkey
  #[arg(long, value_name = "HOTKEY")]
  confirm: Option<String>,

  /// Food hotkey
  #[arg(long, value_name = "HOTKEY")]
  food: Option<String>,

  /// Food duration (minutes)
  #[arg(long, value_name = "30|40|45")]
  food_duration: Option<u32>,

  /// Macro 1 hotkey
  #[arg(long, value_name = "HOTKEY")]
  macro1: Option<String>,

  /// Macro 1 duration (seconds)
  #[arg(long, value_name = "SECONDS")]
  macro1_duration: Option<u32>,

  /// Macro 2 hotkey
  #[arg(long, value_name = "HOTKEY")]
  macro2: Option<String>,

  /// Macro 2 duration (seconds)
  #[arg(long, value_name = "SECONDS")]
  macro2_duration: Option<u32>,

  /// Potion hotkey
  #[arg(long, value_name = "HOTKEY")]
  potion: Option<String>,

  /// Start/Pause hotkey
  #[arg(long, value_name = "HOTKEY")]
  start_pause: Option<String>,

  /// Stop hotkey
  #[arg(long, value_name = "HOTKEY")]
  stop: Option<String>,

  /// Sets a custom config file
  #[arg(short, long, value_name = "FILE")]
  config: Option<PathBuf>,

  /// Verbose output
  #[arg(short, long)]
  verbose: bool,
}

/// Config subcommand
#[derive(Parser)]
struct Config {
  /// Amount to craft
  #[arg(long)]
  amount: Option<u32>,

  /// Cancel hotkey
  #[arg(long)]
  cancel: Option<String>,

  /// Confirm hotkey
  #[arg(long)]
  confirm: Option<String>,

  /// Food hotkey
  #[arg(long)]
  food: Option<String>,

  /// Food duration (minutes)
  #[arg(long)]
  food_duration: Option<u32>,

  /// Macro 1 hotkey
  #[arg(long)]
  macro1: Option<String>,

  /// Macro 1 duration (seconds)
  #[arg(long)]
  macro1_duration: Option<u32>,

  /// Macro 2 hotkey
  #[arg(long)]
  macro2: Option<String>,

  /// Macro 2 duration (seconds)
  #[arg(long)]
  macro2_duration: Option<u32>,

  /// Potion hotkey
  #[arg(long)]
  potion: Option<String>,

  /// Start/Pause hotkey
  #[arg(long)]
  start_pause: Option<String>,

  /// Stop hotkey
  #[arg(long)]
  stop: Option<String>,

  /// Sets a custom config file
  #[arg(short, long, value_name = "FILE")]
  config: Option<PathBuf>,

  /// Verbose output
  #[arg(short, long)]
  verbose: bool,
}

/// Config file
#[derive(Serialize, Deserialize)]
struct ConfigData {
  amount: u32,
  cancel: String,
  confirm: String,
  food: String,
  food_duration: u32,
  macro1: String,
  macro1_duration: u32,
  macro2: String,
  macro2_duration: u32,
  potion: String,
  start_pause: String,
  stop: String,
}

fn main() {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Start(start) => {
      // parse config
      let mut config_file = ConfigData {
        amount: 0,
        cancel: "".to_string(),
        confirm: "".to_string(),
        food: "".to_string(),
        food_duration: 0,
        macro1: "".to_string(),
        macro1_duration: 0,
        macro2: "".to_string(),
        macro2_duration: 0,
        potion: "".to_string(),
        start_pause: "".to_string(),
        stop: "".to_string(),
      };
      if start.config.is_some() {
        let file = std::fs::File::open(start.config.as_ref().unwrap()).expect("Could not open file.");
        config_file = serde_yaml::from_reader(file).expect("Could not read values.");
      }

      // amount
      let amount: u32;
      if start.amount.is_none() {
        amount = config_file.amount;
      } else {
        amount = start.amount.unwrap();
      }

      // cancel
      let cancel: String;
      if start.cancel.is_none() {
        cancel = config_file.cancel;
      } else {
        cancel = start.cancel.as_ref().unwrap().to_string();
      }

      // confirm
      let confirm: String;
      if start.confirm.is_none() {
        confirm = config_file.confirm;
      } else {
        confirm = start.confirm.as_ref().unwrap().to_string();
      }

      // food
      let food: String;
      if start.food.is_none() {
        food = config_file.food;
      } else {
        food = start.food.as_ref().unwrap().to_string();
      }

      // food_duration
      let food_duration: u32;
      if start.food_duration.is_none() {
        food_duration = config_file.food_duration;
      } else {
        food_duration = start.food_duration.unwrap();
      }

      // macro1
      let macro1: String;
      if start.macro1.is_none() {
        macro1 = config_file.macro1;
      } else {
        macro1 = start.macro1.as_ref().unwrap().to_string();
      }

      // macro1_duration
      let macro1_duration: u32;
      if start.macro1_duration.is_none() {
        macro1_duration = config_file.macro1_duration;
      } else {
        macro1_duration = start.macro1_duration.unwrap();
      }

      // macro2
      let macro2: String;
      if start.macro2.is_none() {
        macro2 = config_file.macro2;
      } else {
        macro2 = start.macro2.as_ref().unwrap().to_string();
      }

      // macro2_duration
      let macro2_duration: u32;
      if start.macro2_duration.is_none() {
        macro2_duration = config_file.macro2_duration;
      } else {
        macro2_duration = start.macro2_duration.unwrap();
      }

      // potion
      let potion: String;
      if start.potion.is_none() {
        potion = config_file.potion;
      } else {
        potion = start.potion.as_ref().unwrap().to_string();
      }

      // start_pause
      let start_pause: String;
      if start.start_pause.is_none() {
        start_pause = config_file.start_pause;
      } else {
        start_pause = start.start_pause.as_ref().unwrap().to_string();
      }

      // stop
      let stop: String;
      if start.stop.is_none() {
        stop = config_file.stop;
      } else {
        stop = start.stop.as_ref().unwrap().to_string();
      }

      let _xiv = init(amount, cancel, confirm, food, food_duration, macro1, macro1_duration, macro2, macro2_duration, potion, start_pause, stop);
    },

    Commands::Config(config) => {
      // parse config
      let mut config_file = ConfigData {
        amount: 0,
        cancel: "".to_string(),
        confirm: "".to_string(),
        food: "".to_string(),
        food_duration: 0,
        macro1: "".to_string(),
        macro1_duration: 0,
        macro2: "".to_string(),
        macro2_duration: 0,
        potion: "".to_string(),
        start_pause: "".to_string(),
        stop: "".to_string(),
      };
      if config.config.is_some() {
        let file = std::fs::File::open(config.config.as_ref().unwrap()).expect("Could not open file.");
        config_file = serde_yaml::from_reader(file).expect("Could not read values.");
      }

      // amount
      let amount: u32;
      if config.amount.is_none() {
        amount = config_file.amount;
      } else {
        amount = config.amount.unwrap();
      }

      // cancel
      let cancel: String;
      if config.cancel.is_none() {
        cancel = config_file.cancel;
      } else {
        cancel = config.cancel.as_ref().unwrap().to_string();
      }

      // confirm
      let confirm: String;
      if config.confirm.is_none() {
        confirm = config_file.confirm;
      } else {
        confirm = config.confirm.as_ref().unwrap().to_string();
      }

      // food
      let food: String;
      if config.food.is_none() {
        food = config_file.food;
      } else {
        food = config.food.as_ref().unwrap().to_string();
      }

      // food_duration
      let food_duration: u32;
      if config.food_duration.is_none() {
        food_duration = config_file.food_duration;
      } else {
        food_duration = config.food_duration.unwrap();
      }

      // macro1
      let macro1: String;
      if config.macro1.is_none() {
        macro1 = config_file.macro1;
      } else {
        macro1 = config.macro1.as_ref().unwrap().to_string();
      }

      // macro1_duration
      let macro1_duration: u32;
      if config.macro1_duration.is_none() {
        macro1_duration = config_file.macro1_duration;
      } else {
        macro1_duration = config.macro1_duration.unwrap();
      }

      // macro2
      let macro2: String;
      if config.macro2.is_none() {
        macro2 = config_file.macro2;
      } else {
        macro2 = config.macro2.as_ref().unwrap().to_string();
      }

      // macro2_duration
      let macro2_duration: u32;
      if config.macro2_duration.is_none() {
        macro2_duration = config_file.macro2_duration;
      } else {
        macro2_duration = config.macro2_duration.unwrap();
      }

      // potion
      let potion: String;
      if config.potion.is_none() {
        potion = config_file.potion;
      } else {
        potion = config.potion.as_ref().unwrap().to_string();
      }

      // start_pause
      let start_pause: String;
      if config.start_pause.is_none() {
        start_pause = config_file.start_pause;
      } else {
        start_pause = config.start_pause.as_ref().unwrap().to_string();
      }

      // stop
      let stop: String;
      if config.stop.is_none() {
        stop = config_file.stop;
      } else {
        stop = config.stop.as_ref().unwrap().to_string();
      }

      println!("amount: {}", amount);
      println!("cancel: {}", cancel);
      println!("confirm: {}", confirm);
      println!("food: {}", food);
      println!("food_duration: {}", food_duration);
      println!("macro1: {}", macro1);
      println!("macro1_duration: {}", macro1_duration);
      println!("macro2: {}", macro2);
      println!("macro2_duration: {}", macro2_duration);
      println!("potion: {}", potion);
      println!("start_pause: {}", start_pause);
      println!("stop: {}", stop);
    },
  }
}

/// Crafter struct
struct Crafter {
  running: bool,
  program_running: bool,
  food: String,
  food_duration: u32,
  food_count: u32,
  start_food_time: u32,
  potion: String,
  potion_count: u32,
  start_potion_time: u32,
  current_amount: u32,
  max_amount: u32,
  macro1: String,
  macro1_duration: u32,
  macro2: String,
  macro2_duration: u32,
  cancel: String,
  confirm: String,
  start_pause: String,
  stop: String
}

/// Initialize Crafter struct
fn init(amount: u32, cancel: String, confirm: String, food: String, food_duration: u32, macro1: String, macro1_duration: u32, macro2: String, macro2_duration: u32, potion: String, start_pause: String, stop: String) -> Crafter{
  return Crafter {
    running: false,
    program_running: true,
    food: food.trim().to_lowercase(),
    food_duration: food_duration,
    food_count: 0,
    start_food_time: 0,
    potion: potion.trim().to_lowercase(),
    potion_count: 0,
    start_potion_time: 0,
    current_amount: 0,
    max_amount: amount,
    macro1: macro1.trim().to_lowercase(),
    macro1_duration: macro1_duration,
    macro2: macro2.trim().to_lowercase(),
    macro2_duration: macro2_duration,
    cancel: cancel.trim().to_lowercase(),
    confirm: confirm.trim().to_lowercase(),
    start_pause: start_pause.trim().to_lowercase(),
    stop: stop.trim().to_lowercase()
  }
}
