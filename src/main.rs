use aoc::{event::EventProvider, ui::app::App};
use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};

use tui::{backend::TermionBackend, Terminal};

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::default();
    let events = EventProvider::new();
    loop {
        terminal.draw(|f| app.draw(f))?;
        if let Ok(v) = events.next() {
            match v {
                aoc::event::Event::Tick => {}
                aoc::event::Event::KeyPress(k) => match k {
                    Key::Char('j') | Key::Down => app.select(aoc::ui::Movement::Down),
                    Key::Char('k') | Key::Up => app.select(aoc::ui::Movement::Up),
                    Key::Char('\n') => app.confirm(),

                    k if (k == Key::Char('q')) => {
                        break;
                    }
                    _ => {}
                },
            }
        }
    }
    drop(terminal);

    Ok(())
}
