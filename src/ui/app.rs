use crate::config::Config;

use super::welcome::Welcome;
use super::TermionFrame;

pub struct App {
    welcome: Welcome,
    current_state: AppState,
    config: Config,
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
        self.welcome.draw(f);
    }
}
