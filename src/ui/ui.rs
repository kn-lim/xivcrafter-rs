use crate::App;

use std::fs;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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
pub fn ui_home<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .direction(Direction::Horizontal)
        .split(area);

    draw_settings(f, app, chunks[0]);

    draw_status(f, app, chunks[1]);
}

fn draw_settings<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
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
  f.render_widget(table, area);
}

fn draw_status<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
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
  f.render_widget(block, area);

  let status = Layout::default()
      .direction(Direction::Vertical)
      .margin(1)
      .constraints(
          [
              Constraint::Length(1), // Instructions
              Constraint::Length(1), // Instructions
              Constraint::Length(1), // Empty
              Constraint::Length(4), // Progress Gauge
              Constraint::Length(1), // Empty
          ]
          .as_ref(),
      )
      .split(area);

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
pub fn ui_config<B>(f: &mut Frame<B>, app: &App, area: Rect)
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
