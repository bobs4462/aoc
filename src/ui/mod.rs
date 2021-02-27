use std::io::Stdout;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Frame;

pub mod app;

type TermionFrame<'a> = Frame<'a, TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>;

pub enum Movement {
    Up = -1,
    Down = 1,
}
