use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use serde::{Deserialize, Serialize};

use std::{error::Error, fs, io, path::PathBuf};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table, Tabs, Wrap},
    Frame, Terminal,
};

/// Config file
#[derive(Serialize, Deserialize, Clone)]
struct Config {
    id: i32,
    last_used: bool,
    name: String,
    amount: i32,
    food: String,
    food_duration: i32,
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

struct App<'a> {
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
    pub food_duration: i32,
    pub food_start_time: i32,

    // Potion
    pub potion: String,
    pub potion_count: i32,
    pub potion_start_time: i32,

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
    fn new(path: PathBuf) -> App<'a> {
        let file = fs::read_to_string(&path).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("Unable to parse JSON");

        let configs: Vec<Config> = serde_json::from_value(json).unwrap();

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

    pub fn increment_amount(&mut self) {
        self.current_amount += 1;
    }

    pub fn increment_food(&mut self) {
        self.food_count += 1;
    }

    pub fn increment_potion(&mut self) {
        self.potion_count += 1;
    }

    /// update changes app's values to match the config file
    pub fn update(&mut self) {
        let file = fs::read_to_string(self.config.clone()).expect("Unable to read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("Unable to parse JSON");

        let configs: Vec<Config> = serde_json::from_value(json).unwrap();

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
}

fn main() -> Result<(), Box<dyn Error>> {
    // FOR DEBUG PURPOSES
    // UNCOMMENT TO USE
    // env::set_var("RUST_BACKTRACE", "1");

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
    let mut app = App::new(path);
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // Setup XIVCrafter
    // TODO

    loop {
        // Update settings in app
        app.update();

        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // Tabs
    let tabs = app
        .tabs
        .iter()
        .map(|t| Spans::from(vec![Span::styled(*t, Style::default().fg(Color::Green))]))
        .collect();
    let tabs_content = Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL).title("XIVCrafter"))
        .select(app.index)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );
    f.render_widget(tabs_content, chunks[0]);

    match app.index {
        0 => ui_home(f, app, chunks[1]),
        1 => ui_config(f, app, chunks[1]),
        _ => {}
    };
}

// Home Tab
fn ui_home<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    // Settings
    let amount = &app.max_amount.to_string();
    let food_duration = &app.food_duration.to_string();
    let macro1_duration = &app.macro1_duration.to_string();
    let macro2_duration = &app.macro2_duration.to_string();
    let macro3_duration = &app.macro3_duration.to_string();

    let mut rows = Vec::new();
    rows.push(Row::new(vec!["Name:", &app.name]));
    rows.push(Row::new(vec!["Amount:", amount]));

    if &app.food != "" {
        rows.push(Row::new(vec!["Food:", &app.food]));
        rows.push(Row::new(vec!["Food Duration:", food_duration]));
    }

    if &app.potion != "" {
        rows.push(Row::new(vec!["Potion:", &app.potion]));
    }

    rows.push(Row::new(vec!["Macro 1:", &app.macro1]));
    rows.push(Row::new(vec!["Macro 1 Duration:", macro1_duration]));

    if &app.macro2 != "" {
        rows.push(Row::new(vec!["Macro 2:", &app.macro2]));
        rows.push(Row::new(vec!["Macro 2 Duration:", macro2_duration]));
    }

    if &app.macro3 != "" {
        rows.push(Row::new(vec!["Macro 3:", &app.macro3]));
        rows.push(Row::new(vec!["Macro 3 Duration:", macro3_duration]));
    }

    rows.push(Row::new(vec!["Start/Pause:", &app.start_pause]));
    rows.push(Row::new(vec!["Stop", &app.stop]));
    rows.push(Row::new(vec!["Confirm", &app.confirm]));
    rows.push(Row::new(vec!["Cancel", &app.cancel]));

    let table = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Settings").borders(Borders::ALL))
        .widths(&[Constraint::Percentage(40), Constraint::Percentage(60)]);
    f.render_widget(table, chunks[0]);

    // Status
    let mut title = String::from("Status: ");
    let mut block = Block::default().borders(Borders::ALL);
    if !&app.program_running {
        title.push_str("WAITING TO START");
        block = block.title(title);
    } else {
        if app.running {
            title.push_str("CRAFTING");
            block = block.title(title).style(Style::default().fg(Color::Green));
        } else {
            title.push_str("PAUSED");
            block = block.title(title).style(Style::default().fg(Color::Red));
        }
    }
    f.render_widget(block, chunks[1]);

    let status = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1), // Instructions
                Constraint::Length(1), // Instructions
                Constraint::Length(1), // Empty
                Constraint::Length(2), // Progress Gauge
                Constraint::Length(1), // Empty
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Print Instructions
    let mut instructions_1 = String::from("Press \"");
    instructions_1.push_str(&app.start_pause);
    instructions_1.push_str("\" to Start/Pause");
    f.render_widget(Paragraph::new(instructions_1), status[0]);

    let mut instructions_2 = String::from("Press \"");
    instructions_2.push_str(&app.stop);
    instructions_2.push_str("\" to Stop");
    f.render_widget(Paragraph::new(instructions_2), status[1]);

    // Progress Gauge
    let mut progress = (app.current_amount * 100 / app.max_amount) as u16;
    if progress >= 100 {
        progress = 100;
    }

    let current_amount = app.current_amount.to_string();
    let max_amount = app.max_amount.to_string();

    title = String::from("Crafted: ");
    title.push_str(&current_amount);
    title.push_str("/");
    title.push_str(&max_amount);
    let gauge = Gauge::default()
        .block(Block::default().title(title))
        .gauge_style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .percent(progress);
    f.render_widget(gauge, status[3])
}

// Config Tab
fn ui_config<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Vertical)
        .split(area);

    let home = vec![Spans::from(app.config.display().to_string())];
    let content = Paragraph::new(home)
        .block(
            Block::default()
                .title("Config Content")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(content, chunks[0]);

    let file = fs::read_to_string(&app.config).expect("Error reading file");

    let status = vec![Spans::from(file)];
    let status_content = Paragraph::new(status)
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    f.render_widget(status_content, chunks[1]);
}
