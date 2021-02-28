use std::{fs::read_dir, path::Path};

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::challenge::Challenge;

use super::{Movement, TermionFrame};

pub struct App {
    state: AppState,
    challenge: Challenge,
    year_state: ListState,
    day_state: ListState,
    part_state: ListState,
    path: String,
}

#[derive(PartialEq, PartialOrd)]
enum AppState {
    SelectingYear,
    SelectingDay,
    SelectingPart,
    SelectingFile,
    Solving,
}

impl Default for App {
    fn default() -> Self {
        let mut year_state = ListState::default();
        year_state.select(Some(0));
        App {
            state: AppState::SelectingFile,
            challenge: Challenge::default(),
            year_state,
            day_state: ListState::default(),
            part_state: ListState::default(),
            path: String::from("."),
        }
    }
}

impl App {
    pub fn draw(&mut self, f: &mut TermionFrame) {
        let chunks = self.build_layout(f.size());
        self.render_list(f, chunks[1], Challenge::years().iter());
        if self.state > AppState::SelectingYear {
            self.render_list(f, chunks[2], Challenge::days().iter());
        }
        if self.state > AppState::SelectingDay {
            self.render_list(f, chunks[3], Challenge::parts().iter());
        }
        if self.state > AppState::SelectingPart {
            let entries = read_dir(&self.path).unwrap();
            let entry_strings = entries
                .map(|e| e.map_or(String::from(""), |e| e.file_name().into_string().unwrap()));
            self.render_list(f, chunks[0], entry_strings)
        }
    }

    fn build_layout(&self, area: Rect) -> Vec<Rect> {
        let mut chunks: Vec<Rect> = Vec::new();
        let layout = Layout::default()
            .constraints(
                [
                    Constraint::Length(10),
                    Constraint::Min(15),
                    Constraint::Length(10),
                ]
                .as_ref(),
            )
            .split(area);
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(50),
                    Constraint::Min(15),
                    Constraint::Length(50),
                ]
                .as_ref(),
            )
            .split(layout[1]);
        let layout = Layout::default()
            .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)].as_ref())
            .split(layout[1]);
        chunks.push(layout[1]);
        let mut layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ]
                .as_ref(),
            )
            .split(layout[0]);
        chunks.append(&mut layout);
        chunks
    }

    fn render_list<I, D>(&mut self, f: &mut TermionFrame, area: Rect, iterable: I)
    where
        I: Iterator<Item = D>,
        D: std::fmt::Display,
    {
        let items: Vec<ListItem> = iterable.map(|i| ListItem::new(i.to_string())).collect();
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("✓");
        f.render_stateful_widget(items, area, &mut self.year_state)
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
