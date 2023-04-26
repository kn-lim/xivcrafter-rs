use enigo::*;

const START_CRAFT_DELAY: u32 = 2;
const END_CRAFT_DELAY: u32 = 3;
const KEY_DELAY: u32 = 1;
const POTION_DURATION: u32 = 900;

/// Crafter struct
pub struct Crafter {
    /// Is crafting in progress?
    pub running: bool,

    /// Is XIVCrafter currently running?
    pub program_running: bool,

    /// Food macro
    pub food: String,

    /// Food duration
    pub food_duration: u32,

    /// Amount of food used
    pub food_count: u32,

    /// Food consumption start time
    pub start_food_time: u32,

    /// Potion macro
    pub potion: String,

    /// Amount of potions used
    pub potion_count: u32,

    /// Potion consumption start time
    pub start_potion_time: u32,

    /// Current amount crafted
    pub current_amount: u32,

    /// Max amount to craft
    pub max_amount: u32,

    /// Macro 1
    pub macro1: String,

    /// Macro 1 duration
    pub macro1_duration: u32,

    /// Macro 2
    pub macro2: String,

    /// Macro 2 duration
    pub macro2_duration: u32,

    /// Cancel macro
    pub cancel: String,

    /// Confirm macro
    pub confirm: String,

    /// Start/Pause macro
    pub start_pause: String,

    /// Stop macro
    pub stop: String,

    /// Start time of XIVCrafter
    pub start_time: u32,

    /// End time of XIVCrafter
    pub end_time: u32,

    /// Verbose output
    pub verbose: bool,
}

impl Crafter {
    pub fn init(
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
        verbose: bool,
    ) -> Crafter {
        let food_duration_seconds = food_duration * 60;

        return Crafter {
            running: false,
            program_running: true,
            food: food.trim().to_lowercase(),
            food_duration: food_duration_seconds,
            food_count: 0,
            start_food_time: 0,
            potion: potion.trim().to_lowercase(),
            potion_count: 0,
            start_potion_time: 0,
            current_amount: 0,
            max_amount: amount,
            macro1: macro1.trim().to_lowercase(),
            macro1_duration,
            macro2: macro2.trim().to_lowercase(),
            macro2_duration,
            cancel: cancel.trim().to_lowercase(),
            confirm: confirm.trim().to_lowercase(),
            start_pause: start_pause.trim().to_lowercase(),
            stop: stop.trim().to_lowercase(),
            start_time: 0,
            end_time: 0,
            verbose,
        };
    }

    pub fn run(&mut self) {
        println!("Press \"{}\" to Start/Pause", &self.start_pause);
        println!("Press \"{}\" to Stop", &self.stop);

        // while self.program_running {
        // while self.running {
        self.start_craft();

        if self.food != String::from("") {
            self.check_food();
        }

        if self.potion != String::from("") {
            self.check_potion();
        }

        self.activate_macro();
        // }
        // }
    }

    fn start_program(&self) {
        println!("Starting XIVCrafter");
    }

    fn stop_program(&self) {
        println!("Stopping XIVCrafter");
    }

    fn exit_program(&self) {
        println!("Exiting XIVCrafter");
    }

    fn start_craft(&self) {
        println!("Starting craft");
    }

    fn stop_craft(&self) {
        println!("Stopping craft");
    }

    fn check_food(&self) {
        println!("Checking food");
    }

    fn consume_food(&self) {
        println!("Consuming food");
    }

    fn check_potion(&self) {
        println!("Checking potion");
    }

    fn consume_potion(&self) {
        println!("Consuming potion");
    }

    fn activate_macro(&self) {
        println!("Activating macro");

        // Activate Macro 1

        self.delay(self.macro1_duration);

        // Activate Macro 2
        if self.macro2 != String::from("") {
            self.delay(self.macro2_duration);
        }
    }

    fn increment(&self) {
        
    }

    fn delay(&self, time: u32) {
        println!("Delay: {}", time);
    }
}

/// Initialize Crafter struct
pub fn init(
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
    verbose: bool,
) -> Crafter {
    let food_duration_seconds = food_duration * 60;

    return Crafter {
        running: false,
        program_running: true,
        food: food.trim().to_lowercase(),
        food_duration: food_duration_seconds,
        food_count: 0,
        start_food_time: 0,
        potion: potion.trim().to_lowercase(),
        potion_count: 0,
        start_potion_time: 0,
        current_amount: 0,
        max_amount: amount,
        macro1: macro1.trim().to_lowercase(),
        macro1_duration,
        macro2: macro2.trim().to_lowercase(),
        macro2_duration,
        cancel: cancel.trim().to_lowercase(),
        confirm: confirm.trim().to_lowercase(),
        start_pause: start_pause.trim().to_lowercase(),
        stop: stop.trim().to_lowercase(),
        start_time: 0,
        end_time: 0,
        verbose,
    };
}
