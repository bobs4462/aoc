pub enum Movement {
    Up = -1,
    Down = 1,
}

use std::{
    fmt::{Display, Formatter},
    fs::{read_dir, File},
    path::PathBuf,
};

use crate::{
    solver::{Part, SolveError},
    statics::YEARS,
};

pub struct App {
    pub(crate) state: AppState,
    pub(crate) year: isize,
    pub(crate) day: isize,
    pub(crate) part: isize,
    pub(crate) dir_entry: isize,
    pub(crate) path: PathBuf,
}

#[derive(PartialEq, PartialOrd)]
pub enum AppState {
    SelectingYear,
    SelectingDay,
    SelectingFile,
    SelectingPart,
    Solving,
}

impl Default for App {
    fn default() -> Self {
        App {
            state: AppState::SelectingYear,
            year: 0,
            day: 0,
            part: 0,
            dir_entry: 0,
            path: std::env::current_dir().expect("Unable to read current directory"),
        }
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{year}. {day}. File: {path:?}. Part: {part}",
            year = YEARS[self.year as usize],
            day = YEARS[self.year as usize].day(self.day as usize),
            path = self.path,
            part = self.part
        )
    }
}

impl App {
    pub fn vmove(&mut self, movement: Movement) {
        let mv = movement as isize;
        match self.state {
            AppState::SelectingYear => {
                let years = YEARS.len() as isize;
                self.year = (self.year + mv + years) % years;
            }
            AppState::SelectingDay => {
                let days = YEARS[self.year as usize].daysc();
                self.day = (self.day + mv + days) % days;
            }
            AppState::SelectingFile => {
                let mut entries = read_dir(&self.path)
                    .expect("Coudn't read directory")
                    .count() as isize;
                if let Some(s) = self.path.file_name() {
                    if s != "/" {
                        entries += 1;
                    }
                }
                self.dir_entry = (self.dir_entry + mv + entries) % entries;
            }
            AppState::SelectingPart => {
                self.part = (self.part + 1) % 2;
            }
            AppState::Solving => {}
        }
    }

    pub fn confirm(&mut self) {
        match self.state {
            AppState::SelectingYear => {
                self.state = AppState::SelectingDay;
            }
            AppState::SelectingDay => {
                self.state = AppState::SelectingFile;
            }
            AppState::SelectingPart => {
                self.state = AppState::Solving;
            }
            AppState::SelectingFile => {
                if self.dir_entry == 0 {
                    self.path = self
                        .path
                        .parent()
                        .expect("Couldn't read parent directory")
                        .to_path_buf();
                    return;
                } else if read_dir(&self.path)
                    .unwrap()
                    .nth(self.dir_entry as usize - 1)
                    .unwrap()
                    .unwrap()
                    .path()
                    .is_file()
                {
                    self.state = AppState::SelectingPart;
                }
                self.path = read_dir(&self.path)
                    .unwrap()
                    .nth(self.dir_entry as usize - 1)
                    .unwrap()
                    .unwrap()
                    .path();
                self.dir_entry = 0;
            }
            AppState::Solving => {}
        }
    }

    pub fn solve(&self, part: Part) -> Result<String, SolveError> {
        let solver = YEARS[self.year as usize].day(self.day as usize).solver();
        let file = File::open(&self.path).expect("Coudn't read the supplied file");
        solver.solve(file, part)
    }

    pub fn dir_entries(&self) -> Vec<String> {
        let mut entries = vec![String::from("..")];
        entries.append(
            &mut read_dir(&self.path)
                .unwrap()
                .map(|de| de.unwrap().file_name().to_str().unwrap().to_string())
                .collect(),
        );
        entries
    }
}
