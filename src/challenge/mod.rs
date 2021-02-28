use std::{
    fmt::{Display, Formatter},
    fs::File,
};

#[derive(Copy, Clone)]
pub enum Year {
    Y2015 = 2015,
    Y2016,
}

pub struct Challenge {
    year: Option<Year>,
    day: Option<usize>,
    part: Option<usize>,
    file: Option<File>,
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str_repr = match self {
            Year::Y2015 => "Year 2015",
            Year::Y2016 => "Year 2016",
        };
        write!(f, "{}", str_repr)
    }
}

impl Challenge {
    pub fn years() -> Vec<Year> {
        vec![Year::Y2015, Year::Y2016]
    }
    pub fn days() -> Vec<usize> {
        (1..26).collect()
    }
    pub fn parts() -> Vec<usize> {
        (1..3).collect()
    }
    pub fn reset(&mut self) {
        self.year = None;
        self.day = None;
        self.part = None;
        self.file = None;
    }
}

impl Default for Challenge {
    fn default() -> Self {
        Challenge {
            year: None,
            day: None,
            part: None,
            file: None,
        }
    }
}

impl Display for Challenge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Year: {}. Day: {}. Part: {}.",
            self.year.unwrap(),
            self.day.unwrap(),
            self.part.unwrap()
        )
    }
}
