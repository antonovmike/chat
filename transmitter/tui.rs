use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::{db::data_base, transmitter};

enum InputMode {
    Normal,
    Editing,
}
struct App {
    scroll: u16,
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
}

impl App {
    fn new() -> App {
        App {
            scroll: 0,
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
    }
}

pub fn run_ui() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(60);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.messages.push(app.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Percentage(75),
                Constraint::Percentage(0),
                Constraint::Percentage(0),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(size);

    let text_db = data_base();

    let mut previous_mess = vec![];
    for sp in text_db.clone() {
        previous_mess.push(Spans::from(sp))
    }

    // Text visible while typing. Disappears after Enter
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Red),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Write a message: ")
                .title_alignment(Alignment::Center)
                .style(Style::default().add_modifier(Modifier::BOLD)),
        );
    f.render_widget(input, chunks[3]);

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[3].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[3].y + 1,
            )
        }
    }

    // Typed messages
    let mut messages_ty: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(_i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}", m)))];
            ListItem::new(content)
        })
        .collect();
    // Messages from db
    let mut messages_db: Vec<ListItem> = text_db
        .iter().rev().take(7)
        .enumerate()
        .map(|(_i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}", m)))];
            share_text(m);
            ListItem::new(content)
        })
        .collect();

    messages_db.append(& mut messages_ty);

    let messages =
        List::new(messages_db)
            .block(Block::default()
            .borders(Borders::ALL)
            .title(" Chat ")
            .title_alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD))
        );
    f.render_widget(messages, chunks[0]);
}

fn share_text(msg: &String) {
    transmitter::send_text((msg.clone(), true));
}