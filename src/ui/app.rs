use crate::config::Config;

use super::TermionFrame;
use super::{welcome::Welcome, Movement};

pub struct App {
    welcome: Welcome,
    current_state: AppState,
    pub config: Config,
}

enum AppState {
    WelcomeScreen,
}

impl Default for App {
    fn default() -> Self {
        App {
            welcome: Welcome::default(),
            current_state: AppState::WelcomeScreen,
            config: Config::default(),
        }
    }
}

impl App {
    pub fn draw(&mut self, f: &mut TermionFrame) {
        match self.current_state {
            AppState::WelcomeScreen => self.welcome.draw(f),
        }
    }

    pub fn move_selected(&mut self, movement: Movement) {
        match self.current_state {
            AppState::WelcomeScreen => self.welcome.move_selected(movement),
        }
    }

    pub fn pick(&mut self) {
        match self.current_state {
            AppState::WelcomeScreen => {
                self.config.year = self.welcome.pick();
            }
        }
    }
}
