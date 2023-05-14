mod app;
mod config;
mod hiraganas;
mod ui;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use hiraganas::get_hiraganas;
use rand::thread_rng;
use std::{cell::Cell, io::stdout};
use tui::{backend::CrosstermBackend, Terminal};

fn handle_input(key: KeyEvent, points: &Cell<u16>, app: &mut app::App) -> Result<(), ()> {
    match key.code {
        KeyCode::Esc => {
            return Err(());
        }
        KeyCode::Char(c) => {
            app.get_input_mut().push(c);
        }
        KeyCode::Backspace => {
            app.get_input_mut().pop();
        }
        KeyCode::Enter => {
            if app.get_popup().is_some() {
                app.set_popup(None);
                return Ok(());
            }
            let hiragana = app.current_hiragana().unwrap();
            if hiragana
                .get_roumanji()
                .eq_ignore_ascii_case(app.get_input())
            {
                points.set(points.get() + 1);
            } else {
                app.set_popup(Some(format!("Wrong, it's {}", hiragana.get_roumanji())));
            }
            app.get_input_mut().clear();
            if app.next_hiragana().is_none() {
                return Err(());
            }
        }
        _ => {}
    };
    Ok(())
}

#[allow(unused_mut)]
fn main() -> Result<(), std::io::Error> {
    let mut points = Cell::new(0);
    let mut rng = thread_rng();
    let mut hiraganas = get_hiraganas(&mut rng);
    let mut _app = app::App::new(&mut hiraganas);

    // setup terminal
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui::ui(f, &mut _app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if handle_input(key, &points, &mut _app).is_err() {
                break;
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    println!("You have {}/{} points.", points.get(), hiraganas.len());
    Ok(())
}
