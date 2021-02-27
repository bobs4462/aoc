use crate::config::Config;

use super::{puzzles::Puzzles, TermionFrame};
use super::{welcome::Welcome, Movement};

pub struct App {
    welcome: Welcome,
    puzzles: Puzzles,
    current_state: AppState,
    pub config: Config,
}

enum AppState {
    WelcomeScreen,
    PuzzlesScreen,
}

impl Default for App {
    fn default() -> Self {
        App {
            welcome: Welcome::default(),
            puzzles: Puzzles::default(),
            current_state: AppState::WelcomeScreen,
            config: Config::default(),
        }
    }
}

impl App {
    pub fn draw(&mut self, f: &mut TermionFrame) {
        match self.current_state {
            AppState::WelcomeScreen => self.welcome.draw(f),
            AppState::PuzzlesScreen => self.puzzles.draw(f),
        }
    }

    pub fn move_selected(&mut self, movement: Movement) {
        match self.current_state {
            AppState::WelcomeScreen => self.welcome.move_selected(movement),
            AppState::PuzzlesScreen => self.puzzles.move_selected(movement),
        }
    }

    pub fn pick(&mut self) {
        match self.current_state {
            AppState::WelcomeScreen => {
                self.config.year = self.welcome.pick();
                self.current_state = AppState::PuzzlesScreen;
            }
            AppState::PuzzlesScreen => {
                let (day, part) = self.puzzles.pick();
                if day != 0 {
                    self.config.challenge = day;
                }
                if part != 0 {
                    self.config.part = part;
                }
            }
        }
    }
}
