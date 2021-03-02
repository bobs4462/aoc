use std::{
    fs::{read_dir, File},
    path::Path,
};

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::challenge::Challenge;

use super::{Movement, TermionFrame};

pub struct App {
    state: AppState,
    pub challenge: Challenge,
    year_state: ListState,
    day_state: ListState,
    part_state: ListState,
    file_state: ListState,
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
            state: AppState::SelectingYear,
            challenge: Challenge::default(),
            year_state,
            day_state: ListState::default(),
            part_state: ListState::default(),
            file_state: ListState::default(),
            path: std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
        }
    }
}

impl App {
    pub fn draw(&mut self, f: &mut TermionFrame) {
        let chunks = self.build_layout(f.size());
        Self::render_list(
            f,
            chunks[1],
            Challenge::years().iter(),
            &mut self.year_state,
        );
        if self.state > AppState::SelectingYear {
            Self::render_list(f, chunks[2], Challenge::days().iter(), &mut self.day_state);
        }
        if self.state > AppState::SelectingDay {
            Self::render_list(
                f,
                chunks[3],
                Challenge::parts().iter(),
                &mut self.part_state,
            );
        }
        if self.state > AppState::SelectingPart {
            let entries = read_dir(&self.path).unwrap();
            let entry_strings = entries
                .map(|e| e.map_or(String::from(""), |e| e.file_name().into_string().unwrap()));
            let iter = vec![String::from("..")];
            let entry_strings = iter.into_iter().chain(entry_strings);
            Self::render_list(f, chunks[0], entry_strings, &mut self.file_state)
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

    fn render_list<I, D>(f: &mut TermionFrame, area: Rect, iterable: I, state: &mut ListState)
    where
        I: Iterator<Item = D>,
        D: std::fmt::Display,
    {
        let items: Vec<ListItem> = iterable.map(|i| ListItem::new(i.to_string())).collect();
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("✓");
        f.render_stateful_widget(items, area, state);
    }

    pub fn select(&mut self, movement: Movement) {
        match self.state {
            AppState::SelectingYear => {
                let i = self.year_state.selected().expect("Year was not selected");
                let years = Challenge::years().len() as isize;
                let i = (i as isize + movement as isize + years) % years;
                // print!("SELECTING YEAR {}", i);
                self.year_state.select(Some(i as usize));
            }
            AppState::SelectingDay => {
                let i = self.day_state.selected().expect("Day was not selected");
                let days = Challenge::days().len() as isize;
                let i = (i as isize + movement as isize + days) % days;
                // print!("SELECTING day {}", i);
                self.day_state.select(Some(i as usize));
            }
            AppState::SelectingPart => {
                let i = self.part_state.selected().expect("Part was not selected");
                let parts = Challenge::parts().len() as isize;
                let i = (i as isize + movement as isize + parts) % parts;
                // print!("SELECTING part {}", i);
                self.part_state.select(Some(i as usize));
            }
            AppState::SelectingFile => {
                let i = self.file_state.selected().expect("File was not selected");
                let files = read_dir(&self.path)
                    .expect("Coudn't read directory")
                    .count() as isize
                    + 1;
                let i = (i as isize + movement as isize + files) % files;
                // print!("SELECTING file {}", i);
                self.file_state.select(Some(i as usize));
            }
            AppState::Solving => {}
        }
    }

    pub fn confirm(&mut self) {
        match self.state {
            AppState::SelectingYear => {
                self.state = AppState::SelectingDay;
                self.day_state.select(Some(0));
            }
            AppState::SelectingDay => {
                self.state = AppState::SelectingPart;
                self.part_state.select(Some(0));
            }
            AppState::SelectingPart => {
                self.state = AppState::SelectingFile;
                self.file_state.select(Some(0));
            }
            AppState::SelectingFile => {
                let i = self.file_state.selected().expect("File was not selected");

                if i == 0 {
                    self.path = Path::new(&self.path)
                        .parent()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();
                } else if read_dir(&self.path)
                    .unwrap()
                    .nth(i - 1)
                    .unwrap()
                    .unwrap()
                    .path()
                    .is_dir()
                {
                    self.path = read_dir(&self.path)
                        .unwrap()
                        .nth(i - 1)
                        .unwrap()
                        .unwrap()
                        .path()
                        .to_string_lossy()
                        .to_string();
                } else {
                    let year = Challenge::years()[self.year_state.selected().unwrap()];
                    let day = Challenge::days()[self.day_state.selected().unwrap()];
                    let part = Challenge::days()[self.part_state.selected().unwrap()];
                    let file = self.file_state.selected().unwrap();
                    let path = read_dir(&self.path)
                        .unwrap()
                        .nth(file - 1)
                        .unwrap()
                        .unwrap()
                        .path()
                        .to_string_lossy()
                        .to_string();
                    self.challenge.year = Some(year);
                    self.challenge.day = Some(day);
                    self.challenge.part = Some(part);
                    self.challenge.file = Some(File::open(path).expect("FILE COULD NOT BE OPENED"));
                    self.state = AppState::Solving;
                }
            }
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

    pub fn all_set(&self) -> bool {
        if let Some(_) = self.challenge.file {
            return true;
        }
        false
    }
}
