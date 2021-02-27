use tui::layout::{Constraint, Direction, Layout, Rect};

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
lazy_static! {
    static ref FIRST_LAYOUT: Vec<Layout> = {
        vec![
            Layout::default().constraints(
                [
                    Constraint::Max(30),
                    Constraint::Min(15),
                    Constraint::Max(30),
                ]
                .as_ref(),
            ),
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Max(30),
                        Constraint::Min(15),
                        Constraint::Max(30),
                    ]
                    .as_ref(),
                ),
        ]
    };
}
// static STATE_ONE_CONSTRAINTS: &[Constraint] = &[Constraint::Min(10)];
// static STATE_TWO_CONSTRAINTS: &[Constraint] =
// &[Constraint::Percentage(50), Constraint::Percentage(50)];
// static STATE_THREE_CONSTRAINTS: &[Constraint] = &[
// Constraint::Ratio(1, 3),
// Constraint::Ratio(1, 3),
// Constraint::Ratio(1, 3),
// ];
// static STATE_FOUR_CONSTRAINTS: &[Constraint] = &[Constraint::Min(10)];
// static STATE_FIVE_CONSTRAINTS: &[Constraint] = &[Constraint::Min(10)];

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

    fn build_layout(&self, area: Rect) -> Vec<Rect> {
        let mut chunks: Vec<Rect> = Vec::new();
        chunks
    }

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
