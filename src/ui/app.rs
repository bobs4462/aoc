use tui::layout::Rect;

use crate::challenge::Challenge;

use super::{Movement, TermionFrame};

pub struct App {
    state: AppState,
    challenge: Challenge,
}

enum AppState {
    SelectingYear,
    SelectingDay,
    SelectingPart,
    SelectingFile,
    Solving,
}

impl Default for App {
    fn default() -> Self {
        App {
            state: AppState::SelectingYear,
            challenge: Challenge::default(),
        }
    }
}

impl App {
    pub fn draw(&mut self, f: &mut TermionFrame) {
        let mut chunks: Vec<Rect>;
        match self.state {
            AppState::SelectingYear => {}
            AppState::SelectingDay => {}
            AppState::SelectingPart => {}
            AppState::SelectingFile => {}
            AppState::Solving => {}
        }
    }

    fn build_layout(&self, area: Rect) {}

    pub fn select(&mut self, movement: Movement) {
        match self.state {
            AppState::SelectingYear => {}
            AppState::SelectingDay => {}
            AppState::SelectingPart => {}
            AppState::SelectingFile => {}
            AppState::Solving => {}
        }
    }

    pub fn confirm(&mut self) {
        match self.state {
            AppState::SelectingYear => {}
            AppState::SelectingDay => {}
            AppState::SelectingPart => {}
            AppState::SelectingFile => {}
            AppState::Solving => {}
        }
    }

    pub fn back(&mut self) {
        match self.state {
            AppState::SelectingYear => {}
            AppState::SelectingDay => {
                self.state = AppState::SelectingYear;
            }
            AppState::SelectingPart => {
                self.state = AppState::SelectingDay;
            }
            AppState::SelectingFile => {
                self.state = AppState::SelectingPart;
            }
            AppState::Solving => {
                self.state = AppState::SelectingFile;
            }
        }
    }
}
