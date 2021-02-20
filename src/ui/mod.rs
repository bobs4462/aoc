use std::io::Stdout;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Frame;

pub mod app;
mod welcome;

type TermionFrame<'a> = Frame<'a, TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>;
