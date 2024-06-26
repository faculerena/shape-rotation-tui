use std::io;
use crossterm::event::{self, Event, KeyCode};

mod app;
mod ui;
mod cube;
mod terminal;

use app::App;
use terminal::{init_terminal, restore_terminal};

fn main() -> io::Result<()> {
    let mut terminal = init_terminal()?;
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') => app.rotate("x", 0.05),
                KeyCode::Char('k') => app.rotate("y", 0.05),
                KeyCode::Char('l') => app.rotate("z", 0.05),
                KeyCode::Char('u') => app.rotate("x", -0.05),
                KeyCode::Char('i') => app.rotate("y", -0.05),
                KeyCode::Char('o') => app.rotate("z", -0.05),
                _ => {}
            }
        }
    }

    restore_terminal()
}
