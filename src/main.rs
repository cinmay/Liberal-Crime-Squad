use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};


enum Event<I> {
    Input(I),
    Tick,
}


#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Opening,
    Home,
    SavegameName,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Opening => 0,
            MenuItem::Home => 1,
            MenuItem::SavegameName => 2,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut active_menu_item = MenuItem::Home;
    let mut pet_list_state = ListState::default();
    pet_list_state.select(Some(0));

    let mut savefile_name = String::new();
    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            match active_menu_item {
                MenuItem::Opening => rect.render_widget(render_opening(), chunks[1]),
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::SavegameName => rect.render_widget(render_savegame_name(&savefile_name), chunks[1]),
            }
        })?;

        match active_menu_item {
            MenuItem::Opening => {
                match rx.recv()? {
                    Event::Input(event) => match event.code {
                        KeyCode::Esc => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
                        _ => active_menu_item = MenuItem::SavegameName,
                    },
                    Event::Tick => {}
                }

            }
            MenuItem::Home => {
                match rx.recv()? {
                    Event::Input(event) => match event.code {
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                        KeyCode::Char('o') => active_menu_item = MenuItem::Opening,
                        _ => {}
                    },
                    Event::Tick => {}
                }
            }
            MenuItem::SavegameName  => {
                match rx.recv()? {
                    Event::Input(event) => match event.code {
                        KeyCode::Esc => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Enter =>  active_menu_item = MenuItem::Home,

                        KeyCode::Backspace => {
                            savefile_name.pop();
                        }
                        _ => {
                            if let KeyCode::Char(c) = event.code {
                                savefile_name.push(c);
                            }
                        }
                    },
                    Event::Tick => {}
                }
            }
        }
    }

    Ok(())
}
fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "pet-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Hello Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}
fn render_opening<'a>() -> Paragraph<'a> {
    let opening = Paragraph::new(vec![
        Spans::from(vec![Span::styled(
                "Liberal Crime Squad",
                Style::default().fg(Color::Green),
            )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Inspired by the 1983 version of Oubliette")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("\"Let's get on with it!\"")]),
        Spans::from(vec![Span::raw("-- Grimith")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("v3.9 Copyright (C) 2002-4, Tarn Adams")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("A Bay 12 Games Production")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("http://bay12games.com/lcs/")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("A Rust Rewright, mostly for fun and learning")]),
        Spans::from(vec![Span::raw("Maintained by the Open Source Community")]),
        Spans::from(vec![Span::raw("https://github.com/cinmay/Liberal-Crime-Squad")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("A huge thanks to the 4.12 community for their work on imporoving game.")]),
        Spans::from(vec![Span::raw("https://github.com/King-Drake/Liberal-Crime-Squad")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press ESC now to quit. Quitting later causes your progress to be saved.")]),
        Spans::from(vec![Span::raw("Press any other key to pursue your Liberal Agenda!")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Opening")
            .border_type(BorderType::Plain),
    );
    opening
}

fn render_savegame_name<'a>(savefile_name:&'a String ) -> Paragraph<'a> {
    let savegame_name = Paragraph::new(vec![
        Spans::from(vec![Span::raw("In what world will you pursue your Liberal Agenda?")]),
        Spans::from(vec![Span::raw("Enter a name for the save file.")]),
        Spans::from(vec![Span::raw(savefile_name)]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Savegame Name")
            .border_type(BorderType::Plain),
    );
    savegame_name
}
