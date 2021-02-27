use aoc::{event::EventProvider, ui::app::App};
use std::io;
// use termion::input::TermRead;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};

use tui::{backend::TermionBackend, Terminal};

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::default();
    let events = EventProvider::with_config(&app.config);
    loop {
        terminal.draw(|f| app.draw(f))?;
        if let Ok(v) = events.next() {
            match v {
                aoc::event::Event::Tick => {}
                aoc::event::Event::KeyPress(k) => match k {
                    Key::Char('j') | Key::Down => app.move_selected(aoc::ui::Movement::Down),
                    Key::Char('k') | Key::Up => app.move_selected(aoc::ui::Movement::Up),
                    Key::Char('l') | Key::Right => app.move_selected(aoc::ui::Movement::Right),
                    Key::Char('h') | Key::Left => app.move_selected(aoc::ui::Movement::Left),
                    Key::Char('\n') => app.pick(),

                    k if (k == app.config.quit_key) => {
                        break;
                    }
                    _ => {}
                },
            }
        }
    }
    drop(terminal);
    print!("{}", app.config);

    Ok(())
}
