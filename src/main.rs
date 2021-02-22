use aoc::ui::app::App;
use std::io;
use termion::input::TermRead;
use termion::{raw::IntoRawMode, screen::AlternateScreen};

use tui::{backend::TermionBackend, Terminal};

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::default();
    let mut counter = 100;
    loop {
        terminal.draw(|f| app.draw(f))?;
        std::thread::sleep(std::time::Duration::from_millis(200));
        counter -= 1;
        if counter == 0 {
            break;
        }
    }
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let _ = io::stdin().read_passwd(&mut stdout).unwrap().unwrap();

    Ok(())
}
